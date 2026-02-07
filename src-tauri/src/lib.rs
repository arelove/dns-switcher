#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Emitter, Manager, Runtime, State};
use log::{error, info};
use std::sync::{Arc, Mutex};

mod dns;
mod network;
mod presets;
mod types;
mod custom_presets;
mod tray;
mod mini_window;

use dns::DnsManager;
use network::NetworkManager;
use types::{DnsConfiguration, DnsPreset, DnsTestResult, NetworkAdapter, WindowsVersion};
use custom_presets::CustomPresetsManager;

/// Shared state for selected adapter
pub struct AppState {
    pub selected_adapter: Arc<Mutex<Option<String>>>,
}

/// Get selected adapter
#[tauri::command]
async fn get_selected_adapter(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let selected = state.selected_adapter.lock()
        .map_err(|e| e.to_string())?;
    Ok(selected.clone())
}

/// Set selected adapter (shared between main and mini windows)
#[tauri::command]
async fn set_selected_adapter<R: Runtime>(
    adapter_name: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    {
        let mut selected = state.selected_adapter.lock()
            .map_err(|e| e.to_string())?;
        *selected = Some(adapter_name.clone());
    }
    
    // Notify both windows about the change
    let _ = app.emit("adapter-changed", adapter_name);
    
    Ok(())
}

/// Toggle mini window visibility
#[tauri::command]
async fn toggle_mini_window<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    mini_window::toggle(app).await
}

/// Set mini window always on top
#[tauri::command]
async fn set_mini_always_on_top<R: Runtime>(
    app: tauri::AppHandle<R>,
    always_on_top: bool,
) -> Result<(), String> {
    mini_window::set_always_on_top(&app, always_on_top)
}

/// Update tray tooltip with current DNS
#[tauri::command]
async fn update_tray_tooltip<R: Runtime>(
    app: tauri::AppHandle<R>,
    tooltip: String,
) -> Result<(), String> {
    tray::update_tooltip(&app, tooltip)
}

/// Update tray menu with quick presets
#[tauri::command]
async fn update_tray_menu<R: Runtime>(
    app: tauri::AppHandle<R>,
    current_dns: Option<String>,
    quick_preset_names: Vec<String>,
) -> Result<(), String> {
    tray::update_menu(&app, current_dns, Some(quick_preset_names))
}

#[tauri::command]
async fn get_custom_presets() -> Result<Vec<DnsPreset>, String> {
    info!("📋 Loading custom DNS presets...");
    CustomPresetsManager::load_custom_presets()
        .map_err(|e| {
            error!("❌ Failed to load custom presets: {}", e);
            e.to_string()
        })
}

#[tauri::command]
async fn add_custom_preset(preset: DnsPreset) -> Result<(), String> {
    info!("➕ Adding custom preset: {}", preset.name);
    CustomPresetsManager::add_custom_preset(preset)
        .map_err(|e| {
            error!("❌ Failed to add custom preset: {}", e);
            e.to_string()
        })
}

#[tauri::command]
async fn delete_custom_preset(id: String) -> Result<(), String> {
    info!("🗑️ Deleting custom preset: {}", id);
    CustomPresetsManager::delete_custom_preset(&id)
        .map_err(|e| {
            error!("❌ Failed to delete custom preset: {}", e);
            e.to_string()
        })
}

#[tauri::command]
async fn update_custom_preset(preset: DnsPreset) -> Result<(), String> {
    info!("✏️ Updating custom preset: {}", preset.name);
    CustomPresetsManager::update_custom_preset(preset)
        .map_err(|e| {
            error!("❌ Failed to update custom preset: {}", e);
            e.to_string()
        })
}

#[tauri::command]
async fn get_network_adapters() -> Result<Vec<NetworkAdapter>, String> {
    info!("🌐 Fetching network adapters...");
    let network_manager = NetworkManager::new();
    match network_manager.get_adapters().await {
        Ok(adapters) => {
            info!("✅ Found {} network adapter(s)", adapters.len());
            Ok(adapters)
        }
        Err(e) => {
            error!("❌ Failed to get network adapters: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn get_current_dns(adapter_name: String) -> Result<DnsConfiguration, String> {
    info!("📡 Getting DNS configuration for: {}", adapter_name);
    let dns_manager = DnsManager::new();
    dns_manager
        .get_current_dns(&adapter_name)
        .map_err(|e| {
            error!("❌ Failed to get DNS configuration: {}", e);
            e.to_string()
        })
}

#[tauri::command]
async fn set_dns(
    adapter_name: String,
    ipv4_servers: Vec<String>,
    ipv6_servers: Vec<String>,
    doh_template: Option<String>,
    _dot_hostname: Option<String>,
) -> Result<(), String> {
    info!("🔧 Setting DNS for adapter: {}", adapter_name);
    let dns_manager = DnsManager::new();
    dns_manager
        .set_dns(
            &adapter_name,
            ipv4_servers,
            ipv6_servers,
            doh_template,
        )
        .map_err(|e| {
            error!("❌ Failed to set DNS: {}", e);
            e.to_string()
        })
}

#[tauri::command]
async fn reset_dns(adapter_name: String) -> Result<(), String> {
    info!("🔄 Resetting DNS for adapter: {}", adapter_name);
    let dns_manager = DnsManager::new();
    dns_manager.reset_to_dhcp(&adapter_name).map_err(|e| {
        error!("❌ Failed to reset DNS: {}", e);
        e.to_string()
    })
}

#[tauri::command]
async fn test_dns(server: String) -> Result<DnsTestResult, String> {
    info!("🔍 Testing DNS server: {}", server);
    let dns_manager = DnsManager::new();
    dns_manager
        .test_dns(&server)
        .await
        .map_err(|e| {
            error!("❌ DNS test failed: {}", e);
            e.to_string()
        })
}

#[tauri::command]
async fn get_dns_presets() -> Result<Vec<DnsPreset>, String> {
    info!("📋 Loading DNS presets...");
    Ok(presets::get_default_presets())
}

#[tauri::command]
async fn get_preset_by_id(id: String) -> Result<Option<DnsPreset>, String> {
    info!("🔍 Looking for preset with ID: {}", id);
    Ok(presets::get_preset_by_id(&id))
}

#[tauri::command]
async fn flush_dns_cache() -> Result<(), String> {
    info!("🧹 Flushing DNS cache...");
    let dns_manager = DnsManager::new();
    dns_manager.flush_dns_cache().map_err(|e| {
        error!("❌ Failed to flush DNS cache: {}", e);
        e.to_string()
    })
}

#[tauri::command]
async fn get_windows_version() -> Result<WindowsVersion, String> {
    info!("🪟 Getting Windows version...");
    let dns_manager = DnsManager::new();
    dns_manager.get_windows_version().map_err(|e| {
        error!("❌ Failed to get Windows version: {}", e);
        e.to_string()
    })
}

/// Initialize the logger with custom formatting
fn init_logger() {
    use env_logger::Builder;
    use log::LevelFilter;
    use std::io::Write;

    Builder::new()
        .format(|buf, record| {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let level = match record.level() {
                log::Level::Error => "❌ ERROR",
                log::Level::Warn => "⚠️  WARN ",
                log::Level::Info => "ℹ️  INFO ",
                log::Level::Debug => "🐛 DEBUG",
                log::Level::Trace => "🔍 TRACE",
            };
            
            writeln!(
                buf,
                "[{}] {} > {}",
                timestamp,
                level,
                record.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    init_logger();
    
    info!("🚀 DNS Changer v1.0.0 starting...");
    info!("💻 Platform: Windows");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            selected_adapter: Arc::new(Mutex::new(None)),
        })
        .setup(|app| {
            info!("🔧 Setting up application...");
            
            // Setup tray menu event handler
            if let Some(tray) = app.tray_by_id("main-tray") {
                tray.on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "show_main" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                info!("👀 Showing main window");
                            }
                        }
                        "toggle_mini" => {
                            let app_handle = app.clone();
                            tauri::async_runtime::spawn(async move {
                                if let Err(e) = mini_window::toggle(app_handle).await {
                                    error!("❌ Failed to toggle mini window: {}", e);
                                }
                            });
                        }
                        id if id.starts_with("preset_") => {
                            // Extract preset index from "preset_0", "preset_1", etc.
                            if let Some(index_str) = id.strip_prefix("preset_") {
                                if let Ok(index) = index_str.parse::<usize>() {
                                    info!("🎯 Tray preset clicked: index {}", index);
                                    let _ = app.emit("apply-preset-from-tray", index);
                                }
                            }
                        }
                        _ => {}
                    }
                });
            }
            
            info!("✅ Application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_network_adapters,
            get_current_dns,
            set_dns,
            reset_dns,
            test_dns,
            get_dns_presets,
            get_preset_by_id,
            flush_dns_cache,
            get_windows_version,
            get_custom_presets,
            add_custom_preset,
            delete_custom_preset,
            update_custom_preset,
            toggle_mini_window,
            set_mini_always_on_top,
            update_tray_tooltip,
            update_tray_menu,
            get_selected_adapter,
            set_selected_adapter,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}