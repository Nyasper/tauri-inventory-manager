mod inventory;
mod queries;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(inventory::StartupNotice::default());
            let path = inventory::db_path(app.handle()).map_err(|error| error.message)?;
            let (pool, notice) = inventory::open_with_recovery(&path)?;
            if let Some(message) = notice {
                app.state::<inventory::StartupNotice>().set(message);
            }
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
            inventory::update_movement_supplier,
            inventory::cancel_movement,
            inventory::delete_movements,
            inventory::count_movements,
            inventory::change_delete_password,
            inventory::save_settings,
            inventory::backup_database,
            inventory::restore_database,
            inventory::take_startup_notice,
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
