mod inventory;
mod queries;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
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
            queries::list_movements_page,
            queries::get_settings,
            queries::find_duplicate,
        ])
        .run(tauri::generate_context!())
        .expect("No se pudo iniciar la aplicación");
}
