use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu}, 
    Runtime
};

/// Build tray menu with quick presets
pub fn build_tray_menu<R: Runtime>(
    app: &tauri::AppHandle<R>,
    current_dns: Option<String>,
    quick_preset_names: Option<Vec<String>>,
) -> Result<Menu<R>, tauri::Error> {
    let menu = Menu::new(app)?;

    // Current DNS info
    if let Some(dns) = current_dns {
        let current_item = MenuItem::with_id(
            app,
            "current_dns",
            format!("Current: {}", dns),
            false,
            None::<&str>,
        )?;
        menu.append(&current_item)?;
        menu.append(&PredefinedMenuItem::separator(app)?)?;
    }

    // Quick Presets submenu - use actual quick presets from localStorage
    let preset_names = quick_preset_names.unwrap_or_else(|| {
        // Default presets if none provided
        vec![
            "Google DNS".to_string(),
            "Cloudflare".to_string(),
            "Quad9".to_string(),
        ]
    });

    if !preset_names.is_empty() {
        let mut preset_items: Vec<Box<dyn tauri::menu::IsMenuItem<R>>> = Vec::new();
        
        for (idx, name) in preset_names.iter().enumerate() {
            let item = MenuItem::with_id(
                app,
                &format!("preset_{}", idx),
                name,
                true,
                None::<&str>,
            )?;
            preset_items.push(Box::new(item));
        }

        // Convert Vec<Box<dyn IsMenuItem>> to Vec<&dyn IsMenuItem>
        let preset_refs: Vec<&dyn tauri::menu::IsMenuItem<R>> = preset_items
            .iter()
            .map(|item| item.as_ref() as &dyn tauri::menu::IsMenuItem<R>)
            .collect();

        let presets_menu = Submenu::with_id_and_items(
            app,
            "quick_presets",
            "Quick Presets",
            true,
            &preset_refs,
        )?;
        menu.append(&presets_menu)?;
        menu.append(&PredefinedMenuItem::separator(app)?)?;
    }

    // Main window
    menu.append(&MenuItem::with_id(
        app,
        "show_main",
        "Show Main Window",
        true,
        None::<&str>,
    )?)?;

    // Mini window toggle
    menu.append(&MenuItem::with_id(
        app,
        "toggle_mini",
        "Toggle Mini Window",
        true,
        Some("Ctrl+M"),
    )?)?;

    menu.append(&PredefinedMenuItem::separator(app)?)?;

    // Quit
    menu.append(&PredefinedMenuItem::quit(app, Some("Quit"))?)?;

    Ok(menu)
}

/// Update tray tooltip
pub fn update_tooltip<R: Runtime>(
    app: &tauri::AppHandle<R>,
    tooltip: String,
) -> Result<(), String> {
    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_tooltip(Some(tooltip))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Update tray menu with current DNS and quick presets
pub fn update_menu<R: Runtime>(
    app: &tauri::AppHandle<R>,
    current_dns: Option<String>,
    quick_preset_names: Option<Vec<String>>,
) -> Result<(), String> {
    if let Some(tray) = app.tray_by_id("main-tray") {
        let menu = build_tray_menu(app, current_dns, quick_preset_names)
            .map_err(|e| e.to_string())?;
        tray.set_menu(Some(menu))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}