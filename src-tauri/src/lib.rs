mod inventory;
mod queries;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let path = inventory::db_path(app.handle()).map_err(|error| error.message)?;
            let pool = tauri::async_runtime::block_on(inventory::connect(&path))?;
            app.manage(inventory::DbState::new(pool));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            inventory::save_product,
            inventory::save_catalog,
            inventory::archive_record,
            inventory::delete_product,
            inventory::delete_supplier,
            inventory::create_movement,
            inventory::cancel_movement,
            inventory::delete_movements,
            inventory::count_movements,
            inventory::change_delete_password,
            inventory::save_settings,
            inventory::backup_database,
            inventory::restore_database,
            queries::list_products,
            queries::list_presentations,
            queries::list_suppliers,
            queries::list_categories,
            queries::page_movements,
            queries::get_settings,
            queries::find_duplicate,
        ])
        .run(tauri::generate_context!())
        .expect("No se pudo iniciar la aplicación");
}
