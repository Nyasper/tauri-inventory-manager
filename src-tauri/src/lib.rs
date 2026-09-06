use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{DbInstances, DbPool};
use sqlx::{Pool, Sqlite};

const SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    icon TEXT
);

CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
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
INSERT OR IGNORE INTO categories (name, color, icon) VALUES
    ('Bebidas', '#3b82f6', 'cup-soda'),
    ('Alimentos', '#22c55e', 'apple'),
    ('Limpieza', '#8b5cf6', 'spray-can'),
    ('Higiene', '#14b8a6', 'droplets'),
    ('Otros', '#64748b', 'boxes');

INSERT OR IGNORE INTO settings (key, value) VALUES
    ('stock_low_threshold', '5'),
    ('stock_medium_threshold', '20');
"#;

const SEED_DATA_SQL: &str = r#"
INSERT OR IGNORE INTO suppliers (name, contact, notes) VALUES
    ('Distribuidora Norte', 'contacto@norte.com', 'Entregas los lunes'),
    ('Proveedor Sur', '11-2222-3333', 'Pagos a 30 días'),
    ('Importadora Central', NULL, 'Pedidos mínimos grandes');

INSERT OR IGNORE INTO responsible_persons (name, is_default) VALUES
    ('Ana Pérez', 1),
    ('Carlos Gómez', 0);

INSERT OR IGNORE INTO products (name, description, category_id, stock) VALUES
    ('Agua Mineral 500ml', 'Agua purificada en botella', 1, 120),
    ('Café Molido 250g', 'Café de origen', 2, 45),
    ('Detergente 1L', 'Detergente líquido', 3, 30),
    ('Jabón de Manos', 'Jabón antibacterial', 4, 3),
    ('Galletas Variadas', 'Paquete surtido', 2, 8),
    ('Refresco Cola 2L', 'Refresco de cola', 1, 60);

INSERT OR IGNORE INTO presentations (product_id, name, factor) VALUES
    (1, 'Botella', 1),
    (1, 'Six-pack', 6),
    (1, 'Caja 24u', 24),
    (2, 'Paquete', 1),
    (2, 'Display 12u', 12),
    (3, 'Botella', 1),
    (3, 'Caja 12u', 12),
    (4, 'Unidad', 1),
    (4, 'Pack x6', 6),
    (5, 'Paquete', 1),
    (6, 'Botella', 1),
    (6, 'Caja 6u', 6);

UPDATE products SET base_presentation_id = (
    SELECT id FROM presentations WHERE product_id = products.id AND name IN ('Botella','Paquete','Unidad') LIMIT 1
);

INSERT OR IGNORE INTO movements (product_id, type, quantity, presentation_id, presentation_name, base_quantity, supplier_id, responsible_person_id, notes) VALUES
    (1, 'in', 100, (SELECT id FROM presentations WHERE product_id=1 AND name='Botella'), 'Botella', 100, 1, 1, 'Compra inicial'),
    (1, 'in', 2, (SELECT id FROM presentations WHERE product_id=1 AND name='Caja 24u'), 'Caja 24u', 48, 1, 1, 'Reposición'),
    (1, 'out', 30, (SELECT id FROM presentations WHERE product_id=1 AND name='Botella'), 'Botella', 30, NULL, 2, 'Venta mostrador'),
    (2, 'in', 60, (SELECT id FROM presentations WHERE product_id=2 AND name='Paquete'), 'Paquete', 60, 2, 1, NULL),
    (2, 'out', 15, (SELECT id FROM presentations WHERE product_id=2 AND name='Paquete'), 'Paquete', 15, NULL, 2, 'Uso interno'),
    (3, 'in', 40, (SELECT id FROM presentations WHERE product_id=3 AND name='Botella'), 'Botella', 40, 3, 1, NULL),
    (3, 'out', 10, (SELECT id FROM presentations WHERE product_id=3 AND name='Botella'), 'Botella', 10, NULL, 2, 'Venta'),
    (4, 'in', 10, (SELECT id FROM presentations WHERE product_id=4 AND name='Unidad'), 'Unidad', 10, 1, 1, NULL),
    (4, 'out', 7, (SELECT id FROM presentations WHERE product_id=4 AND name='Unidad'), 'Unidad', 7, NULL, 2, 'Consumo'),
    (5, 'in', 20, (SELECT id FROM presentations WHERE product_id=5 AND name='Paquete'), 'Paquete', 20, 2, 1, NULL),
    (5, 'out', 12, (SELECT id FROM presentations WHERE product_id=5 AND name='Paquete'), 'Paquete', 12, NULL, 2, 'Venta'),
    (6, 'in', 70, (SELECT id FROM presentations WHERE product_id=6 AND name='Botella'), 'Botella', 70, 3, 1, NULL),
    (6, 'out', 10, (SELECT id FROM presentations WHERE product_id=6 AND name='Botella'), 'Botella', 10, NULL, 2, 'Venta');

INSERT OR IGNORE INTO suppliers (name, contact, notes) VALUES
    ('Limpiezas del Valle', 'ventas@valle.com', 'Especialistas en químicos'),
    ('Química Industrial SA', NULL, 'Productos a granel'),
    ('Mayorista Express', '555-1234', 'Entrega en 24h');

INSERT OR IGNORE INTO responsible_persons (name, is_default) VALUES
    ('Lucía Martín', 0);

INSERT OR IGNORE INTO products (name, description, category_id, stock) VALUES
    ('Cloro 1L', 'Cloro concentrado para desinfección', 3, 30),
    ('Suavizante 750ml', 'Suavizante de ropa', 3, 24),
    ('Esponja Multiusos', 'Esponja abrasiva', 3, 36),
    ('Limpiador Multi 500ml', 'Limpiador multipropósito', 3, 15),
    ('Desinfectante 500ml', 'Desinfectante de superficies', 3, 11),
    ('Shampoo 400ml', 'Shampoo hidratante', 4, 12);

INSERT OR IGNORE INTO presentations (product_id, name, factor) VALUES
    (7, 'Botella', 1),
    (7, 'Caja 12u', 12),
    (8, 'Botella', 1),
    (8, 'Caja 6u', 6),
    (9, 'Unidad', 1),
    (9, 'Pack x10', 10),
    (10, 'Botella', 1),
    (10, 'Caja 12u', 12),
    (11, 'Botella', 1),
    (11, 'Caja 6u', 6),
    (12, 'Unidad', 1),
    (12, 'Pack x4', 4);

INSERT OR IGNORE INTO movements (product_id, type, quantity, presentation_id, presentation_name, base_quantity, supplier_id, responsible_person_id, notes) VALUES
    (7, 'in', 50, (SELECT id FROM presentations WHERE product_id=7 AND name='Botella'), 'Botella', 50, 4, 1, 'Compra mensual'),
    (7, 'out', 20, (SELECT id FROM presentations WHERE product_id=7 AND name='Botella'), 'Botella', 20, NULL, 3, 'Uso área común'),
    (8, 'in', 36, (SELECT id FROM presentations WHERE product_id=8 AND name='Botella'), 'Botella', 36, 5, 1, NULL),
    (8, 'out', 12, (SELECT id FROM presentations WHERE product_id=8 AND name='Botella'), 'Botella', 12, NULL, 2, 'Venta'),
    (9, 'in', 100, (SELECT id FROM presentations WHERE product_id=9 AND name='Unidad'), 'Unidad', 100, 6, 1, NULL),
    (9, 'out', 64, (SELECT id FROM presentations WHERE product_id=9 AND name='Unidad'), 'Unidad', 64, NULL, 2, 'Consumo'),
    (10, 'in', 40, (SELECT id FROM presentations WHERE product_id=10 AND name='Botella'), 'Botella', 40, 4, 1, NULL),
    (10, 'out', 25, (SELECT id FROM presentations WHERE product_id=10 AND name='Botella'), 'Botella', 25, NULL, 3, 'Venta'),
    (11, 'in', 30, (SELECT id FROM presentations WHERE product_id=11 AND name='Botella'), 'Botella', 30, 5, 1, NULL),
    (11, 'out', 19, (SELECT id FROM presentations WHERE product_id=11 AND name='Botella'), 'Botella', 19, NULL, 2, 'Uso limpieza'),
    (12, 'in', 30, (SELECT id FROM presentations WHERE product_id=12 AND name='Unidad'), 'Unidad', 30, 6, 1, NULL),
    (12, 'out', 18, (SELECT id FROM presentations WHERE product_id=12 AND name='Unidad'), 'Unidad', 18, NULL, 3, 'Venta');

UPDATE products SET base_presentation_id = (
    SELECT id FROM presentations WHERE product_id = products.id AND name IN ('Botella','Paquete','Unidad') LIMIT 1
) WHERE base_presentation_id IS NULL;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            init_db(app.handle()).map_err(|e| {
                eprintln!("failed to initialize database: {e}");
                e.into()
            })
        })
        .invoke_handler(tauri::generate_handler![
            create_movement,
            delete_movement,
            delete_product
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_db(app: &tauri::AppHandle) -> Result<(), String> {
    tauri::async_runtime::block_on(async {
        let pool = get_pool(app).await?;
        sqlx::raw_sql(SCHEMA_SQL)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
        migrate(&pool).await?;
        for sql in [SEED_SQL, SEED_DATA_SQL] {
            sqlx::raw_sql(sql).execute(&pool).await.map_err(|e| e.to_string())?;
        }
        Ok(())
    })
}

async fn migrate(pool: &Pool<Sqlite>) -> Result<(), String> {
    let has_icon = column_exists(pool, "categories", "icon").await?;
    if !has_icon {
        sqlx::raw_sql("ALTER TABLE categories ADD COLUMN icon TEXT")
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    for (name, icon) in [
        ("Bebidas", "cup-soda"),
        ("Alimentos", "apple"),
        ("Limpieza", "spray-can"),
        ("Higiene", "droplets"),
        ("Otros", "boxes"),
    ] {
        sqlx::query("UPDATE categories SET icon = ? WHERE name = ? AND icon IS NULL")
            .bind(icon)
            .bind(name)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    let has_image = column_exists(pool, "products", "image").await?;
    if has_image {
        let _ = sqlx::raw_sql("ALTER TABLE products DROP COLUMN image")
            .execute(pool)
            .await;
    }

    Ok(())
}

async fn column_exists(
    pool: &Pool<Sqlite>,
    table: &str,
    column: &str,
) -> Result<bool, String> {
    let rows: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM pragma_table_info(?)")
            .bind(table)
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;
    Ok(rows.iter().any(|(name,)| name == column))
}
