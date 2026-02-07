use crate::types::{DnsConfiguration, NetworkAdapter};
use crate::dns::DnsManager;
use anyhow::{Context, Result};
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct NetworkManager;

impl NetworkManager {
    pub fn new() -> Self {
        Self
    }

    #[cfg(windows)]
    pub async fn get_adapters(&self) -> Result<Vec<NetworkAdapter>> {
        let output = Command::new("netsh")
            .args(&["interface", "show", "interface"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .context("Failed to execute netsh command")?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        // println!("Network interfaces output:\n{}", output_str);
        
        let adapters = self.parse_adapters(&output_str)?;
        
        // Получаем DNS для каждого адаптера
        let dns_manager = DnsManager::new();
        let mut result = Vec::new();
        
        for adapter in adapters {
            let current_dns = dns_manager
                .get_current_dns(&adapter.name)
                .unwrap_or_else(|_| DnsConfiguration {
                    primary: None,
                    secondary: None,
                    primary_ipv6: None,
                    secondary_ipv6: None,
                    doh_enabled: false,
                    doh_template: None,
                    dot_hostname: None,
                    is_dhcp: true,
                    original_primary: None,
                    original_secondary: None,
                    original_primary_ipv6: None,
                    original_secondary_ipv6: None,
                });
            
            result.push(NetworkAdapter {
                name: adapter.name,
                description: adapter.description,
                is_connected: adapter.is_connected,
                current_dns,
            });
        }
        
        Ok(result)
    }

    #[cfg(not(windows))]
    pub async fn get_adapters(&self) -> Result<Vec<NetworkAdapter>> {
        anyhow::bail!("This function is only supported on Windows")
    }

    #[cfg(windows)]
    fn parse_adapters(&self, output: &str) -> Result<Vec<TempAdapter>> {
        let mut adapters = Vec::new();
        
        // Парсим вывод netsh interface show interface
        // Формат: Admin State    State          Type             Interface Name
        // Пример: Enabled        Connected      Dedicated        Wi-Fi
        
        let lines: Vec<&str> = output.lines().collect();
        
        for line in lines.iter().skip(3) { // Пропускаем заголовки
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // Разбиваем строку на части (учитываем множественные пробелы)
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() < 4 {
                continue;
            }
            
            let admin_state = parts[0];
            let state = parts[1];
            let _interface_type = parts[2];
            let name = parts[3..].join(" ");
            
            // Определяем статус подключения
            let is_connected = state.eq_ignore_ascii_case("Connected") || 
                              state.eq_ignore_ascii_case("Подключено");
            
            let is_enabled = admin_state.eq_ignore_ascii_case("Enabled") || 
                            admin_state.eq_ignore_ascii_case("Включено");
            
            // Фильтруем ненужные адаптеры
            if name.contains("Loopback") || 
               name.contains("vEthernet") ||
               name.contains("VPN") ||
               name.contains("OpenVPN") {
                continue;
            }
            
            adapters.push(TempAdapter {
                name: name.clone(),
                description: name.clone(),
                is_connected: is_connected && is_enabled,
            });
        }
        
        println!("Parsed {} adapters", adapters.len());
        for adapter in &adapters {
            println!("  - {}: connected={}", 
                    adapter.name, adapter.is_connected);
        }
        
        Ok(adapters)
    }
}

// Временная структура для парсинга
#[derive(Debug, Clone)]
struct TempAdapter {
    name: String,
    description: String,
    is_connected: bool,
}