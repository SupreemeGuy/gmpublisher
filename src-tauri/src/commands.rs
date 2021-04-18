use std::path::{Path, PathBuf};

use tauri::{InvokeMessage, Params};

pub fn invoke_handler<M>() -> impl Fn(InvokeMessage<M>) + Send + Sync + 'static
where
	M: Params
{
	tauri::generate_handler![
		free_caches,
		check_dir,
		check_file,

		crate::transactions::cancel_transaction,

		crate::appdata::update_settings,
		crate::appdata::validate_gmod,
		crate::appdata::window_resized,

		crate::game_addons::browse_installed_addons,
		crate::game_addons::get_installed_addon,

		crate::steam::is_steam_connected,
		crate::steam::get_steam_user,
		crate::steam::workshop::browse_my_workshop,

		crate::addon_size_analyzer::addon_size_analyzer,

		crate::content_generator::get_content_generator_manifests,
		crate::content_generator::update_content_generator_manifest,
	]
}

#[tauri::command]
pub fn free_caches() {
	crate::game_addons::free_caches();
	crate::steam::workshop::free_caches();
}

#[tauri::command]
pub fn check_file(path: PathBuf, extension: Option<String>) -> bool {
	path.is_absolute() && path.exists() && path.is_file() && match extension {
		Some(extension) => {
			if let Some(picked_extension) = path.extension() {
				extension.eq_ignore_ascii_case(&picked_extension.to_string_lossy())
			} else {
				false
			}
		},
		None => true
	}
}

#[tauri::command]
pub fn check_dir(path: PathBuf) -> bool {
	path.is_absolute() && path.exists() && path.is_dir()
}
