use serde::Deserialize;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Row, Sqlite,
};
use std::{path::Path, time::Duration};
use tauri::State;
use unicode_normalization::UnicodeNormalization;

type Db = Pool<Sqlite>;
const SCHEMA: &str = include_str!("schema.sql");
pub async fn connect(path: &Path) -> Result<Db, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(
            SqliteConnectOptions::new()
                .filename(path)
                .create_if_missing(true)
                .foreign_keys(true)
                .busy_timeout(Duration::from_secs(10)),
        )
        .await?;
    sqlx::raw_sql(SCHEMA).execute(&pool).await?;
    Ok(pool)
}

/// Loads the same SQL used by `bun run seed` so its integrity is covered by Rust tests.
#[cfg(test)]
async fn seed_demo_data(db: &Db) -> Result<(), sqlx::Error> {
    let mut tx = db.begin().await?;
    let product_count: i64 = sqlx::query_scalar("SELECT count(*) FROM products")
        .fetch_one(&mut *tx)
        .await?;
    if product_count > 0 {
        tx.rollback().await?;
        return Ok(());
    }
    sqlx::raw_sql(include_str!("seed.sql"))
        .execute(&mut *tx)
        .await?;
    tx.commit().await
}
fn error(e: sqlx::Error) -> String {
    if let Some(db) = e.as_database_error() {
        if db.is_unique_violation() {
            let entity = if db.message().contains("products.") {
                "un producto"
            } else if db.message().contains("suppliers.") {
                "un proveedor"
            } else if db.message().contains("categories.") {
                "una categoría"
            } else if db.message().contains("presentations.") {
                "una presentación de este producto"
            } else {
                "un responsable"
            };
            return format!("Ya existe {entity} con ese nombre (puede estar archivado).");
        }
        if db.is_foreign_key_violation() {
            return "El registro asociado no existe o está siendo utilizado.".into();
        }
    }
    eprintln!("Database error: {e}");
    "No se pudo completar la operación. Revisa los datos e inténtalo nuevamente.".into()
}
fn name(value: &str) -> Result<(String, String), String> {
    let display = value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .nfc()
        .collect::<String>();
    if display.is_empty() || display.chars().count() > 200 {
        return Err("El nombre debe tener entre 1 y 200 caracteres.".into());
    }
    let key = display.to_lowercase().nfc().collect();
    Ok((display, key))
}
fn positive(value: f64) -> Result<(), String> {
    if !value.is_finite() || value <= 0.0 || value > 1e9 {
        return Err(
            "La cantidad y el factor deben ser positivos y como máximo 1.000 millones.".into(),
        );
    }
    if value < 0.000001 || (value - (value * 1e6).round() / 1e6).abs() > 1e-10 {
        return Err("Se admiten hasta 6 decimales en cantidades, factores y conversiones.".into());
    }
    Ok(())
}
#[derive(Deserialize)]
pub struct PresentationInput {
    id: Option<i64>,
    name: String,
    factor: f64,
}
#[derive(Deserialize)]
pub struct ProductInput {
    id: Option<i64>,
    name: String,
    description: Option<String>,
    category_id: Option<i64>,
    presentations: Vec<PresentationInput>,
}
#[tauri::command]
pub async fn save_product(db: State<'_, Db>, input: ProductInput) -> Result<i64, String> {
    save_product_db(&db, input).await
}
async fn save_product_db(db: &Db, input: ProductInput) -> Result<i64, String> {
    let (display, key) = name(&input.name)?;
    if input.presentations.is_empty()
        || input
            .presentations
            .iter()
            .filter(|p| p.factor == 1.0)
            .count()
            != 1
    {
        return Err("Selecciona exactamente una presentación base con factor 1.".into());
    }
    let mut keys = std::collections::HashSet::new();
    let mut ids = std::collections::HashSet::new();
    for p in &input.presentations {
        positive(p.factor)?;
        if !keys.insert(name(&p.name)?.1) {
            return Err("Hay presentaciones con el mismo nombre.".into());
        }
        if let Some(id) = p.id {
            if !ids.insert(id) {
                return Err("Presentación repetida.".into());
            }
        }
    }
    let mut tx = db.begin().await.map_err(error)?;
    // Take the write lock before reading mutable inventory data.
    sqlx::query("UPDATE settings SET value=value WHERE key='default_view'")
        .execute(&mut *tx)
        .await
        .map_err(error)?;
    let id = if let Some(id) = input.id {
        let row=sqlx::query("SELECT base_presentation_id, EXISTS(SELECT 1 FROM movements WHERE product_id=products.id) AS used FROM products WHERE id=? AND archived=0")
            .bind(id).fetch_optional(&mut *tx).await.map_err(error)?.ok_or("El producto no está disponible.")?;
        if row.get::<bool, _>("used") {
            let base = input
                .presentations
                .iter()
                .find(|p| p.factor == 1.0)
                .unwrap();
            if base.id != Some(row.get("base_presentation_id")) {
                return Err(
                    "No se puede cambiar la unidad base de un producto con historial.".into(),
                );
            }
        }
        sqlx::query("UPDATE products SET name=?,name_key=?,description=?,category_id=?,updated_at=datetime('now') WHERE id=?")
            .bind(&display).bind(&key).bind(&input.description).bind(input.category_id).bind(id).execute(&mut *tx).await.map_err(error)?;
        id
    } else {
        if !ids.is_empty() {
            return Err("Las presentaciones nuevas no deben tener identificador.".into());
        }
        sqlx::query("INSERT INTO products(name,name_key,description,category_id,base_presentation_id) VALUES(?,?,?,?, -1)")
            .bind(&display).bind(&key).bind(&input.description).bind(input.category_id).execute(&mut *tx).await.map_err(error)?.last_insert_rowid()
    };
    // Retire omitted presentations; historical references remain valid.
    sqlx::query("UPDATE presentations SET archived=1 WHERE product_id=?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(error)?;
    let mut base = 0;
    for mut p in input.presentations {
        let (display, key) = name(&p.name)?;
        if p.id.is_none() {
            p.id = sqlx::query_scalar(
                "SELECT id FROM presentations WHERE product_id=? AND name_key=? AND archived=1",
            )
            .bind(id)
            .bind(&key)
            .fetch_optional(&mut *tx)
            .await
            .map_err(error)?;
        }
        let pres = if let Some(pid) = p.id {
            let old=sqlx::query("SELECT factor, EXISTS(SELECT 1 FROM movements WHERE presentation_id=presentations.id) AS used FROM presentations WHERE id=? AND product_id=?")
                .bind(pid).bind(id).fetch_optional(&mut *tx).await.map_err(error)?.ok_or("La presentación no pertenece a este producto.")?;
            if old.get::<bool, _>("used") && old.get::<f64, _>("factor") != p.factor {
                return Err("Una presentación con historial no puede cambiar de factor. Crea otra presentación.".into());
            }
            sqlx::query(
                "UPDATE presentations SET name=?,name_key=?,factor=?,archived=0 WHERE id=?",
            )
            .bind(display)
            .bind(key)
            .bind(p.factor)
            .bind(pid)
            .execute(&mut *tx)
            .await
            .map_err(error)?;
            pid
        } else {
            sqlx::query(
                "INSERT INTO presentations(product_id,name,name_key,factor) VALUES(?,?,?,?)",
            )
            .bind(id)
            .bind(display)
            .bind(key)
            .bind(p.factor)
            .execute(&mut *tx)
            .await
            .map_err(error)?
            .last_insert_rowid()
        };
        if p.factor == 1.0 {
            base = pres;
        }
    }
    sqlx::query("UPDATE products SET base_presentation_id=? WHERE id=?")
        .bind(base)
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(error)?;
    tx.commit().await.map_err(error)?;
    Ok(id)
}
#[derive(Deserialize)]
pub struct CatalogInput {
    kind: String,
    id: Option<i64>,
    name: String,
    contact: Option<String>,
    notes: Option<String>,
    color: Option<String>,
    icon: Option<String>,
    #[serde(default)]
    is_default: bool,
}
#[tauri::command]
pub async fn save_catalog(db: State<'_, Db>, input: CatalogInput) -> Result<i64, String> {
    save_catalog_db(&db, input).await
}
async fn save_catalog_db(db: &Db, input: CatalogInput) -> Result<i64, String> {
    let (display, key) = name(&input.name)?;
    let mut tx = db.begin().await.map_err(error)?;
    let result=match input.kind.as_str() {
        "supplier" => sqlx::query("INSERT INTO suppliers(id,name,name_key,contact,notes) VALUES(?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET name=excluded.name,name_key=excluded.name_key,contact=excluded.contact,notes=excluded.notes")
            .bind(input.id).bind(display).bind(key).bind(input.contact).bind(input.notes).execute(&mut *tx).await,
        "category" => sqlx::query("INSERT INTO categories(id,name,name_key,color,icon) VALUES(?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET name=excluded.name,name_key=excluded.name_key,color=excluded.color,icon=excluded.icon")
            .bind(input.id).bind(display).bind(key).bind(input.color).bind(input.icon).execute(&mut *tx).await,
        "person" => {
            if input.is_default { sqlx::query("UPDATE responsible_persons SET is_default=0 WHERE is_default=1").execute(&mut *tx).await.map_err(error)?; }
            sqlx::query("INSERT INTO responsible_persons(id,name,name_key,is_default) VALUES(?,?,?,?) ON CONFLICT(id) DO UPDATE SET name=excluded.name,name_key=excluded.name_key,is_default=excluded.is_default")
                .bind(input.id).bind(display).bind(key).bind(input.is_default).execute(&mut *tx).await
        },
        _ => return Err("Tipo de registro inválido.".into())
    }.map_err(error)?;
    tx.commit().await.map_err(error)?;
    Ok(input.id.unwrap_or(result.last_insert_rowid()))
}
#[tauri::command]
pub async fn archive_record(
    db: State<'_, Db>,
    kind: String,
    id: i64,
    restore: bool,
) -> Result<(), String> {
    archive_record_db(&db, kind, id, restore).await
}
async fn archive_record_db(db: &Db, kind: String, id: i64, restore: bool) -> Result<(), String> {
    let sql = match (kind.as_str(), restore) {
        ("product", false) => "UPDATE products SET archived=1 WHERE id=? AND stock=0",
        ("product", true) => "UPDATE products SET archived=0 WHERE id=?",
        ("supplier", false) => "UPDATE suppliers SET archived=1 WHERE id=?",
        ("supplier", true) => "UPDATE suppliers SET archived=0 WHERE id=?",
        ("person", false) => "UPDATE responsible_persons SET archived=1,is_default=0 WHERE id=?",
        ("person", true) => "UPDATE responsible_persons SET archived=0 WHERE id=?",
        ("category", false) => "DELETE FROM categories WHERE id=?",
        _ => return Err("Operación inválida.".into()),
    };
    if sqlx::query(sql)
        .bind(id)
        .execute(db)
        .await
        .map_err(error)?
        .rows_affected()
        != 1
    {
        return Err(
            "El registro no existe o tiene stock. Registra un ajuste antes de archivarlo.".into(),
        );
    }
    Ok(())
}
#[derive(Deserialize)]
pub struct MovementInput {
    product_id: i64,
    #[serde(rename = "type")]
    movement_type: String,
    quantity: f64,
    presentation_id: i64,
    supplier_id: Option<i64>,
    responsible_person_id: Option<i64>,
    notes: Option<String>,
    #[serde(default)]
    adjustment: bool,
    reason: Option<String>,
}
#[tauri::command]
pub async fn create_movement(db: State<'_, Db>, input: MovementInput) -> Result<(), String> {
    create_movement_db(&db, input).await
}
async fn create_movement_db(db: &Db, input: MovementInput) -> Result<(), String> {
    positive(input.quantity)?;
    if !["in", "out"].contains(&input.movement_type.as_str()) {
        return Err("Tipo de movimiento inválido.".into());
    }
    if input.movement_type == "out" && input.supplier_id.is_some() {
        return Err("Una salida no admite proveedor.".into());
    }
    if input.adjustment && input.reason.as_deref().unwrap_or("").trim().is_empty() {
        return Err("Indica el motivo del ajuste.".into());
    }
    let mut tx = db.begin().await.map_err(error)?;
    sqlx::query("UPDATE settings SET value=value WHERE key='default_view'")
        .execute(&mut *tx)
        .await
        .map_err(error)?;
    let row=sqlx::query("SELECT r.factor,r.name,p.name AS product_name FROM presentations r JOIN products p ON p.id=r.product_id WHERE r.id=? AND p.id=? AND r.archived=0 AND p.archived=0")
        .bind(input.presentation_id).bind(input.product_id).fetch_optional(&mut *tx).await.map_err(error)?.ok_or("El producto o la presentación no están disponibles.")?;
    let factor: f64 = row.get("factor");
    let base = input.quantity * factor;
    positive(base)?;
    let base = (base * 1e6).round() / 1e6;
    let mut names = Vec::new();
    for (id, sql) in [
        (
            input.supplier_id,
            "SELECT name FROM suppliers WHERE id=? AND archived=0",
        ),
        (
            input.responsible_person_id,
            "SELECT name FROM responsible_persons WHERE id=? AND archived=0",
        ),
    ] {
        names.push(if let Some(id) = id {
            Some(
                sqlx::query_scalar::<_, String>(sql)
                    .bind(id)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(error)?
                    .ok_or("El proveedor o responsable no está disponible.")?,
            )
        } else {
            None
        });
    }
    let delta = if input.movement_type == "in" {
        base
    } else {
        -base
    };
    if sqlx::query("UPDATE products SET stock=round(stock+?,6),updated_at=datetime('now') WHERE id=? AND round(stock+?,6)>=0 AND round(stock+?,6)<=1000000000")
        .bind(delta).bind(input.product_id).bind(delta).bind(delta).execute(&mut *tx).await.map_err(error)?.rows_affected()!=1 {
        return Err("Stock insuficiente o cantidad fuera del límite permitido.".into());
    }
    sqlx::query("INSERT INTO movements(product_id,type,quantity,presentation_id,presentation_name,product_name,factor,base_quantity,supplier_id,supplier_name,responsible_person_id,person_name,notes,kind,reason) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")
        .bind(input.product_id).bind(input.movement_type).bind(input.quantity).bind(input.presentation_id)
        .bind(row.get::<String,_>("name")).bind(row.get::<String,_>("product_name")).bind(factor).bind(base)
        .bind(input.supplier_id).bind(&names[0]).bind(input.responsible_person_id).bind(&names[1]).bind(input.notes)
        .bind(if input.adjustment {"adjustment"} else {"regular"}).bind(input.reason)
        .execute(&mut *tx).await.map_err(error)?;
    tx.commit().await.map_err(error)
}
#[tauri::command]
pub async fn cancel_movement(db: State<'_, Db>, id: i64, reason: String) -> Result<(), String> {
    cancel_movement_db(&db, id, reason).await
}
async fn cancel_movement_db(db: &Db, id: i64, reason: String) -> Result<(), String> {
    if reason.trim().is_empty() {
        return Err("Indica el motivo de la anulación.".into());
    }
    let mut tx = db.begin().await.map_err(error)?;
    if sqlx::query("UPDATE movements SET cancelled_at=datetime('now'),cancellation_reason=? WHERE id=? AND cancelled_at IS NULL")
        .bind(reason.trim()).bind(id).execute(&mut *tx).await.map_err(error)?.rows_affected()!=1 { return Err("El movimiento ya está anulado o no existe.".into()); }
    let row = sqlx::query("SELECT product_id,type,base_quantity FROM movements WHERE id=?")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(error)?;
    let base: f64 = row.get("base_quantity");
    let delta = if row.get::<String, _>("type") == "in" {
        -base
    } else {
        base
    };
    if sqlx::query("UPDATE products SET stock=round(stock+?,6),updated_at=datetime('now') WHERE id=? AND archived=0 AND round(stock+?,6)>=0 AND round(stock+?,6)<=1000000000")
        .bind(delta).bind(row.get::<i64,_>("product_id")).bind(delta).bind(delta).execute(&mut *tx).await.map_err(error)?.rows_affected()!=1 {
        return Err("No se puede anular: stock insuficiente o producto archivado.".into());
    }
    tx.commit().await.map_err(error)
}
#[tauri::command]
pub async fn save_settings(
    db: State<'_, Db>,
    low: Option<f64>,
    medium: Option<f64>,
    view: Option<String>,
) -> Result<(), String> {
    let mut tx = db.begin().await.map_err(error)?;
    if let (Some(low), Some(medium)) = (low, medium) {
        if !low.is_finite() || !medium.is_finite() || low < 0.0 || medium <= low || medium > 1e9 {
            return Err("Los umbrales deben cumplir: 0 ≤ bajo < medio ≤ 1.000 millones.".into());
        }
        for (key, value) in [
            ("stock_low_threshold", low),
            ("stock_medium_threshold", medium),
        ] {
            sqlx::query("UPDATE settings SET value=? WHERE key=?")
                .bind(value.to_string())
                .bind(key)
                .execute(&mut *tx)
                .await
                .map_err(error)?;
        }
    } else if low.is_some() || medium.is_some() {
        return Err("Indica ambos umbrales.".into());
    }
    if let Some(view) = view {
        if !["dashboard", "products", "movements", "suppliers"].contains(&view.as_str()) {
            return Err("Vista inválida.".into());
        }
        sqlx::query("UPDATE settings SET value=? WHERE key='default_view'")
            .bind(view)
            .execute(&mut *tx)
            .await
            .map_err(error)?;
    }
    tx.commit().await.map_err(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn product(id: Option<i64>, name: &str, pres: Option<i64>) -> ProductInput {
        ProductInput {
            id,
            name: name.into(),
            description: None,
            category_id: None,
            presentations: vec![PresentationInput {
                id: pres,
                name: "Unidad".into(),
                factor: 1.0,
            }],
        }
    }
    fn catalog(kind: &str, id: Option<i64>, name: &str, default: bool) -> CatalogInput {
        CatalogInput {
            kind: kind.into(),
            id,
            name: name.into(),
            contact: None,
            notes: None,
            color: None,
            icon: None,
            is_default: default,
        }
    }
    async fn fixture(db: &Db, name: &str) -> (i64, i64) {
        let id = save_product_db(db, product(None, name, None))
            .await
            .unwrap();
        let pres = sqlx::query_scalar("SELECT base_presentation_id FROM products WHERE id=?")
            .bind(id)
            .fetch_one(db)
            .await
            .unwrap();
        (id, pres)
    }
    fn movement(pair: (i64, i64), kind: &str, quantity: f64) -> MovementInput {
        MovementInput {
            product_id: pair.0,
            presentation_id: pair.1,
            movement_type: kind.into(),
            quantity,
            supplier_id: None,
            responsible_person_id: None,
            notes: None,
            adjustment: false,
            reason: None,
        }
    }
    async fn stock(db: &Db, id: i64) -> f64 {
        sqlx::query_scalar("SELECT stock FROM products WHERE id=?")
            .bind(id)
            .fetch_one(db)
            .await
            .unwrap()
    }
    #[test]
    fn unique_unicode_names_and_editing() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = fixture(&db, "  CAFÉ   Molido ").await;
            assert!(
                save_product_db(&db, product(None, "cafe\u{301} molido", None))
                    .await
                    .is_err()
            );
            save_product_db(&db, product(Some(a.0), "Café Molido", Some(a.1)))
                .await
                .unwrap();
            for kind in ["supplier", "category", "person"] {
                let id = save_catalog_db(&db, catalog(kind, None, "  ÁRBOL  Norte ", false))
                    .await
                    .unwrap();
                assert!(
                    save_catalog_db(&db, catalog(kind, None, "a\u{301}rbol norte", false))
                        .await
                        .is_err()
                );
                save_catalog_db(&db, catalog(kind, Some(id), "Árbol Norte", false))
                    .await
                    .unwrap();
            }
            assert_ne!(name("año").unwrap().1, name("ano").unwrap().1);
            assert!(save_product_db(&db, product(None, "  ", None))
                .await
                .is_err());
        });
    }
    #[test]
    fn failed_product_save_rolls_back_and_preserves_ids() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = fixture(&db, "Agua").await;
            let mut input = product(Some(a.0), "Nombre cambiado", Some(a.1));
            input.presentations.push(PresentationInput {
                id: Some(999),
                name: "Caja".into(),
                factor: 6.0,
            });
            assert!(save_product_db(&db, input).await.is_err());
            let row = sqlx::query("SELECT name,base_presentation_id FROM products WHERE id=?")
                .bind(a.0)
                .fetch_one(&db)
                .await
                .unwrap();
            assert_eq!(row.get::<String, _>("name"), "Agua");
            assert_eq!(row.get::<i64, _>("base_presentation_id"), a.1);
            let archived: i64 = sqlx::query_scalar("SELECT archived FROM presentations WHERE id=?")
                .bind(a.1)
                .fetch_one(&db)
                .await
                .unwrap();
            assert_eq!(archived, 0);
            let mut input = product(Some(a.0), "Agua", Some(a.1));
            input.presentations.push(PresentationInput {
                id: None,
                name: "Caja".into(),
                factor: 6.0,
            });
            save_product_db(&db, input).await.unwrap();
            save_product_db(&db, product(Some(a.0), "Agua", Some(a.1)))
                .await
                .unwrap();
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM presentations WHERE archived=1")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                1
            );
        });
    }
    #[test]
    fn invalid_movements_and_foreign_keys() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = fixture(&db, "Agua").await;
            let b = fixture(&db, "Café").await;
            for qty in [0.0, -1.0, f64::NAN, f64::INFINITY, 1e13] {
                assert!(create_movement_db(&db, movement(a, "in", qty))
                    .await
                    .is_err());
            }
            assert!(create_movement_db(&db, movement((a.0, b.1), "in", 1.0))
                .await
                .is_err());
            assert!(create_movement_db(&db, movement(a, "out", 1.0))
                .await
                .is_err());
            let mut input = movement(a, "in", 1.0);
            input.supplier_id = Some(999);
            assert!(create_movement_db(&db, input).await.is_err());
            assert!(
                sqlx::query("UPDATE products SET category_id=999 WHERE id=?")
                    .bind(a.0)
                    .execute(&db)
                    .await
                    .is_err()
            );
            assert!(sqlx::query("UPDATE products SET stock=-1 WHERE id=?")
                .bind(a.0)
                .execute(&db)
                .await
                .is_err());
            assert_eq!(stock(&db, a.0).await, 0.0);
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM movements")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                0
            );
        });
    }
    #[test]
    fn concurrent_withdrawals_and_cancellation_are_atomic() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = fixture(&db, "Agua").await;
            create_movement_db(&db, movement(a, "in", 10.0))
                .await
                .unwrap();
            let db1 = db.clone();
            let db2 = db.clone();
            let first = tauri::async_runtime::spawn(async move {
                create_movement_db(&db1, movement(a, "out", 7.0)).await
            });
            let second = tauri::async_runtime::spawn(async move {
                create_movement_db(&db2, movement(a, "out", 7.0)).await
            });
            assert_ne!(first.await.unwrap().is_ok(), second.await.unwrap().is_ok());
            assert_eq!(stock(&db, a.0).await, 3.0);
            assert!(cancel_movement_db(&db, 1, "Error".into()).await.is_err());
            assert_eq!(
                sqlx::query_scalar::<_, i64>(
                    "SELECT count(*) FROM movements WHERE cancelled_at IS NOT NULL"
                )
                .fetch_one(&db)
                .await
                .unwrap(),
                0
            );
            cancel_movement_db(&db, 2, "Salida duplicada".into())
                .await
                .unwrap();
            assert!(cancel_movement_db(&db, 2, "Otra vez".into()).await.is_err());
            assert_eq!(stock(&db, a.0).await, 10.0);
            cancel_movement_db(&db, 1, "Compra incorrecta".into())
                .await
                .unwrap();
            assert_eq!(stock(&db, a.0).await, 0.0);
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM movements")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                2
            );
        });
    }
    #[test]
    fn historical_snapshots_archiving_adjustments_and_base_are_preserved() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = fixture(&db, "Agua").await;
            let supplier = save_catalog_db(&db, catalog("supplier", None, "Proveedor", false))
                .await
                .unwrap();
            let mut input = movement(a, "in", 5.0);
            input.supplier_id = Some(supplier);
            create_movement_db(&db, input).await.unwrap();
            save_product_db(&db, product(Some(a.0), "Agua nueva", Some(a.1)))
                .await
                .unwrap();
            save_catalog_db(
                &db,
                catalog("supplier", Some(supplier), "Proveedor nuevo", false),
            )
            .await
            .unwrap();
            archive_record_db(&db, "supplier".into(), supplier, false)
                .await
                .unwrap();
            let row = sqlx::query("SELECT product_name,supplier_name FROM movements")
                .fetch_one(&db)
                .await
                .unwrap();
            assert_eq!(row.get::<String, _>("product_name"), "Agua");
            assert_eq!(row.get::<String, _>("supplier_name"), "Proveedor");
            assert!(archive_record_db(&db, "product".into(), a.0, false)
                .await
                .is_err());
            assert!(save_product_db(&db, product(Some(a.0), "Agua", None))
                .await
                .is_err());
            let mut adjustment = movement(a, "out", 5.0);
            adjustment.adjustment = true;
            assert!(create_movement_db(&db, adjustment).await.is_err());
            let mut adjustment = movement(a, "out", 5.0);
            adjustment.adjustment = true;
            adjustment.reason = Some("Recuento físico".into());
            create_movement_db(&db, adjustment).await.unwrap();
            archive_record_db(&db, "product".into(), a.0, false)
                .await
                .unwrap();
            assert!(create_movement_db(&db, movement(a, "in", 1.0))
                .await
                .is_err());
            archive_record_db(&db, "product".into(), a.0, true)
                .await
                .unwrap();
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM movements")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                2
            );
        });
    }
    #[test]
    fn fractional_stock_reaches_exact_zero() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = fixture(&db, "Granel").await;
            create_movement_db(&db, movement(a, "in", 0.3))
                .await
                .unwrap();
            create_movement_db(&db, movement(a, "out", 0.1))
                .await
                .unwrap();
            create_movement_db(&db, movement(a, "out", 0.2))
                .await
                .unwrap();
            assert_eq!(stock(&db, a.0).await, 0.0);
            assert!(create_movement_db(&db, movement(a, "in", 0.0000001))
                .await
                .is_err());
            archive_record_db(&db, "product".into(), a.0, false)
                .await
                .unwrap();
        });
    }
    #[test]
    fn default_person_and_repeated_initialization() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = save_catalog_db(&db, catalog("person", None, "Ana", true))
                .await
                .unwrap();
            save_catalog_db(&db, catalog("person", None, "Juan", true))
                .await
                .unwrap();
            assert!(save_catalog_db(&db, catalog("person", None, "Ana", true))
                .await
                .is_err());
            assert_eq!(
                sqlx::query_scalar::<_, String>(
                    "SELECT name FROM responsible_persons WHERE is_default=1"
                )
                .fetch_one(&db)
                .await
                .unwrap(),
                "Juan"
            );
            assert!(
                sqlx::query("UPDATE responsible_persons SET is_default=1 WHERE id=?")
                    .bind(a)
                    .execute(&db)
                    .await
                    .is_err()
            );
            fixture(&db, "Agua").await;
            sqlx::query("UPDATE settings SET value='8' WHERE key='stock_low_threshold'")
                .execute(&db)
                .await
                .unwrap();
            sqlx::raw_sql(SCHEMA).execute(&db).await.unwrap();
            sqlx::raw_sql(SCHEMA).execute(&db).await.unwrap();
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM products")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                1
            );
            assert_eq!(
                sqlx::query_scalar::<_, String>(
                    "SELECT value FROM settings WHERE key='stock_low_threshold'"
                )
                .fetch_one(&db)
                .await
                .unwrap(),
                "8"
            );
            assert!(sqlx::query("PRAGMA foreign_key_check")
                .fetch_all(&db)
                .await
                .unwrap()
                .is_empty());
        });
    }
    #[test]
    fn demo_seed_is_coherent_and_idempotent() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            seed_demo_data(&db).await.unwrap();
            let first_products: i64 = sqlx::query_scalar("SELECT count(*) FROM products")
                .fetch_one(&db)
                .await
                .unwrap();
            let first_movements: i64 = sqlx::query_scalar("SELECT count(*) FROM movements")
                .fetch_one(&db)
                .await
                .unwrap();
            assert_eq!(first_products, 6);
            assert_eq!(first_movements, 12);
            assert!(sqlx::query("PRAGMA foreign_key_check")
                .fetch_all(&db)
                .await
                .unwrap()
                .is_empty());
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM products WHERE stock > 0")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                6
            );
            seed_demo_data(&db).await.unwrap();
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM products")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                first_products
            );
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM movements")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                first_movements
            );
        });
    }
}
