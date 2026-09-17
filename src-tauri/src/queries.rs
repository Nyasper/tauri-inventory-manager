use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, QueryBuilder, Row, Sqlite};
use tauri::State;

use crate::inventory::{name as normalize_name, CommandError, DbState};

type Db = Pool<Sqlite>;

/// Tamaño de página por defecto de la vista de movimientos.
pub(crate) const DEFAULT_PAGE_SIZE: i64 = 50;
/// Rango permitido para el tamaño de página configurable.
pub(crate) const MIN_PAGE_SIZE: i64 = 10;
pub(crate) const MAX_PAGE_SIZE: i64 = 200;
/// Cota defensiva del índice de página: evita desbordar el offset.
const MAX_PAGE_INDEX: i64 = 10_000_000;

fn query_error(error: sqlx::Error) -> CommandError {
    eprintln!("Query error: {error}");
    CommandError::new(
        "query_failed",
        "No se pudo consultar la información. Inténtalo nuevamente.",
    )
}

/// Escapa los comodines de `LIKE` para que la búsqueda sea literal.
pub(crate) fn escape_like(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        if matches!(character, '\\' | '%' | '_') {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

#[derive(Serialize, FromRow)]
pub struct ProductRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub base_presentation_id: i64,
    pub stock: f64,
    pub archived: i64,
    pub created_at: String,
    pub updated_at: String,
    pub base_unit: Option<String>,
    pub category_name: Option<String>,
    pub category_color: Option<String>,
    pub category_icon: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct PresentationRow {
    pub id: i64,
    pub product_id: i64,
    pub name: String,
    pub factor: f64,
}

#[derive(Serialize, FromRow)]
pub struct SupplierRow {
    pub id: i64,
    pub name: String,
    pub contact: Option<String>,
    pub notes: Option<String>,
    pub archived: i64,
}

#[derive(Serialize, FromRow)]
pub struct CategoryRow {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Serialize)]
pub struct SettingsRow {
    pub stock_low_threshold: f64,
    pub stock_medium_threshold: f64,
    pub default_view: String,
    pub movements_page_size: i64,
}

#[derive(Serialize)]
pub struct DuplicateRow {
    pub id: i64,
    pub archived: i64,
}

#[derive(Serialize, FromRow)]
pub struct MovementRow {
    pub id: i64,
    pub product_id: Option<i64>,
    #[serde(rename = "type")]
    pub movement_type: String,
    pub quantity: f64,
    pub presentation_id: Option<i64>,
    pub presentation_name: String,
    pub product_name: String,
    pub factor: f64,
    pub base_quantity: f64,
    pub supplier_id: Option<i64>,
    pub supplier_name: Option<String>,
    pub notes: Option<String>,
    pub reason: Option<String>,
    pub kind: String,
    pub created_at: String,
    pub cancelled_at: Option<String>,
    pub cancellation_reason: Option<String>,
}

#[derive(Serialize)]
pub struct MovementsPage {
    pub items: Vec<MovementRow>,
    pub total: i64,
}

/// Filtros ya normalizados por el frontend (fechas convertidas a UTC).
#[derive(Deserialize, Default)]
pub struct MovementFilters {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(rename = "type", default)]
    pub movement_type: Option<String>,
    #[serde(default)]
    pub product: Option<i64>,
    #[serde(default)]
    pub supplier: Option<i64>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
}

pub(crate) async fn list_products_db(
    db: &Db,
    archived: bool,
) -> Result<Vec<ProductRow>, CommandError> {
    sqlx::query_as::<_, ProductRow>(
        "SELECT p.id, p.name, p.description, p.category_id, p.base_presentation_id,
                p.stock, p.archived, p.created_at, p.updated_at,
                b.name AS base_unit, c.name AS category_name,
                c.color AS category_color, c.icon AS category_icon
         FROM products p
         LEFT JOIN presentations b ON b.id = p.base_presentation_id
         LEFT JOIN categories c ON c.id = p.category_id
         WHERE p.archived = ?
         ORDER BY p.name COLLATE NOCASE",
    )
    .bind(archived)
    .fetch_all(db)
    .await
    .map_err(query_error)
}

pub(crate) async fn list_presentations_db(
    db: &Db,
    product_id: i64,
) -> Result<Vec<PresentationRow>, CommandError> {
    sqlx::query_as::<_, PresentationRow>(
        "SELECT id, product_id, name, factor
         FROM presentations
         WHERE product_id = ? AND archived = 0
         ORDER BY factor ASC",
    )
    .bind(product_id)
    .fetch_all(db)
    .await
    .map_err(query_error)
}

pub(crate) async fn list_suppliers_db(
    db: &Db,
    archived: bool,
) -> Result<Vec<SupplierRow>, CommandError> {
    sqlx::query_as::<_, SupplierRow>(
        "SELECT id, name, contact, notes, archived
         FROM suppliers
         WHERE archived = ?
         ORDER BY name COLLATE NOCASE",
    )
    .bind(archived)
    .fetch_all(db)
    .await
    .map_err(query_error)
}

pub(crate) async fn list_categories_db(db: &Db) -> Result<Vec<CategoryRow>, CommandError> {
    sqlx::query_as::<_, CategoryRow>(
        "SELECT id, name, color, icon FROM categories ORDER BY name COLLATE NOCASE",
    )
    .fetch_all(db)
    .await
    .map_err(query_error)
}

pub(crate) async fn page_movements_db(
    db: &Db,
    filters: &MovementFilters,
) -> Result<MovementsPage, CommandError> {
    let mut query = QueryBuilder::<Sqlite>::new(
        "SELECT id, product_id, type AS movement_type, quantity, presentation_id,
                presentation_name, product_name, factor, base_quantity, supplier_id,
                supplier_name, notes, reason, kind,
                created_at, cancelled_at, cancellation_reason,
                count(*) OVER () AS total_count
         FROM movements",
    );
    apply_movement_filters(&mut query, filters);

    let page_size = filters
        .page_size
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .clamp(MIN_PAGE_SIZE, MAX_PAGE_SIZE);
    let page = filters.page.unwrap_or(0).clamp(0, MAX_PAGE_INDEX);
    let offset = page.saturating_mul(page_size);
    query
        .push(" ORDER BY created_at DESC, id DESC LIMIT ")
        .push_bind(page_size)
        .push(" OFFSET ")
        .push_bind(offset);

    let rows = query.build().fetch_all(db).await.map_err(query_error)?;
    let mut total = rows
        .first()
        .map(|row| row.get::<i64, _>("total_count"))
        .unwrap_or(0);
    if rows.is_empty() && offset > 0 {
        total = count_movements_where(db, filters).await?;
    }
    let mut items = Vec::with_capacity(rows.len());
    for row in &rows {
        items.push(MovementRow::from_row(row).map_err(query_error)?);
    }
    Ok(MovementsPage { items, total })
}

/// Añade las condiciones de los filtros de movimientos a un `QueryBuilder`.
fn apply_movement_filters(query: &mut QueryBuilder<Sqlite>, filters: &MovementFilters) {
    let mut separator = " WHERE ";

    if let Some(search) = filters
        .search
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let pattern = format!("%{}%", escape_like(search));
        query
            .push(separator)
            .push("(product_name LIKE ")
            .push_bind(pattern.clone())
            .push(" ESCAPE '\\' OR supplier_name LIKE ")
            .push_bind(pattern.clone())
            .push(" ESCAPE '\\' OR notes LIKE ")
            .push_bind(pattern)
            .push(" ESCAPE '\\')");
        separator = " AND ";
    }
    if let Some(kind) = filters
        .movement_type
        .as_deref()
        .filter(|value| matches!(*value, "in" | "out"))
    {
        query
            .push(separator)
            .push("type=")
            .push_bind(kind.to_string());
        separator = " AND ";
    }
    if let Some(product) = filters.product {
        query.push(separator).push("product_id=").push_bind(product);
        separator = " AND ";
    }
    if let Some(supplier) = filters.supplier {
        query
            .push(separator)
            .push("supplier_id=")
            .push_bind(supplier);
        separator = " AND ";
    }
    if let Some(from) = filters.from.as_deref().filter(|value| !value.is_empty()) {
        query
            .push(separator)
            .push("created_at>=")
            .push_bind(from.to_string());
        separator = " AND ";
    }
    if let Some(to) = filters.to.as_deref().filter(|value| !value.is_empty()) {
        query
            .push(separator)
            .push("created_at<")
            .push_bind(to.to_string());
        separator = " AND ";
    }
    match filters.status.as_deref() {
        Some("active") => {
            query.push(separator).push("cancelled_at IS NULL");
        }
        Some("cancelled") => {
            query.push(separator).push("cancelled_at IS NOT NULL");
        }
        _ => {}
    }
}

/// Total de movimientos que cumplen los filtros, sin paginar.
async fn count_movements_where(db: &Db, filters: &MovementFilters) -> Result<i64, CommandError> {
    let mut query = QueryBuilder::<Sqlite>::new("SELECT count(*) FROM movements");
    apply_movement_filters(&mut query, filters);
    let row = query.build().fetch_one(db).await.map_err(query_error)?;
    row.try_get::<i64, _>(0).map_err(query_error)
}

pub(crate) async fn get_settings_db(db: &Db) -> Result<SettingsRow, CommandError> {
    let rows = sqlx::query("SELECT key, value FROM settings")
        .fetch_all(db)
        .await
        .map_err(query_error)?;
    let mut settings = SettingsRow {
        stock_low_threshold: 5.0,
        stock_medium_threshold: 20.0,
        default_view: "dashboard".into(),
        movements_page_size: DEFAULT_PAGE_SIZE,
    };
    for row in rows {
        let key: String = row.try_get("key").map_err(query_error)?;
        let value: String = row.try_get("value").map_err(query_error)?;
        match key.as_str() {
            "stock_low_threshold" => {
                if let Ok(parsed) = value.parse::<f64>() {
                    if parsed.is_finite() && parsed >= 0.0 {
                        settings.stock_low_threshold = parsed;
                    }
                }
            }
            "stock_medium_threshold" => {
                if let Ok(parsed) = value.parse::<f64>() {
                    if parsed.is_finite() && parsed >= 0.0 {
                        settings.stock_medium_threshold = parsed;
                    }
                }
            }
            "default_view" => {
                if matches!(
                    value.as_str(),
                    "dashboard" | "products" | "movements" | "suppliers"
                ) {
                    settings.default_view = value;
                }
            }
            "movements_page_size" => {
                if let Ok(parsed) = value.parse::<i64>() {
                    if (MIN_PAGE_SIZE..=MAX_PAGE_SIZE).contains(&parsed) {
                        settings.movements_page_size = parsed;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(settings)
}

pub(crate) async fn find_duplicate_db(
    db: &Db,
    kind: &str,
    name: &str,
    current_id: Option<i64>,
) -> Result<Option<DuplicateRow>, CommandError> {
    let table = match kind {
        "product" => "products",
        "supplier" => "suppliers",
        _ => {
            return Err(CommandError::new(
                "invalid_kind",
                "Tipo de registro inválido.",
            ))
        }
    };
    let (_, key) = normalize_name(name)?;
    let sql = format!("SELECT id, archived FROM {table} WHERE name_key = ? AND id <> ? LIMIT 1");
    let row = sqlx::query(&sql)
        .bind(key)
        .bind(current_id.unwrap_or(-1))
        .fetch_optional(db)
        .await
        .map_err(query_error)?;
    Ok(row.map(|row| DuplicateRow {
        id: row.get("id"),
        archived: row.get("archived"),
    }))
}

#[tauri::command]
pub async fn list_products(
    state: State<'_, DbState>,
    archived: bool,
) -> Result<Vec<ProductRow>, CommandError> {
    let pool = state.pool().read().await;
    list_products_db(&pool, archived).await
}

#[tauri::command]
pub async fn list_presentations(
    state: State<'_, DbState>,
    product_id: i64,
) -> Result<Vec<PresentationRow>, CommandError> {
    let pool = state.pool().read().await;
    list_presentations_db(&pool, product_id).await
}

#[tauri::command]
pub async fn list_suppliers(
    state: State<'_, DbState>,
    archived: bool,
) -> Result<Vec<SupplierRow>, CommandError> {
    let pool = state.pool().read().await;
    list_suppliers_db(&pool, archived).await
}

#[tauri::command]
pub async fn list_categories(state: State<'_, DbState>) -> Result<Vec<CategoryRow>, CommandError> {
    let pool = state.pool().read().await;
    list_categories_db(&pool).await
}

#[tauri::command]
pub async fn page_movements(
    state: State<'_, DbState>,
    filters: MovementFilters,
) -> Result<MovementsPage, CommandError> {
    let pool = state.pool().read().await;
    page_movements_db(&pool, &filters).await
}

#[tauri::command]
pub async fn get_settings(state: State<'_, DbState>) -> Result<SettingsRow, CommandError> {
    let pool = state.pool().read().await;
    get_settings_db(&pool).await
}

#[tauri::command]
pub async fn find_duplicate(
    state: State<'_, DbState>,
    kind: String,
    name: String,
    current_id: Option<i64>,
) -> Result<Option<DuplicateRow>, CommandError> {
    let pool = state.pool().read().await;
    find_duplicate_db(&pool, &kind, &name, current_id).await
}
