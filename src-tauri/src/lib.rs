mod inventory;
mod queries;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let path = inventory::db_path(app.handle())?;
            let pool = tauri::async_runtime::block_on(inventory::connect(&path))?;
            app.manage(inventory::DbState::new(pool));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            inventory::save_product,
            inventory::save_catalog,
            inventory::archive_record,
            inventory::create_movement,
            inventory::cancel_movement,
            inventory::save_settings,
            inventory::backup_database,
            inventory::restore_database,
            queries::list_products,
            queries::list_presentations,
            queries::list_suppliers,
            queries::list_categories,
            queries::list_persons,
            queries::page_movements,
            queries::get_settings,
            queries::find_duplicate,
        ])
        .run(tauri::generate_context!())
        .expect("No se pudo iniciar la aplicación");
}
