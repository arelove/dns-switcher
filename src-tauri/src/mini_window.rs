use tauri::{Manager, Runtime};
use log::info;

/// Toggle mini window visibility
pub async fn toggle<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("mini") {
        if window.is_visible().unwrap_or(false) {
            info!("🙈 Hiding mini window");
            window.hide().map_err(|e| e.to_string())?;
        } else {
            info!("👀 Showing mini window");
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Set mini window always on top
pub fn set_always_on_top<R: Runtime>(
    app: &tauri::AppHandle<R>,
    always_on_top: bool,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("mini") {
        window
            .set_always_on_top(always_on_top)
            .map_err(|e| e.to_string())?;
        info!("📌 Mini window always on top: {}", always_on_top);
    }
    Ok(())
}