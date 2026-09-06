use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{DbInstances, DbPool, Migration, MigrationKind};
use sqlx::{Pool, Sqlite};

const SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT
);

CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT,
    description TEXT,
    image TEXT,
    category_id INTEGER,
    base_presentation_id INTEGER,
    stock REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS presentations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    factor REAL NOT NULL,
    UNIQUE(product_id, name)
);

CREATE TABLE IF NOT EXISTS suppliers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    contact TEXT,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS responsible_persons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS movements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id INTEGER NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('in','out')),
    quantity REAL NOT NULL,
    presentation_id INTEGER NOT NULL,
    presentation_name TEXT NOT NULL DEFAULT '',
    base_quantity REAL NOT NULL,
    supplier_id INTEGER,
    responsible_person_id INTEGER,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_presentations_product ON presentations(product_id);
CREATE INDEX IF NOT EXISTS idx_movements_product ON movements(product_id);
"#;

const SEED_SQL: &str = r#"
INSERT OR IGNORE INTO categories (name, color) VALUES
    ('Bebidas', '#3b82f6'),
    ('Alimentos', '#22c55e'),
    ('Limpieza', '#8b5cf6'),
    ('Higiene', '#14b8a6'),
    ('Otros', '#64748b');

INSERT OR IGNORE INTO settings (key, value) VALUES
    ('stock_low_threshold', '5'),
    ('stock_medium_threshold', '20');
"#;

#[derive(serde::Deserialize)]
struct MovementInput {
    product_id: i64,
    #[serde(rename = "type")]
    movement_type: String,
    quantity: f64,
    presentation_id: i64,
    supplier_id: Option<i64>,
    responsible_person_id: Option<i64>,
    notes: Option<String>,
}

async fn get_pool(app: &AppHandle) -> Result<Pool<Sqlite>, String> {
    let instances = app.state::<DbInstances>();
    let map = instances.0.read().await;
    let pool = map
        .get("sqlite:app.db")
        .ok_or_else(|| "Database not loaded".to_string())?;
    match pool {
        DbPool::Sqlite(p) => Ok(p.clone()),
    }
}

async fn presentation_info(app: &AppHandle, id: i64) -> Result<(f64, String), String> {
    let pool = get_pool(app).await?;
    let row: (f64, String) =
        sqlx::query_as("SELECT factor, name FROM presentations WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .map_err(|e| e.to_string())?;
    Ok(row)
}

#[tauri::command]
async fn create_movement(app: AppHandle, input: MovementInput) -> Result<(), String> {
    if input.movement_type != "in" && input.movement_type != "out" {
        return Err("Invalid movement type".into());
    }

    let (factor, presentation_name) = presentation_info(&app, input.presentation_id).await?;
    let base_quantity = input.quantity * factor;
    let delta = if input.movement_type == "in" {
        base_quantity
    } else {
        -base_quantity
    };

    let pool = get_pool(&app).await?;
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO movements
         (product_id, type, quantity, presentation_id, presentation_name, base_quantity, supplier_id, responsible_person_id, notes)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(input.product_id)
    .bind(&input.movement_type)
    .bind(input.quantity)
    .bind(input.presentation_id)
    .bind(&presentation_name)
    .bind(base_quantity)
    .bind(input.supplier_id)
    .bind(input.responsible_person_id)
    .bind(input.notes)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query("UPDATE products SET stock = stock + ?, updated_at = datetime('now') WHERE id = ?")
        .bind(delta)
        .bind(input.product_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_movement(app: AppHandle, id: i64) -> Result<(), String> {
    let pool = get_pool(&app).await?;
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let row: (i64, String, f64) =
        sqlx::query_as("SELECT product_id, type, base_quantity FROM movements WHERE id = ?")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

    let (product_id, movement_type, base_quantity) = row;
    let delta = if movement_type == "in" {
        -base_quantity
    } else {
        base_quantity
    };

    sqlx::query("DELETE FROM movements WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("UPDATE products SET stock = stock + ?, updated_at = datetime('now') WHERE id = ?")
        .bind(delta)
        .bind(product_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_product(app: AppHandle, id: i64) -> Result<(), String> {
    let pool = get_pool(&app).await?;
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM movements WHERE product_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM presentations WHERE product_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_product_image(app: AppHandle, source_path: String) -> Result<String, String> {
    let source = std::path::PathBuf::from(&source_path);
    if !source.exists() {
        return Err("Source file does not exist".into());
    }

    let ext = source
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png")
        .to_string();
    let filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);

    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let images_dir = data_dir.join("images");
    std::fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;

    let dest = images_dir.join(&filename);
    std::fs::copy(&source, &dest).map_err(|e| e.to_string())?;

    Ok(format!("images/{}", filename))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(
                    "sqlite:app.db",
                    vec![
                        Migration {
                            version: 1,
                            description: "create_schema",
                            sql: SCHEMA_SQL,
                            kind: MigrationKind::Up,
                        },
                        Migration {
                            version: 2,
                            description: "seed_defaults",
                            sql: SEED_SQL,
                            kind: MigrationKind::Up,
                        },
                        Migration {
                            version: 4,
                            description: "drop_product_code",
                            sql: "ALTER TABLE products DROP COLUMN code;",
                            kind: MigrationKind::Up,
                        },
                    ],
                )
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            create_movement,
            delete_movement,
            delete_product,
            save_product_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
