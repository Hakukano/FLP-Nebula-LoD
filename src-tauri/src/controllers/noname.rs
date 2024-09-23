use tauri::AppHandle;

use crate::models::noname::Noname;

#[tauri::command]
pub fn noname_index(handle: AppHandle) -> Noname {
    Noname::new(&handle)
}
