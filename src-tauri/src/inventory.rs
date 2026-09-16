use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Connection, Pool, QueryBuilder, Row, Sqlite, SqliteConnection,
};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
use tauri::{async_runtime::RwLock, AppHandle, Manager, State};
use unicode_normalization::UnicodeNormalization;

type Db = Pool<Sqlite>;
const SCHEMA: &str = include_str!("schema.sql");

/// Error serializable de los comandos: el frontend recibe `{ code, message }`
/// y decide con el código, sin depender del texto en español.
#[derive(Debug, Serialize)]
pub struct CommandError {
    pub code: &'static str,
    pub message: String,
}

impl CommandError {
    pub(crate) fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

/// Pool de conexiones conmutable: permite cerrarlo y reabrirlo al restaurar
/// la base de datos desde un archivo de respaldo.
pub struct DbState(RwLock<Db>);

impl DbState {
    pub fn new(pool: Db) -> Self {
        Self(RwLock::new(pool))
    }

    /// Acceso al pool compartido. Se usa un `RwLock` para poder cerrar y
    /// reabrir la conexión al restaurar la base de datos.
    pub(crate) fn pool(&self) -> &RwLock<Db> {
        &self.0
    }
}

pub async fn connect(path: &Path) -> Result<Db, sqlx::Error> {
    // Una única conexión serializa todas las transacciones, por lo que las
    // lecturas y escrituras de stock (leer-y-luego-actualizar) no pueden
    // intercalarse entre comandos concurrentes.
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
    migrate_remove_persons(&pool).await?;
    migrate_allow_deleted_products(&pool).await?;
    Ok(pool)
}

const MOVEMENTS_TABLE: &str = r#"
CREATE TABLE movements_new (
 id INTEGER PRIMARY KEY, product_id INTEGER REFERENCES products(id) ON DELETE SET NULL,
 type TEXT NOT NULL CHECK(type IN ('in','out')),
 quantity REAL NOT NULL CHECK(quantity>0 AND quantity<=1000000000), presentation_id INTEGER,
 presentation_name TEXT NOT NULL, product_name TEXT NOT NULL,
 factor REAL NOT NULL CHECK(factor>0 AND factor<=1000000000),
 base_quantity REAL NOT NULL CHECK(base_quantity>0 AND base_quantity<=1000000000),
 supplier_id INTEGER REFERENCES suppliers(id) ON DELETE SET NULL, supplier_name TEXT,
 notes TEXT, reason TEXT, kind TEXT NOT NULL DEFAULT 'regular' CHECK(kind IN ('regular','adjustment')),
 created_at TEXT NOT NULL DEFAULT (datetime('now')), cancelled_at TEXT, cancellation_reason TEXT,
 FOREIGN KEY(product_id,presentation_id) REFERENCES presentations(product_id,id) ON DELETE SET NULL,
 CHECK(type='in' OR supplier_id IS NULL),
 CHECK(kind='regular' OR (reason IS NOT NULL AND length(trim(reason))>0)),
 CHECK((cancelled_at IS NULL AND cancellation_reason IS NULL) OR
       (cancelled_at IS NOT NULL AND cancellation_reason IS NOT NULL AND length(trim(cancellation_reason))>0))
)"#;

const MOVEMENTS_COLUMNS: &str = "id,product_id,type,quantity,presentation_id,presentation_name,product_name,factor,base_quantity,supplier_id,supplier_name,notes,reason,kind,created_at,cancelled_at,cancellation_reason";

/// Reconstruye `movements` con la definición actual, incluidas las claves
/// foráneas opcionales que permiten conservar movimientos de productos
/// eliminados.
async fn replace_movements_table(db: &Db) -> Result<(), sqlx::Error> {
    let mut tx = db.begin().await?;
    sqlx::query(MOVEMENTS_TABLE).execute(&mut *tx).await?;
    sqlx::query(&format!(
        "INSERT INTO movements_new ({MOVEMENTS_COLUMNS}) SELECT {MOVEMENTS_COLUMNS} FROM movements"
    ))
    .execute(&mut *tx)
    .await?;
    sqlx::query("DROP TABLE movements")
        .execute(&mut *tx)
        .await?;
    sqlx::query("ALTER TABLE movements_new RENAME TO movements")
        .execute(&mut *tx)
        .await?;
    for index in [
        "CREATE INDEX IF NOT EXISTS movements_date ON movements(created_at DESC,id DESC)",
        "CREATE INDEX IF NOT EXISTS movements_product ON movements(product_id,created_at DESC)",
        "CREATE INDEX IF NOT EXISTS movements_supplier ON movements(supplier_id)",
    ] {
        sqlx::query(index).execute(&mut *tx).await?;
    }
    tx.commit().await
}

/// Versiones anteriores asociaban cada movimiento a un responsable.
/// Reconstruye la tabla sin esas columnas y elimina el catálogo de responsables.
async fn migrate_remove_persons(db: &Db) -> Result<(), sqlx::Error> {
    let columns = sqlx::query("PRAGMA table_info(movements)")
        .fetch_all(db)
        .await?;
    let has_columns = columns.iter().any(|row| {
        matches!(
            row.get::<String, _>("name").as_str(),
            "responsible_person_id" | "person_name"
        )
    });
    let has_table: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='responsible_persons')",
    )
    .fetch_one(db)
    .await?;
    if !has_columns && !has_table {
        return Ok(());
    }
    if has_columns {
        replace_movements_table(db).await?;
    }
    sqlx::query("DROP TABLE IF EXISTS responsible_persons")
        .execute(db)
        .await?;
    Ok(())
}

/// Bases anteriores impedían borrar productos o proveedores con movimientos.
/// Reconstruye `movements` con `product_id`/`presentation_id` opcionales y el
/// proveedor limpio al eliminarse, para conservar el historial marcado como
/// "producto/proveedor eliminado".
async fn migrate_allow_deleted_products(db: &Db) -> Result<(), sqlx::Error> {
    let columns = sqlx::query("PRAGMA table_info(movements)")
        .fetch_all(db)
        .await?;
    let product_required = columns.iter().any(|row| {
        row.get::<String, _>("name") == "product_id" && row.get::<i64, _>("notnull") == 1
    });
    let keys = sqlx::query("PRAGMA foreign_key_list(movements)")
        .fetch_all(db)
        .await?;
    let supplier_set_null = keys.iter().any(|row| {
        row.get::<String, _>("from") == "supplier_id"
            && row.get::<String, _>("on_delete") == "SET NULL"
    });
    if !product_required && supplier_set_null {
        return Ok(());
    }
    replace_movements_table(db).await
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
fn error(e: sqlx::Error) -> CommandError {
    if let Some(db) = e.as_database_error() {
        if db.is_unique_violation() {
            let (code, entity) = if db.message().contains("products.") {
                ("duplicate_product", "un producto")
            } else if db.message().contains("suppliers.") {
                ("duplicate_supplier", "un proveedor")
            } else if db.message().contains("categories.") {
                ("duplicate_category", "una categoría")
            } else if db.message().contains("presentations.") {
                (
                    "duplicate_presentation",
                    "una presentación de este producto",
                )
            } else {
                ("duplicate_record", "un registro")
            };
            return CommandError::new(
                code,
                format!("Ya existe {entity} con ese nombre (puede estar archivado)."),
            );
        }
        if db.is_foreign_key_violation() {
            return CommandError::new(
                "foreign_key",
                "El registro asociado no existe o está siendo utilizado.",
            );
        }
    }
    eprintln!("Database error: {e}");
    CommandError::new(
        "database_error",
        "No se pudo completar la operación. Revisa los datos e inténtalo nuevamente.",
    )
}
/// Normaliza un nombre para mostrarlo y para compararlo de forma única:
/// colapsa espacios, aplica NFC y calcula su clave en minúsculas.
pub(crate) fn name(value: &str) -> Result<(String, String), CommandError> {
    let display = value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .nfc()
        .collect::<String>();
    if display.is_empty() || display.chars().count() > 200 {
        return Err(CommandError::new(
            "invalid_name",
            "El nombre debe tener entre 1 y 200 caracteres.",
        ));
    }
    let key = display.to_lowercase().nfc().collect();
    Ok((display, key))
}
fn positive(value: f64) -> Result<(), CommandError> {
    if !value.is_finite() || value <= 0.0 || value > 1e9 {
        return Err(CommandError::new(
            "invalid_quantity",
            "La cantidad y el factor deben ser positivos y como máximo 1.000 millones.",
        ));
    }
    if value < 0.000001 || (value - (value * 1e6).round() / 1e6).abs() > 1e-10 {
        return Err(CommandError::new(
            "invalid_decimals",
            "Se admiten hasta 6 decimales en cantidades, factores y conversiones.",
        ));
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
pub async fn save_product(
    db: State<'_, DbState>,
    input: ProductInput,
) -> Result<i64, CommandError> {
    let db = db.0.read().await;
    save_product_db(&db, input).await
}
async fn save_product_db(db: &Db, input: ProductInput) -> Result<i64, CommandError> {
    let (display, key) = name(&input.name)?;
    if input.presentations.is_empty()
        || input
            .presentations
            .iter()
            .filter(|p| p.factor == 1.0)
            .count()
            != 1
    {
        return Err(CommandError::new(
            "invalid_presentations",
            "Selecciona exactamente una presentación base con factor 1.",
        ));
    }
    let mut keys = std::collections::HashSet::new();
    let mut ids = std::collections::HashSet::new();
    for p in &input.presentations {
        positive(p.factor)?;
        if !keys.insert(name(&p.name)?.1) {
            return Err(CommandError::new(
                "duplicate_presentation",
                "Hay presentaciones con el mismo nombre.",
            ));
        }
        if let Some(id) = p.id {
            if !ids.insert(id) {
                return Err(CommandError::new(
                    "duplicate_presentation",
                    "Presentación repetida.",
                ));
            }
        }
    }
    let mut tx = db.begin().await.map_err(error)?;
    let id = if let Some(id) = input.id {
        let row=sqlx::query("SELECT base_presentation_id, EXISTS(SELECT 1 FROM movements WHERE product_id=products.id) AS used FROM products WHERE id=? AND archived=0")
            .bind(id).fetch_optional(&mut *tx).await.map_err(error)?.ok_or_else(|| CommandError::new("unavailable", "El producto no está disponible."))?;
        if row.get::<bool, _>("used") {
            let base = input
                .presentations
                .iter()
                .find(|p| p.factor == 1.0)
                .unwrap();
            if base.id != Some(row.get("base_presentation_id")) {
                return Err(CommandError::new(
                    "base_unit_locked",
                    "No se puede cambiar la unidad base de un producto con historial.",
                ));
            }
        }
        sqlx::query("UPDATE products SET name=?,name_key=?,description=?,category_id=?,updated_at=datetime('now') WHERE id=?")
            .bind(&display).bind(&key).bind(&input.description).bind(input.category_id).bind(id).execute(&mut *tx).await.map_err(error)?;
        id
    } else {
        if !ids.is_empty() {
            return Err(CommandError::new(
                "invalid_presentations",
                "Las presentaciones nuevas no deben tener identificador.",
            ));
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
                .bind(pid).bind(id).fetch_optional(&mut *tx).await.map_err(error)?.ok_or_else(|| CommandError::new("invalid_presentations", "La presentación no pertenece a este producto."))?;
            if old.get::<bool, _>("used") && old.get::<f64, _>("factor") != p.factor {
                return Err(CommandError::new("factor_locked", "Una presentación con historial no puede cambiar de factor. Crea otra presentación."));
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
}
#[tauri::command]
pub async fn save_catalog(
    db: State<'_, DbState>,
    input: CatalogInput,
) -> Result<i64, CommandError> {
    let db = db.0.read().await;
    save_catalog_db(&db, input).await
}
async fn save_catalog_db(db: &Db, input: CatalogInput) -> Result<i64, CommandError> {
    let (display, key) = name(&input.name)?;
    let mut tx = db.begin().await.map_err(error)?;
    let result=match input.kind.as_str() {
        "supplier" => sqlx::query("INSERT INTO suppliers(id,name,name_key,contact,notes) VALUES(?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET name=excluded.name,name_key=excluded.name_key,contact=excluded.contact,notes=excluded.notes")
            .bind(input.id).bind(display).bind(key).bind(input.contact).bind(input.notes).execute(&mut *tx).await,
        "category" => sqlx::query("INSERT INTO categories(id,name,name_key,color,icon) VALUES(?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET name=excluded.name,name_key=excluded.name_key,color=excluded.color,icon=excluded.icon")
            .bind(input.id).bind(display).bind(key).bind(input.color).bind(input.icon).execute(&mut *tx).await,
        _ => return Err(CommandError::new("invalid_kind", "Tipo de registro inválido."))
    }.map_err(error)?;
    tx.commit().await.map_err(error)?;
    Ok(input.id.unwrap_or(result.last_insert_rowid()))
}
#[tauri::command]
pub async fn archive_record(
    db: State<'_, DbState>,
    kind: String,
    id: i64,
    restore: bool,
) -> Result<(), CommandError> {
    let db = db.0.read().await;
    archive_record_db(&db, kind, id, restore).await
}
async fn archive_record_db(
    db: &Db,
    kind: String,
    id: i64,
    restore: bool,
) -> Result<(), CommandError> {
    let table = match kind.as_str() {
        "product" => "products",
        "supplier" => "suppliers",
        "category" => "categories",
        _ => {
            return Err(CommandError::new(
                "invalid_operation",
                "Operación inválida.",
            ))
        }
    };
    let sql = match (kind.as_str(), restore) {
        ("product", false) => "UPDATE products SET archived=1 WHERE id=? AND stock=0",
        ("product", true) => "UPDATE products SET archived=0 WHERE id=?",
        ("supplier", false) => "UPDATE suppliers SET archived=1 WHERE id=?",
        ("supplier", true) => "UPDATE suppliers SET archived=0 WHERE id=?",
        ("category", false) => "DELETE FROM categories WHERE id=?",
        _ => {
            return Err(CommandError::new(
                "invalid_operation",
                "Operación inválida.",
            ))
        }
    };
    let affected = sqlx::query(sql)
        .bind(id)
        .execute(db)
        .await
        .map_err(error)?
        .rows_affected();
    if affected == 1 {
        return Ok(());
    }
    let exists: bool =
        sqlx::query_scalar(&format!("SELECT EXISTS(SELECT 1 FROM {table} WHERE id=?)"))
            .bind(id)
            .fetch_one(db)
            .await
            .map_err(error)?;
    if exists {
        Err(CommandError::new(
            "archive_requires_zero_stock",
            "El producto aún tiene stock. Registra un ajuste antes de archivarlo.",
        ))
    } else {
        Err(CommandError::new("not_found", "El registro no existe."))
    }
}
#[tauri::command]
pub async fn delete_product(
    db: State<'_, DbState>,
    id: i64,
    password: String,
) -> Result<(), CommandError> {
    let db = db.0.read().await;
    delete_product_db(&db, id, password).await
}
/// Elimina definitivamente un producto archivado. Sus movimientos se
/// conservan: las claves foráneas se limpian y el historial queda marcado como
/// "producto eliminado" gracias a los snapshots de nombre.
async fn delete_product_db(db: &Db, id: i64, password: String) -> Result<(), CommandError> {
    check_delete_password(db, &password).await?;
    let mut tx = db.begin().await.map_err(error)?;
    let row = sqlx::query("SELECT archived, stock FROM products WHERE id=?")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(error)?
        .ok_or_else(|| CommandError::new("not_found", "El producto no existe."))?;
    if row.get::<i64, _>("archived") != 1 {
        return Err(CommandError::new(
            "not_archived",
            "Solo se pueden eliminar definitivamente productos archivados.",
        ));
    }
    if row.get::<f64, _>("stock") != 0.0 {
        return Err(CommandError::new(
            "product_has_stock",
            "El producto aún tiene stock. Registra un ajuste antes de eliminarlo.",
        ));
    }
    sqlx::query("DELETE FROM presentations WHERE product_id=?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(error)?;
    sqlx::query("DELETE FROM products WHERE id=?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(error)?;
    tx.commit().await.map_err(error)
}
#[tauri::command]
pub async fn delete_supplier(
    db: State<'_, DbState>,
    id: i64,
    password: String,
) -> Result<(), CommandError> {
    let db = db.0.read().await;
    delete_supplier_db(&db, id, password).await
}
/// Elimina definitivamente un proveedor archivado. Sus movimientos conservan
/// el nombre del proveedor y quedan marcados como "proveedor eliminado".
async fn delete_supplier_db(db: &Db, id: i64, password: String) -> Result<(), CommandError> {
    check_delete_password(db, &password).await?;
    let mut tx = db.begin().await.map_err(error)?;
    let archived: i64 = sqlx::query_scalar("SELECT archived FROM suppliers WHERE id=?")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(error)?
        .ok_or_else(|| CommandError::new("not_found", "El proveedor no existe."))?;
    if archived != 1 {
        return Err(CommandError::new(
            "not_archived",
            "Solo se pueden eliminar definitivamente proveedores archivados.",
        ));
    }
    sqlx::query("DELETE FROM suppliers WHERE id=?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(error)?;
    tx.commit().await.map_err(error)
}
#[derive(Deserialize)]
pub struct MovementInput {
    product_id: i64,
    #[serde(rename = "type")]
    movement_type: String,
    quantity: f64,
    presentation_id: i64,
    supplier_id: Option<i64>,
    notes: Option<String>,
    #[serde(default)]
    adjustment: bool,
    reason: Option<String>,
}
#[tauri::command]
pub async fn create_movement(
    db: State<'_, DbState>,
    input: MovementInput,
) -> Result<(), CommandError> {
    let db = db.0.read().await;
    create_movement_db(&db, input).await
}
async fn create_movement_db(db: &Db, input: MovementInput) -> Result<(), CommandError> {
    positive(input.quantity)?;
    if !["in", "out"].contains(&input.movement_type.as_str()) {
        return Err(CommandError::new(
            "invalid_type",
            "Tipo de movimiento inválido.",
        ));
    }
    if input.movement_type == "out" && input.supplier_id.is_some() {
        return Err(CommandError::new(
            "supplier_not_allowed",
            "Una salida no admite proveedor.",
        ));
    }
    if input.adjustment && input.reason.as_deref().unwrap_or("").trim().is_empty() {
        return Err(CommandError::new(
            "adjustment_reason_required",
            "Indica el motivo del ajuste.",
        ));
    }
    let mut tx = db.begin().await.map_err(error)?;
    let row=sqlx::query("SELECT r.factor,r.name,p.name AS product_name FROM presentations r JOIN products p ON p.id=r.product_id WHERE r.id=? AND p.id=? AND r.archived=0 AND p.archived=0")
        .bind(input.presentation_id).bind(input.product_id).fetch_optional(&mut *tx).await.map_err(error)?.ok_or_else(|| CommandError::new("unavailable", "El producto o la presentación no están disponibles."))?;
    let factor: f64 = row.get("factor");
    let base = input.quantity * factor;
    positive(base)?;
    let base = (base * 1e6).round() / 1e6;
    let supplier_name = if let Some(id) = input.supplier_id {
        Some(
            sqlx::query_scalar::<_, String>("SELECT name FROM suppliers WHERE id=? AND archived=0")
                .bind(id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(error)?
                .ok_or_else(|| {
                    CommandError::new("unavailable", "El proveedor no está disponible.")
                })?,
        )
    } else {
        None
    };
    let delta = if input.movement_type == "in" {
        base
    } else {
        -base
    };
    if sqlx::query("UPDATE products SET stock=round(stock+?,6),updated_at=datetime('now') WHERE id=? AND round(stock+?,6)>=0 AND round(stock+?,6)<=1000000000")
        .bind(delta).bind(input.product_id).bind(delta).bind(delta).execute(&mut *tx).await.map_err(error)?.rows_affected()!=1 {
        return Err(CommandError::new(
            "insufficient_stock",
            "Stock insuficiente o cantidad fuera del límite permitido.",
        ));
    }
    sqlx::query("INSERT INTO movements(product_id,type,quantity,presentation_id,presentation_name,product_name,factor,base_quantity,supplier_id,supplier_name,notes,kind,reason) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?)")
        .bind(input.product_id).bind(input.movement_type).bind(input.quantity).bind(input.presentation_id)
        .bind(row.get::<String,_>("name")).bind(row.get::<String,_>("product_name")).bind(factor).bind(base)
        .bind(input.supplier_id).bind(&supplier_name).bind(input.notes)
        .bind(if input.adjustment {"adjustment"} else {"regular"}).bind(input.reason)
        .execute(&mut *tx).await.map_err(error)?;
    tx.commit().await.map_err(error)
}
#[tauri::command]
pub async fn cancel_movement(
    db: State<'_, DbState>,
    id: i64,
    reason: String,
) -> Result<(), CommandError> {
    let db = db.0.read().await;
    cancel_movement_db(&db, id, reason).await
}
async fn cancel_movement_db(db: &Db, id: i64, reason: String) -> Result<(), CommandError> {
    if reason.trim().is_empty() {
        return Err(CommandError::new(
            "cancellation_reason_required",
            "Indica el motivo de la anulación.",
        ));
    }
    let mut tx = db.begin().await.map_err(error)?;
    if sqlx::query("UPDATE movements SET cancelled_at=datetime('now'),cancellation_reason=? WHERE id=? AND cancelled_at IS NULL")
        .bind(reason.trim()).bind(id).execute(&mut *tx).await.map_err(error)?.rows_affected()!=1 { return Err(CommandError::new("already_cancelled", "El movimiento ya está anulado o no existe.")); }
    let row = sqlx::query("SELECT product_id,type,base_quantity FROM movements WHERE id=?")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(error)?;
    // Los movimientos de productos eliminados conservan su registro, pero ya
    // no tienen stock que revertir.
    if let Some(product_id) = row.get::<Option<i64>, _>("product_id") {
        let base: f64 = row.get("base_quantity");
        let delta = if row.get::<String, _>("type") == "in" {
            -base
        } else {
            base
        };
        if sqlx::query("UPDATE products SET stock=round(stock+?,6),updated_at=datetime('now') WHERE id=? AND archived=0 AND round(stock+?,6)>=0 AND round(stock+?,6)<=1000000000")
            .bind(delta).bind(product_id).bind(delta).bind(delta).execute(&mut *tx).await.map_err(error)?.rows_affected()!=1 {
            return Err(CommandError::new(
                "insufficient_stock",
                "No se puede anular: stock insuficiente o producto archivado.",
            ));
        }
    }
    tx.commit().await.map_err(error)
}
/// Contraseña en texto plano que protege las eliminaciones: movimientos en
/// lote y borrado definitivo de productos y proveedores archivados.
const DEFAULT_DELETE_PASSWORD: &str = "admin";

async fn delete_password_db(db: &Db) -> Result<String, CommandError> {
    let value =
        sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key='delete_password'")
            .fetch_optional(db)
            .await
            .map_err(error)?;
    Ok(value.unwrap_or_else(|| DEFAULT_DELETE_PASSWORD.into()))
}

/// Valida la contraseña de administración que protege las eliminaciones.
async fn check_delete_password(db: &Db, password: &str) -> Result<(), CommandError> {
    if password.trim() != delete_password_db(db).await? {
        return Err(CommandError::new(
            "wrong_password",
            "Contraseña incorrecta.",
        ));
    }
    Ok(())
}

/// Filtro de borrado en lote de movimientos. `from` es inclusivo y `to`
/// exclusivo, ambos en UTC (`YYYY-MM-DD HH:MM:SS`), como en `page_movements`.
#[derive(Deserialize)]
pub struct MovementDeleteFilter {
    pub mode: String,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
}

/// Filtro ya validado: `limit` solo aplica a los modos por cantidad y
/// `descending` indica si el recorte empieza por los movimientos más recientes.
struct ValidatedDeleteFilter {
    limit: Option<i64>,
    descending: bool,
    from: Option<String>,
    to: Option<String>,
}

fn validate_delete_filter(
    filter: &MovementDeleteFilter,
) -> Result<ValidatedDeleteFilter, CommandError> {
    let from = filter
        .from
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let to = filter
        .to
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    match filter.mode.as_str() {
        "recent" | "oldest" => {
            let count = filter.count.unwrap_or(0);
            if !(1..=1_000_000_000).contains(&count) {
                return Err(CommandError::new(
                    "invalid_count",
                    "Indica una cantidad válida de movimientos.",
                ));
            }
            Ok(ValidatedDeleteFilter {
                limit: Some(count),
                descending: filter.mode == "recent",
                from: None,
                to: None,
            })
        }
        "from" => {
            let from = from
                .ok_or_else(|| CommandError::new("invalid_dates", "Indica la fecha de inicio."))?;
            Ok(ValidatedDeleteFilter {
                limit: None,
                descending: false,
                from: Some(from.to_string()),
                to: None,
            })
        }
        "until" => {
            let to =
                to.ok_or_else(|| CommandError::new("invalid_dates", "Indica la fecha límite."))?;
            Ok(ValidatedDeleteFilter {
                limit: None,
                descending: false,
                from: None,
                to: Some(to.to_string()),
            })
        }
        "range" => {
            let (Some(from), Some(to)) = (from, to) else {
                return Err(CommandError::new("invalid_dates", "Indica ambas fechas."));
            };
            if from >= to {
                return Err(CommandError::new(
                    "invalid_dates",
                    "La fecha inicial debe ser anterior a la final.",
                ));
            }
            Ok(ValidatedDeleteFilter {
                limit: None,
                descending: false,
                from: Some(from.to_string()),
                to: Some(to.to_string()),
            })
        }
        _ => Err(CommandError::new(
            "invalid_filter",
            "Modo de eliminación inválido.",
        )),
    }
}

/// Añade las condiciones de fecha del filtro a un `QueryBuilder` sobre `movements`.
fn apply_delete_filter(query: &mut QueryBuilder<Sqlite>, filter: &ValidatedDeleteFilter) {
    let mut separator = " WHERE ";
    if let Some(from) = &filter.from {
        query
            .push(separator)
            .push("created_at>=")
            .push_bind(from.clone());
        separator = " AND ";
    }
    if let Some(to) = &filter.to {
        query
            .push(separator)
            .push("created_at<")
            .push_bind(to.clone());
    }
}

/// Cuenta los movimientos que borraría el filtro, para previsualizarlo.
#[tauri::command]
pub async fn count_movements(
    db: State<'_, DbState>,
    filter: MovementDeleteFilter,
) -> Result<i64, CommandError> {
    let db = db.0.read().await;
    count_movements_db(&db, &filter).await
}
async fn count_movements_db(db: &Db, filter: &MovementDeleteFilter) -> Result<i64, CommandError> {
    let filter = validate_delete_filter(filter)?;
    let mut query = QueryBuilder::<Sqlite>::new("SELECT count(*) FROM movements");
    apply_delete_filter(&mut query, &filter);
    let row = query.build().fetch_one(db).await.map_err(error)?;
    row.try_get::<i64, _>(0).map_err(error)
}

/// Borra en lote los movimientos que coincidan con el filtro (por cantidad o
/// por fechas). El stock de los productos no se modifica: el historial se
/// recorta sin revertir efectos.
#[tauri::command]
pub async fn delete_movements(
    db: State<'_, DbState>,
    filter: MovementDeleteFilter,
    password: String,
) -> Result<i64, CommandError> {
    let db = db.0.read().await;
    delete_movements_db(&db, filter, password).await
}
async fn delete_movements_db(
    db: &Db,
    filter: MovementDeleteFilter,
    password: String,
) -> Result<i64, CommandError> {
    let filter = validate_delete_filter(&filter)?;
    check_delete_password(db, &password).await?;
    let mut tx = db.begin().await.map_err(error)?;
    let deleted = if let Some(limit) = filter.limit {
        let order = if filter.descending {
            "created_at DESC, id DESC"
        } else {
            "created_at ASC, id ASC"
        };
        sqlx::query(&format!(
            "DELETE FROM movements WHERE id IN (SELECT id FROM movements ORDER BY {order} LIMIT ?)"
        ))
        .bind(limit)
        .execute(&mut *tx)
        .await
        .map_err(error)?
        .rows_affected()
    } else {
        let mut query = QueryBuilder::<Sqlite>::new("DELETE FROM movements");
        apply_delete_filter(&mut query, &filter);
        query
            .build()
            .execute(&mut *tx)
            .await
            .map_err(error)?
            .rows_affected()
    };
    tx.commit().await.map_err(error)?;
    Ok(deleted as i64)
}

#[tauri::command]
pub async fn change_delete_password(
    db: State<'_, DbState>,
    old_password: String,
    new_password: String,
) -> Result<(), CommandError> {
    let db = db.0.read().await;
    change_delete_password_db(&db, old_password, new_password).await
}
async fn change_delete_password_db(
    db: &Db,
    old_password: String,
    new_password: String,
) -> Result<(), CommandError> {
    if old_password.trim() != delete_password_db(db).await? {
        return Err(CommandError::new(
            "wrong_password",
            "La contraseña actual es incorrecta.",
        ));
    }
    let new_password = new_password.trim();
    if new_password.is_empty() || new_password.chars().count() > 200 {
        return Err(CommandError::new(
            "invalid_password",
            "La nueva contraseña debe tener entre 1 y 200 caracteres.",
        ));
    }
    sqlx::query("INSERT INTO settings(key,value) VALUES('delete_password',?) ON CONFLICT(key) DO UPDATE SET value=excluded.value")
        .bind(new_password)
        .execute(db)
        .await
        .map_err(error)?;
    Ok(())
}

#[tauri::command]
pub async fn save_settings(
    db: State<'_, DbState>,
    low: Option<f64>,
    medium: Option<f64>,
    view: Option<String>,
) -> Result<(), CommandError> {
    let db = db.0.read().await;
    save_settings_db(&db, low, medium, view).await
}
async fn save_settings_db(
    db: &Db,
    low: Option<f64>,
    medium: Option<f64>,
    view: Option<String>,
) -> Result<(), CommandError> {
    let mut tx = db.begin().await.map_err(error)?;
    if let (Some(low), Some(medium)) = (low, medium) {
        if !low.is_finite() || !medium.is_finite() || low < 0.0 || medium <= low || medium > 1e9 {
            return Err(CommandError::new(
                "invalid_thresholds",
                "Los límites deben cumplir: 0 ≤ bajo < medio ≤ 1.000 millones.",
            ));
        }
        for (key, value) in [
            ("stock_low_threshold", low),
            ("stock_medium_threshold", medium),
        ] {
            let updated = sqlx::query("UPDATE settings SET value=? WHERE key=?")
                .bind(value.to_string())
                .bind(key)
                .execute(&mut *tx)
                .await
                .map_err(error)?
                .rows_affected();
            if updated != 1 {
                return Err(CommandError::new(
                    "database_error",
                    "No se pudo guardar la configuración.",
                ));
            }
        }
    } else if low.is_some() || medium.is_some() {
        return Err(CommandError::new(
            "invalid_thresholds",
            "Indica ambos límites.",
        ));
    }
    if let Some(view) = view {
        if !["dashboard", "products", "movements", "suppliers"].contains(&view.as_str()) {
            return Err(CommandError::new("invalid_view", "Vista inválida."));
        }
        let updated = sqlx::query("UPDATE settings SET value=? WHERE key='default_view'")
            .bind(view)
            .execute(&mut *tx)
            .await
            .map_err(error)?
            .rows_affected();
        if updated != 1 {
            return Err(CommandError::new(
                "database_error",
                "No se pudo guardar la configuración.",
            ));
        }
    }
    tx.commit().await.map_err(error)
}

/// Absolute path of the SQLite database file managed by the application.
pub(crate) fn db_path(app: &AppHandle) -> Result<PathBuf, CommandError> {
    let dir = app.path().app_config_dir().map_err(|e| {
        eprintln!("Config dir error: {e}");
        CommandError::new(
            "config_dir",
            "No se pudo resolver el directorio de configuración.",
        )
    })?;
    Ok(dir.join("inventory.db"))
}

/// Check that a file starts with the SQLite database header.
fn looks_like_sqlite(path: &Path) -> bool {
    use std::io::Read;
    let mut f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let mut header = [0u8; 16];
    f.read_exact(&mut header).is_ok() && &header == b"SQLite format 3\0"
}

/// Tables that a valid backup of this application must contain.
const BACKUP_TABLES: [&str; 6] = [
    "categories",
    "suppliers",
    "products",
    "presentations",
    "movements",
    "settings",
];

/// Validate that `path` is a SQLite database containing this app's tables.
async fn validate_backup(path: &Path) -> Result<(), CommandError> {
    if !path.exists() {
        return Err(CommandError::new(
            "backup_not_found",
            "El archivo de respaldo no existe.",
        ));
    }
    if !looks_like_sqlite(path) {
        return Err(CommandError::new(
            "invalid_backup",
            "El archivo seleccionado no es una base de datos SQLite válida.",
        ));
    }
    let options = SqliteConnectOptions::new().filename(path).read_only(true);
    let mut conn = SqliteConnection::connect_with(&options)
        .await
        .map_err(|_| {
            CommandError::new("invalid_backup", "No se pudo abrir el archivo de respaldo.")
        })?;
    let placeholders = std::iter::repeat_n("?", BACKUP_TABLES.len())
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name IN ({placeholders})"
    );
    let mut query = sqlx::query_scalar::<_, i64>(&sql);
    for table in BACKUP_TABLES {
        query = query.bind(table);
    }
    let found = query.fetch_one(&mut conn).await.map_err(|_| {
        CommandError::new(
            "invalid_backup",
            "El archivo de respaldo está dañado o no es compatible.",
        )
    })?;
    let _ = conn.close().await;
    if found != BACKUP_TABLES.len() as i64 {
        return Err(CommandError::new(
            "invalid_backup",
            "El archivo no parece un respaldo válido de Gestor de Inventario.",
        ));
    }
    Ok(())
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut value = path.as_os_str().to_os_string();
    value.push(suffix);
    PathBuf::from(value)
}

/// Create a consistent snapshot of the live database into `dest`. The snapshot
/// is written next to the destination and renamed at the end, so a failure
/// never destroys a previous backup.
async fn backup_database_db(db: &Db, dest: &Path) -> Result<(), CommandError> {
    if let Some(parent) = dest.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(CommandError::new(
                "backup_folder_missing",
                "La carpeta seleccionada no existe.",
            ));
        }
    }
    let temp = with_suffix(dest, ".tmp");
    let _ = std::fs::remove_file(&temp);
    if let Err(e) = sqlx::query("VACUUM INTO ?")
        .bind(temp.to_string_lossy().as_ref())
        .execute(db)
        .await
    {
        eprintln!("Backup error: {e}");
        let _ = std::fs::remove_file(&temp);
        return Err(CommandError::new(
            "backup_failed",
            "No se pudo crear el respaldo.",
        ));
    }
    if let Err(e) = std::fs::rename(&temp, dest) {
        eprintln!("Backup rename error: {e}");
        let _ = std::fs::remove_file(&temp);
        return Err(CommandError::new(
            "backup_failed",
            "No se pudo sobrescribir el archivo de respaldo.",
        ));
    }
    Ok(())
}

fn same_file(a: &Path, b: &Path) -> bool {
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

/// Replace the live database at `dest` with the backup at `src`, keeping a
/// safety copy of the previous data in `safety`.
async fn restore_database_db(
    db: &RwLock<Db>,
    dest: &Path,
    src: &Path,
    safety: &Path,
) -> Result<(), CommandError> {
    validate_backup(src).await?;
    if same_file(src, dest) {
        return Err(CommandError::new(
            "restore_same_file",
            "Selecciona un archivo distinto de la base de datos actual.",
        ));
    }
    let mut pool = db.write().await;
    backup_database_db(&pool, safety).await.map_err(|_| {
        CommandError::new(
            "restore_safety_failed",
            "No se pudo guardar una copia de seguridad antes de restaurar.",
        )
    })?;
    pool.close().await;
    for suffix in ["-wal", "-shm"] {
        let mut journal = dest.as_os_str().to_os_string();
        journal.push(suffix);
        let _ = std::fs::remove_file(journal);
    }
    std::fs::copy(src, dest).map_err(|e| {
        eprintln!("Restore copy error: {e}");
        CommandError::new("restore_failed", "No se pudo reemplazar la base de datos.")
    })?;
    match connect(dest).await {
        Ok(reopened) => {
            *pool = reopened;
            Ok(())
        }
        Err(e) => {
            eprintln!("Restore error: {e}");
            let _ = std::fs::copy(safety, dest);
            match connect(dest).await {
                Ok(recovered) => {
                    *pool = recovered;
                    Err(CommandError::new(
                        "restore_failed",
                        "No se pudo abrir el respaldo; se restauraron los datos anteriores.",
                    ))
                }
                Err(_) => Err(CommandError::new(
                    "restore_failed",
                    "No se pudo restaurar la base de datos. Reinicia la aplicación.",
                )),
            }
        }
    }
}

/// Create a backup file of the current database at `dest_path`.
#[tauri::command]
pub async fn backup_database(
    db: State<'_, DbState>,
    dest_path: String,
) -> Result<(), CommandError> {
    let db = db.0.read().await;
    backup_database_db(&db, Path::new(&dest_path)).await
}

/// Replace the live database with the backup file at `src_path`.
#[tauri::command]
pub async fn restore_database(
    app: AppHandle,
    db: State<'_, DbState>,
    src_path: String,
) -> Result<(), CommandError> {
    let dest = db_path(&app)?;
    let safety = dest
        .parent()
        .map(|dir| dir.join("pre-restore-backup.db"))
        .ok_or_else(|| {
            CommandError::new(
                "config_dir",
                "No se pudo resolver el directorio de la base de datos.",
            )
        })?;
    restore_database_db(&db.0, &dest, Path::new(&src_path), &safety).await
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
    fn catalog(kind: &str, id: Option<i64>, name: &str) -> CatalogInput {
        CatalogInput {
            kind: kind.into(),
            id,
            name: name.into(),
            contact: None,
            notes: None,
            color: None,
            icon: None,
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
    fn legacy_databases_drop_responsible_persons_and_keep_movements() {
        tauri::async_runtime::block_on(async {
            let dir = temp_dir("migrate-persons");
            let live = dir.join("inventory.db");
            let db = connect(&live).await.unwrap();
            let water = fixture(&db, "Agua").await;
            let mut input = movement(water, "in", 4.0);
            input.notes = Some("Compra inicial".into());
            create_movement_db(&db, input).await.unwrap();
            sqlx::raw_sql(
                "CREATE TABLE responsible_persons (
                   id INTEGER PRIMARY KEY, name TEXT NOT NULL, name_key TEXT NOT NULL UNIQUE,
                   is_default INTEGER NOT NULL DEFAULT 0, archived INTEGER NOT NULL DEFAULT 0
                 );
                 INSERT INTO responsible_persons(id,name,name_key,is_default) VALUES(1,'Ana','ana',1);
                 ALTER TABLE movements ADD COLUMN responsible_person_id INTEGER REFERENCES responsible_persons(id);
                 ALTER TABLE movements ADD COLUMN person_name TEXT;
                 UPDATE movements SET responsible_person_id=1, person_name='Ana';",
            )
            .execute(&db)
            .await
            .unwrap();
            db.close().await;

            let migrated = connect(&live).await.unwrap();
            let columns = sqlx::query("PRAGMA table_info(movements)")
                .fetch_all(&migrated)
                .await
                .unwrap();
            assert!(columns.iter().all(|row| {
                !matches!(
                    row.get::<String, _>("name").as_str(),
                    "responsible_person_id" | "person_name"
                )
            }));
            let persons_table: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='responsible_persons')",
            )
            .fetch_one(&migrated)
            .await
            .unwrap();
            assert!(!persons_table);
            let (product_name, notes): (String, Option<String>) =
                sqlx::query_as("SELECT product_name,notes FROM movements WHERE id=1")
                    .fetch_one(&migrated)
                    .await
                    .unwrap();
            assert_eq!(product_name, "Agua");
            assert_eq!(notes.as_deref(), Some("Compra inicial"));
            assert_eq!(stock(&migrated, water.0).await, 4.0);
            assert!(sqlx::query("PRAGMA foreign_key_check")
                .fetch_all(&migrated)
                .await
                .unwrap()
                .is_empty());
            migrated.close().await;
            let _ = std::fs::remove_dir_all(&dir);
        });
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
            for kind in ["supplier", "category"] {
                let id = save_catalog_db(&db, catalog(kind, None, "  ÁRBOL  Norte "))
                    .await
                    .unwrap();
                assert!(
                    save_catalog_db(&db, catalog(kind, None, "a\u{301}rbol norte"))
                        .await
                        .is_err()
                );
                save_catalog_db(&db, catalog(kind, Some(id), "Árbol Norte"))
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
            let supplier = save_catalog_db(&db, catalog("supplier", None, "Proveedor"))
                .await
                .unwrap();
            let mut input = movement(a, "in", 5.0);
            input.supplier_id = Some(supplier);
            create_movement_db(&db, input).await.unwrap();
            save_product_db(&db, product(Some(a.0), "Agua nueva", Some(a.1)))
                .await
                .unwrap();
            save_catalog_db(&db, catalog("supplier", Some(supplier), "Proveedor nuevo"))
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
    fn archived_products_can_be_deleted_keeping_their_movements() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = fixture(&db, "Agua").await;
            create_movement_db(&db, movement(a, "in", 5.0))
                .await
                .unwrap();
            create_movement_db(&db, movement(a, "out", 5.0))
                .await
                .unwrap();
            // Solo se eliminan productos archivados y con la contraseña correcta.
            assert!(delete_product_db(&db, a.0, "admin".into()).await.is_err());
            archive_record_db(&db, "product".into(), a.0, false)
                .await
                .unwrap();
            assert!(delete_product_db(&db, a.0, "otra".into()).await.is_err());
            delete_product_db(&db, a.0, "admin".into()).await.unwrap();
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM products")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                0
            );
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM presentations")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                0
            );
            let rows = sqlx::query(
                "SELECT product_id,presentation_id,product_name FROM movements ORDER BY id",
            )
            .fetch_all(&db)
            .await
            .unwrap();
            assert_eq!(rows.len(), 2);
            for row in &rows {
                assert_eq!(row.get::<Option<i64>, _>("product_id"), None);
                assert_eq!(row.get::<Option<i64>, _>("presentation_id"), None);
                assert_eq!(row.get::<String, _>("product_name"), "Agua");
            }
            assert!(sqlx::query("PRAGMA foreign_key_check")
                .fetch_all(&db)
                .await
                .unwrap()
                .is_empty());
            // Anular un movimiento de un producto eliminado no revierte stock.
            let id: i64 = sqlx::query_scalar("SELECT id FROM movements ORDER BY id LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap();
            cancel_movement_db(&db, id, "Error de carga".into())
                .await
                .unwrap();
        });
    }
    #[test]
    fn legacy_movements_with_required_product_become_deletable() {
        tauri::async_runtime::block_on(async {
            let dir = temp_dir("migrate-deleted-products");
            let live = dir.join("inventory.db");
            let db = connect(&live).await.unwrap();
            let a = fixture(&db, "Agua").await;
            create_movement_db(&db, movement(a, "in", 2.0))
                .await
                .unwrap();
            // Reconstruye la tabla con la definición anterior, que exigía
            // producto y presentación que existieran siempre.
            sqlx::raw_sql(
                "CREATE TABLE movements_old (
                   id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL REFERENCES products(id),
                   type TEXT NOT NULL, quantity REAL NOT NULL, presentation_id INTEGER NOT NULL,
                   presentation_name TEXT NOT NULL, product_name TEXT NOT NULL,
                   factor REAL NOT NULL, base_quantity REAL NOT NULL,
                   supplier_id INTEGER REFERENCES suppliers(id), supplier_name TEXT,
                   notes TEXT, reason TEXT, kind TEXT NOT NULL DEFAULT 'regular',
                   created_at TEXT NOT NULL DEFAULT (datetime('now')), cancelled_at TEXT, cancellation_reason TEXT,
                   FOREIGN KEY(product_id,presentation_id) REFERENCES presentations(product_id,id)
                 );
                 INSERT INTO movements_old
                   (id,product_id,type,quantity,presentation_id,presentation_name,product_name,factor,base_quantity,kind,created_at)
                 SELECT id,product_id,type,quantity,presentation_id,presentation_name,product_name,factor,base_quantity,kind,created_at FROM movements;
                 DROP TABLE movements;
                 ALTER TABLE movements_old RENAME TO movements;",
            )
            .execute(&db)
            .await
            .unwrap();
            db.close().await;

            let migrated = connect(&live).await.unwrap();
            let columns = sqlx::query("PRAGMA table_info(movements)")
                .fetch_all(&migrated)
                .await
                .unwrap();
            assert!(columns.iter().any(|row| {
                row.get::<String, _>("name") == "product_id" && row.get::<i64, _>("notnull") == 0
            }));
            let keys = sqlx::query("PRAGMA foreign_key_list(movements)")
                .fetch_all(&migrated)
                .await
                .unwrap();
            assert!(keys.iter().any(|row| {
                row.get::<String, _>("from") == "supplier_id"
                    && row.get::<String, _>("on_delete") == "SET NULL"
            }));
            create_movement_db(&migrated, movement(a, "out", 2.0))
                .await
                .unwrap();
            archive_record_db(&migrated, "product".into(), a.0, false)
                .await
                .unwrap();
            delete_product_db(&migrated, a.0, "admin".into())
                .await
                .unwrap();
            let (product_id, name): (Option<i64>, String) =
                sqlx::query_as("SELECT product_id,product_name FROM movements LIMIT 1")
                    .fetch_one(&migrated)
                    .await
                    .unwrap();
            assert_eq!(product_id, None);
            assert_eq!(name, "Agua");
            assert!(sqlx::query("PRAGMA foreign_key_check")
                .fetch_all(&migrated)
                .await
                .unwrap()
                .is_empty());
            migrated.close().await;
            let _ = std::fs::remove_dir_all(&dir);
        });
    }
    #[test]
    fn archived_suppliers_can_be_deleted_keeping_their_movements() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let a = fixture(&db, "Agua").await;
            let supplier = save_catalog_db(&db, catalog("supplier", None, "Proveedor"))
                .await
                .unwrap();
            let mut input = movement(a, "in", 3.0);
            input.supplier_id = Some(supplier);
            create_movement_db(&db, input).await.unwrap();
            // Solo se eliminan proveedores archivados y con la contraseña correcta.
            assert!(delete_supplier_db(&db, supplier, "admin".into())
                .await
                .is_err());
            archive_record_db(&db, "supplier".into(), supplier, false)
                .await
                .unwrap();
            assert!(delete_supplier_db(&db, supplier, "otra".into())
                .await
                .is_err());
            delete_supplier_db(&db, supplier, "admin".into())
                .await
                .unwrap();
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM suppliers")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                0
            );
            let (supplier_id, name): (Option<i64>, Option<String>) =
                sqlx::query_as("SELECT supplier_id,supplier_name FROM movements LIMIT 1")
                    .fetch_one(&db)
                    .await
                    .unwrap();
            assert_eq!(supplier_id, None);
            assert_eq!(name.as_deref(), Some("Proveedor"));
            assert!(sqlx::query("PRAGMA foreign_key_check")
                .fetch_all(&db)
                .await
                .unwrap()
                .is_empty());
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
    fn delete_filter(mode: &str) -> MovementDeleteFilter {
        MovementDeleteFilter {
            mode: mode.into(),
            count: None,
            from: None,
            to: None,
        }
    }
    fn delete_filter_count(mode: &str, count: i64) -> MovementDeleteFilter {
        MovementDeleteFilter {
            mode: mode.into(),
            count: Some(count),
            from: None,
            to: None,
        }
    }
    fn delete_filter_range(
        mode: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> MovementDeleteFilter {
        MovementDeleteFilter {
            mode: mode.into(),
            count: None,
            from: from.map(str::to_string),
            to: to.map(str::to_string),
        }
    }
    #[test]
    fn deleting_movements_requires_password_and_keeps_stock() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let water = fixture(&db, "Agua").await;
            for quantity in [1.0, 2.0, 3.0] {
                create_movement_db(&db, movement(water, "in", quantity))
                    .await
                    .unwrap();
            }
            let coffee = fixture(&db, "Café").await;
            create_movement_db(&db, movement(coffee, "in", 5.0))
                .await
                .unwrap();
            cancel_movement_db(&db, 1, "Compra duplicada".into())
                .await
                .unwrap();

            assert!(
                delete_movements_db(&db, delete_filter_count("recent", 1), "clave".into())
                    .await
                    .is_err()
            );
            assert!(
                delete_movements_db(&db, delete_filter_count("invalid", 1), "admin".into())
                    .await
                    .is_err()
            );
            assert!(
                delete_movements_db(&db, delete_filter_count("recent", 0), "admin".into())
                    .await
                    .is_err()
            );

            assert_eq!(
                delete_movements_db(&db, delete_filter_count("recent", 1), "admin".into())
                    .await
                    .unwrap(),
                1
            );
            let ids: Vec<i64> = sqlx::query_scalar("SELECT id FROM movements ORDER BY id")
                .fetch_all(&db)
                .await
                .unwrap();
            assert_eq!(ids, vec![1, 2, 3]);

            // Cancelled movements are included and stock is never modified.
            assert_eq!(stock(&db, water.0).await, 5.0);
            assert_eq!(stock(&db, coffee.0).await, 5.0);

            assert_eq!(
                delete_movements_db(&db, delete_filter_count("oldest", 10), "admin".into())
                    .await
                    .unwrap(),
                3
            );
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT count(*) FROM movements")
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                0
            );
            assert_eq!(stock(&db, water.0).await, 5.0);
            assert_eq!(
                delete_movements_db(&db, delete_filter_count("recent", 1), "admin".into())
                    .await
                    .unwrap(),
                0
            );
        });
    }

    #[test]
    fn delete_movements_by_date_range_previews_and_deletes() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let water = fixture(&db, "Agua").await;
            for _ in 0..5 {
                create_movement_db(&db, movement(water, "in", 1.0))
                    .await
                    .unwrap();
            }
            for (id, day) in [
                (1, "2026-01-01"),
                (2, "2026-01-02"),
                (3, "2026-01-03"),
                (4, "2026-01-04"),
                (5, "2026-01-05"),
            ] {
                sqlx::query("UPDATE movements SET created_at=? WHERE id=?")
                    .bind(format!("{day} 10:00:00"))
                    .bind(id)
                    .execute(&db)
                    .await
                    .unwrap();
            }

            let onward = delete_filter_range("from", Some("2026-01-03 00:00:00"), None);
            assert_eq!(count_movements_db(&db, &onward).await.unwrap(), 3);
            let backward = delete_filter_range("until", None, Some("2026-01-03 00:00:00"));
            assert_eq!(count_movements_db(&db, &backward).await.unwrap(), 2);
            let interval = delete_filter_range(
                "range",
                Some("2026-01-02 00:00:00"),
                Some("2026-01-05 00:00:00"),
            );
            assert_eq!(count_movements_db(&db, &interval).await.unwrap(), 3);

            assert!(count_movements_db(&db, &delete_filter("from"))
                .await
                .is_err());
            assert!(count_movements_db(&db, &delete_filter("until"))
                .await
                .is_err());
            assert!(count_movements_db(&db, &delete_filter("range"))
                .await
                .is_err());
            let inverted = delete_filter_range(
                "range",
                Some("2026-01-05 00:00:00"),
                Some("2026-01-02 00:00:00"),
            );
            assert!(count_movements_db(&db, &inverted).await.is_err());

            assert_eq!(
                delete_movements_db(&db, interval, "admin".into())
                    .await
                    .unwrap(),
                3
            );
            let remaining: Vec<i64> = sqlx::query_scalar("SELECT id FROM movements ORDER BY id")
                .fetch_all(&db)
                .await
                .unwrap();
            assert_eq!(remaining, vec![1, 5]);
        });
    }

    #[test]
    fn delete_password_can_be_changed_with_the_current_one() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            assert_eq!(delete_password_db(&db).await.unwrap(), "admin");
            assert!(
                change_delete_password_db(&db, "otra".into(), "nueva".into())
                    .await
                    .is_err()
            );
            assert!(change_delete_password_db(&db, "admin".into(), "   ".into())
                .await
                .is_err());
            change_delete_password_db(&db, "admin".into(), "  nueva  ".into())
                .await
                .unwrap();
            assert_eq!(delete_password_db(&db).await.unwrap(), "nueva");

            let water = fixture(&db, "Agua").await;
            create_movement_db(&db, movement(water, "in", 1.0))
                .await
                .unwrap();
            assert!(
                delete_movements_db(&db, delete_filter_count("recent", 1), "admin".into())
                    .await
                    .is_err()
            );
            assert_eq!(
                delete_movements_db(&db, delete_filter_count("recent", 1), "nueva".into())
                    .await
                    .unwrap(),
                1
            );
        });
    }

    #[test]
    fn repeated_initialization_is_idempotent() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
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
            assert_eq!(first_products, 26);
            assert_eq!(first_movements, 52);
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
                25
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
    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("inventory-tests-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
    #[test]
    fn backup_and_restore_roundtrip_preserves_data() {
        tauri::async_runtime::block_on(async {
            let dir = temp_dir("roundtrip");
            let live = dir.join("inventory.db");
            let backup = dir.join("backup.db");
            let safety = dir.join("safety.db");

            let db = connect(&live).await.unwrap();
            let agua = fixture(&db, "Agua").await;
            create_movement_db(&db, movement(agua, "in", 10.0))
                .await
                .unwrap();
            let state = RwLock::new(db);

            backup_database_db(&*state.read().await, &backup)
                .await
                .unwrap();

            {
                let current = state.read().await;
                let cafe = fixture(&current, "Café").await;
                create_movement_db(&current, movement(cafe, "in", 5.0))
                    .await
                    .unwrap();
            }

            restore_database_db(&state, &live, &backup, &safety)
                .await
                .unwrap();

            let pool = state.read().await;
            let names: Vec<String> = sqlx::query_scalar("SELECT name FROM products ORDER BY name")
                .fetch_all(&*pool)
                .await
                .unwrap();
            assert_eq!(names, vec!["Agua".to_string()]);
            assert_eq!(stock(&pool, agua.0).await, 10.0);
            assert!(sqlx::query("PRAGMA foreign_key_check")
                .fetch_all(&*pool)
                .await
                .unwrap()
                .is_empty());
            drop(pool);

            assert!(safety.exists());
            let safety_db = connect(&safety).await.unwrap();
            let safety_count: i64 = sqlx::query_scalar("SELECT count(*) FROM products")
                .fetch_one(&safety_db)
                .await
                .unwrap();
            assert_eq!(safety_count, 2);
            safety_db.close().await;

            drop(state);
            let _ = std::fs::remove_dir_all(&dir);
        });
    }
    #[test]
    fn restore_rejects_invalid_files_and_keeps_data() {
        tauri::async_runtime::block_on(async {
            let dir = temp_dir("invalid");
            let live = dir.join("inventory.db");
            let backup = dir.join("backup.db");
            let safety = dir.join("safety.db");

            let db = connect(&live).await.unwrap();
            fixture(&db, "Agua").await;
            let state = RwLock::new(db);

            std::fs::write(&backup, b"no es sqlite").unwrap();
            let error = restore_database_db(&state, &live, &backup, &safety)
                .await
                .unwrap_err();
            assert_eq!(error.code, "invalid_backup");
            let error = restore_database_db(&state, &live, &live, &safety)
                .await
                .unwrap_err();
            assert_eq!(error.code, "restore_same_file");

            let other = dir.join("other.db");
            let options = SqliteConnectOptions::new()
                .filename(&other)
                .create_if_missing(true);
            let mut conn = SqliteConnection::connect_with(&options).await.unwrap();
            sqlx::raw_sql("CREATE TABLE foo(id INTEGER);")
                .execute(&mut conn)
                .await
                .unwrap();
            conn.close().await.unwrap();
            let error = restore_database_db(&state, &live, &other, &safety)
                .await
                .unwrap_err();
            assert_eq!(error.code, "invalid_backup");
            assert!(!safety.exists());

            let pool = state.read().await;
            let count: i64 = sqlx::query_scalar("SELECT count(*) FROM products")
                .fetch_one(&*pool)
                .await
                .unwrap();
            assert_eq!(count, 1);
            drop(pool);
            drop(state);
            let _ = std::fs::remove_dir_all(&dir);
        });
    }
    #[test]
    fn queries_list_products_with_joined_catalog_data() {
        tauri::async_runtime::block_on(async {
            use crate::queries::list_products_db;

            let db = connect(Path::new(":memory:")).await.unwrap();
            let category = save_catalog_db(&db, catalog("category", None, "Bebidas"))
                .await
                .unwrap();
            let mut input = product(None, "Agua", None);
            input.category_id = Some(category);
            let id = save_product_db(&db, input).await.unwrap();

            let rows = list_products_db(&db, false).await.unwrap();
            let row = rows.iter().find(|row| row.id == id).unwrap();
            assert_eq!(row.base_unit.as_deref(), Some("Unidad"));
            assert_eq!(row.category_name.as_deref(), Some("Bebidas"));
            assert_eq!(row.archived, 0);
            assert!(list_products_db(&db, true).await.unwrap().is_empty());
        });
    }
    #[test]
    fn queries_filter_search_dates_status_and_pagination() {
        tauri::async_runtime::block_on(async {
            use crate::queries::{page_movements_db, MovementFilters, PAGE_SIZE};

            let db = connect(Path::new(":memory:")).await.unwrap();
            let water = fixture(&db, "Agua").await;
            let coffee = fixture(&db, "Café").await;

            let mut plain = movement(water, "in", 10.0);
            plain.notes = Some("sin guion".into());
            create_movement_db(&db, plain).await.unwrap();

            let mut underscore = movement(coffee, "in", 1.0);
            underscore.notes = Some("a_b".into());
            create_movement_db(&db, underscore).await.unwrap();

            let escaped = page_movements_db(
                &db,
                &MovementFilters {
                    search: Some("_".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(escaped.total, 1);
            assert_eq!(escaped.items[0].notes.as_deref(), Some("a_b"));

            let found = page_movements_db(
                &db,
                &MovementFilters {
                    search: Some("café".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(found.total, 1);
            assert_eq!(found.items[0].product_id, Some(coffee.0));

            let by_product = page_movements_db(
                &db,
                &MovementFilters {
                    product: Some(water.0),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(by_product.total, 1);

            let by_type = page_movements_db(
                &db,
                &MovementFilters {
                    movement_type: Some("out".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(by_type.total, 0);

            create_movement_db(&db, movement(water, "out", 1.0))
                .await
                .unwrap();
            let outgoing = page_movements_db(
                &db,
                &MovementFilters {
                    movement_type: Some("out".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(outgoing.total, 1);

            let cancelled = page_movements_db(
                &db,
                &MovementFilters {
                    status: Some("cancelled".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(cancelled.total, 0);

            cancel_movement_db(&db, 3, "Error de carga".into())
                .await
                .unwrap();
            let cancelled = page_movements_db(
                &db,
                &MovementFilters {
                    status: Some("cancelled".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(cancelled.total, 1);
            assert!(cancelled.items[0].cancelled_at.is_some());

            // Se añaden PAGE_SIZE + 3 movimientos: la primera página debe
            // llenarse completa y la segunda conservar los 6 restantes.
            for _ in 0..PAGE_SIZE + 3 {
                create_movement_db(&db, movement(water, "in", 1.0))
                    .await
                    .unwrap();
            }
            let expected_total = PAGE_SIZE + 6;
            let first = page_movements_db(&db, &MovementFilters::default())
                .await
                .unwrap();
            assert_eq!(first.total, expected_total);
            assert_eq!(first.items.len(), PAGE_SIZE as usize);
            let second = page_movements_db(
                &db,
                &MovementFilters {
                    page: Some(1),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(second.total, expected_total);
            assert_eq!(second.items.len(), 6);
        });
    }
    #[test]
    fn queries_find_duplicates_and_read_settings() {
        tauri::async_runtime::block_on(async {
            use crate::queries::{find_duplicate_db, get_settings_db};

            let db = connect(Path::new(":memory:")).await.unwrap();
            let water = fixture(&db, "Agua").await;

            let found = find_duplicate_db(&db, "product", "  agua ", None)
                .await
                .unwrap();
            assert_eq!(found.map(|row| row.id), Some(water.0));
            assert!(find_duplicate_db(&db, "product", "agua", Some(water.0))
                .await
                .unwrap()
                .is_none());

            let supplier = save_catalog_db(&db, catalog("supplier", None, "Proveedor"))
                .await
                .unwrap();
            archive_record_db(&db, "supplier".into(), supplier, false)
                .await
                .unwrap();
            let archived = find_duplicate_db(&db, "supplier", "proveedor", None)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(archived.archived, 1);
            assert!(find_duplicate_db(&db, "unknown", "Proveedor", None)
                .await
                .is_err());

            let defaults = get_settings_db(&db).await.unwrap();
            assert_eq!(defaults.stock_low_threshold, 5.0);
            assert_eq!(defaults.stock_medium_threshold, 20.0);
            assert_eq!(defaults.default_view, "dashboard");

            sqlx::query("UPDATE settings SET value='8' WHERE key='stock_low_threshold'")
                .execute(&db)
                .await
                .unwrap();
            sqlx::query("UPDATE settings SET value='inventada' WHERE key='default_view'")
                .execute(&db)
                .await
                .unwrap();
            let updated = get_settings_db(&db).await.unwrap();
            assert_eq!(updated.stock_low_threshold, 8.0);
            assert_eq!(updated.default_view, "dashboard");
        });
    }
    #[test]
    fn command_errors_expose_stable_codes() {
        tauri::async_runtime::block_on(async {
            let db = connect(Path::new(":memory:")).await.unwrap();
            let water = fixture(&db, "Agua").await;

            let duplicate = save_product_db(&db, product(None, "Agua", None))
                .await
                .unwrap_err();
            assert_eq!(duplicate.code, "duplicate_product");

            let invalid_name = save_product_db(&db, product(None, "   ", None))
                .await
                .unwrap_err();
            assert_eq!(invalid_name.code, "invalid_name");

            let invalid_quantity = create_movement_db(&db, movement(water, "in", 0.0))
                .await
                .unwrap_err();
            assert_eq!(invalid_quantity.code, "invalid_quantity");

            let mut with_supplier = movement(water, "out", 1.0);
            with_supplier.supplier_id = Some(1);
            let supplier = create_movement_db(&db, with_supplier).await.unwrap_err();
            assert_eq!(supplier.code, "supplier_not_allowed");

            let wrong_password =
                delete_movements_db(&db, delete_filter_count("recent", 1), "nope".into())
                    .await
                    .unwrap_err();
            assert_eq!(wrong_password.code, "wrong_password");

            create_movement_db(&db, movement(water, "in", 5.0))
                .await
                .unwrap();
            let archive = archive_record_db(&db, "product".into(), water.0, false)
                .await
                .unwrap_err();
            assert_eq!(archive.code, "archive_requires_zero_stock");

            let missing = archive_record_db(&db, "product".into(), 999, false)
                .await
                .unwrap_err();
            assert_eq!(missing.code, "not_found");

            let bad_kind = archive_record_db(&db, "person".into(), 1, false)
                .await
                .unwrap_err();
            assert_eq!(bad_kind.code, "invalid_operation");

            let invalid_mode = MovementDeleteFilter {
                mode: "nope".into(),
                count: None,
                from: None,
                to: None,
            };
            assert_eq!(
                count_movements_db(&db, &invalid_mode)
                    .await
                    .unwrap_err()
                    .code,
                "invalid_filter"
            );

            let invalid_view = save_settings_db(&db, None, None, Some("inventada".into()))
                .await
                .unwrap_err();
            assert_eq!(invalid_view.code, "invalid_view");

            let invalid_thresholds = save_settings_db(&db, Some(10.0), Some(5.0), None)
                .await
                .unwrap_err();
            assert_eq!(invalid_thresholds.code, "invalid_thresholds");
        });
    }
    #[test]
    fn movement_page_out_of_range_reports_real_total() {
        tauri::async_runtime::block_on(async {
            use crate::queries::{page_movements_db, MovementFilters};

            let db = connect(Path::new(":memory:")).await.unwrap();
            let water = fixture(&db, "Agua").await;
            for _ in 0..3 {
                create_movement_db(&db, movement(water, "in", 1.0))
                    .await
                    .unwrap();
            }

            let page = page_movements_db(
                &db,
                &MovementFilters {
                    page: Some(9),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert!(page.items.is_empty());
            assert_eq!(page.total, 3);
        });
    }
    #[test]
    fn search_escapes_percent_and_backslash() {
        tauri::async_runtime::block_on(async {
            use crate::queries::{escape_like, page_movements_db, MovementFilters};

            assert_eq!(escape_like("100%_\\"), "100\\%\\_\\\\");
            assert_eq!(escape_like("plain"), "plain");

            let db = connect(Path::new(":memory:")).await.unwrap();
            let water = fixture(&db, "Agua").await;
            let mut first = movement(water, "in", 1.0);
            first.notes = Some("descuento 100%".into());
            create_movement_db(&db, first).await.unwrap();
            let mut second = movement(water, "in", 1.0);
            second.notes = Some("sin descuento".into());
            create_movement_db(&db, second).await.unwrap();

            let found = page_movements_db(
                &db,
                &MovementFilters {
                    search: Some("%".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            assert_eq!(found.total, 1);
            assert_eq!(found.items[0].notes.as_deref(), Some("descuento 100%"));
        });
    }
    #[test]
    fn save_settings_persists_valid_values() {
        tauri::async_runtime::block_on(async {
            use crate::queries::get_settings_db;

            let db = connect(Path::new(":memory:")).await.unwrap();
            save_settings_db(&db, Some(3.0), Some(9.0), None)
                .await
                .unwrap();
            save_settings_db(&db, None, None, Some("products".into()))
                .await
                .unwrap();

            let settings = get_settings_db(&db).await.unwrap();
            assert_eq!(settings.stock_low_threshold, 3.0);
            assert_eq!(settings.stock_medium_threshold, 9.0);
            assert_eq!(settings.default_view, "products");
        });
    }
    #[test]
    fn backup_failure_keeps_previous_file_and_cleans_temp() {
        tauri::async_runtime::block_on(async {
            let dir = temp_dir("backup-failure");
            let live = dir.join("inventory.db");
            let db = connect(&live).await.unwrap();
            fixture(&db, "Agua").await;

            let dest = dir.join("dest-as-directory");
            std::fs::create_dir(&dest).unwrap();
            let error = backup_database_db(&db, &dest).await.unwrap_err();
            assert_eq!(error.code, "backup_failed");
            assert!(dest.is_dir());
            assert!(!with_suffix(&dest, ".tmp").exists());

            let missing = dir.join("missing").join("backup.db");
            let error = backup_database_db(&db, &missing).await.unwrap_err();
            assert_eq!(error.code, "backup_folder_missing");

            db.close().await;
            let _ = std::fs::remove_dir_all(&dir);
        });
    }
}
