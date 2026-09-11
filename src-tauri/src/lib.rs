mod inventory;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            let path = app.path().app_config_dir()?.join("inventory.db");
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
        ])
        .run(tauri::generate_context!())
        .expect("No se pudo iniciar la aplicación");
}
