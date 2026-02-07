use crate::types::{DnsConfiguration, DnsTestResult, WindowsVersion};
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use regex::Regex;
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct DnsManager;

impl DnsManager {
    pub fn new() -> Self {
        Self
    }

    #[cfg(windows)]
    pub fn get_windows_version(&self) -> Result<WindowsVersion> {
        debug!("Detecting Windows version...");
        
        let output = Command::new("cmd")
            .args(&["/C", "ver"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .context("Failed to get Windows version")?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        let re = Regex::new(r"Version (\d+)\.(\d+)\.(\d+)").unwrap();
        if let Some(caps) = re.captures(&output_str) {
            let major: u32 = caps.get(1).unwrap().as_str().parse().unwrap_or(10);
            let minor: u32 = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
            let build: u32 = caps.get(3).unwrap().as_str().parse().unwrap_or(0);
            
            // Определяем поддержку DoH (Windows 11 build 22000+)
            let supports_doh = build >= 22000;
            
            // Определяем название версии Windows
            let version_name = Self::get_windows_name(major, minor, build);
            
            info!(
                "🪟 Windows {} detected (Build {}) - DoH support: {}",
                version_name,
                build,
                if supports_doh { "✅" } else { "❌" }
            );
            
            return Ok(WindowsVersion {
                major,
                minor,
                build,
                supports_doh,
            });
        }

        warn!("Could not parse Windows version, using defaults");
        Ok(WindowsVersion {
            major: 10,
            minor: 0,
            build: 0,
            supports_doh: false,
        })
    }

    /// Определяет название версии Windows по номеру сборки
    fn get_windows_name(major: u32, minor: u32, build: u32) -> String {
        match (major, minor, build) {
            // Windows 11
            (10, 0, b) if b >= 22000 => "11".to_string(),
            
            // Windows 10 (различные версии)
            (10, 0, b) if b >= 19041 => "10 (20H1+)".to_string(),
            (10, 0, b) if b >= 18362 => "10 (1903)".to_string(),
            (10, 0, b) if b >= 17763 => "10 (1809)".to_string(),
            (10, 0, b) if b >= 17134 => "10 (1803)".to_string(),
            (10, 0, b) if b >= 16299 => "10 (1709)".to_string(),
            (10, 0, b) if b >= 15063 => "10 (1703)".to_string(),
            (10, 0, b) if b >= 14393 => "10 (1607)".to_string(),
            (10, 0, b) if b >= 10586 => "10 (1511)".to_string(),
            (10, 0, _) => "10".to_string(),
            
            // Windows 8/8.1
            (6, 3, _) => "8.1".to_string(),
            (6, 2, _) => "8".to_string(),
            
            // Windows 7
            (6, 1, _) => "7".to_string(),
            
            // Windows Vista
            (6, 0, _) => "Vista".to_string(),
            
            // Windows XP
            (5, 1, _) => "XP".to_string(),
            (5, 2, _) => "XP 64-bit / Server 2003".to_string(),
            
            // Неизвестная версия
            _ => format!("{}.{}", major, minor),
        }
    }

    #[cfg(not(windows))]
    pub fn get_windows_version(&self) -> Result<WindowsVersion> {
        anyhow::bail!("This function is only supported on Windows")
    }

    #[cfg(windows)]
    pub fn get_current_dns(&self, adapter_name: &str) -> Result<DnsConfiguration> {
        info!("📡 Retrieving DNS configuration for adapter: '{}'", adapter_name);
        
        let output_ipv4 = Command::new("netsh")
            .args(&["interface", "ipv4", "show", "dnsservers", adapter_name])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .context("Failed to execute netsh command for IPv4")?;

        let output_ipv4_str = String::from_utf8_lossy(&output_ipv4.stdout);
        debug!("IPv4 netsh output:\n{}", output_ipv4_str);
        
        let output_ipv6 = Command::new("netsh")
            .args(&["interface", "ipv6", "show", "dnsservers", adapter_name])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .context("Failed to execute netsh command for IPv6")?;

        let output_ipv6_str = String::from_utf8_lossy(&output_ipv6.stdout);
        debug!("IPv6 netsh output:\n{}", output_ipv6_str);
        
        let is_dhcp = output_ipv4_str.contains("DNS servers configured through DHCP") ||
                      output_ipv4_str.contains("DHCP") && !output_ipv4_str.contains("Statically Configured");
        
        let dns_servers_ipv4 = self.parse_dns_servers(&output_ipv4_str);
        let dns_servers_ipv6 = self.parse_dns_servers(&output_ipv6_str);
        
        info!("🔍 Found IPv4 DNS servers: {:?}", dns_servers_ipv4);
        info!("🔍 Found IPv6 DNS servers: {:?}", dns_servers_ipv6);
        info!("⚙️  DHCP configuration: {}", if is_dhcp { "enabled" } else { "disabled" });
        
        let version = self.get_windows_version()?;
        let (doh_enabled, doh_template) = if version.supports_doh {
            self.get_doh_settings(adapter_name)?
        } else {
            (false, None)
        };
        
        if doh_enabled {
            info!("🔒 DNS over HTTPS: enabled");
            if let Some(ref template) = doh_template {
                debug!("DoH template: {}", template);
            }
        } else {
            debug!("DNS over HTTPS: disabled");
        }
        
        Ok(DnsConfiguration {
            primary: dns_servers_ipv4.get(0).cloned(),
            secondary: dns_servers_ipv4.get(1).cloned(),
            primary_ipv6: dns_servers_ipv6.get(0).cloned(),
            secondary_ipv6: dns_servers_ipv6.get(1).cloned(),
            doh_enabled,
            doh_template,
            dot_hostname: None,
            is_dhcp,
            original_primary: dns_servers_ipv4.get(0).cloned(),
            original_secondary: dns_servers_ipv4.get(1).cloned(),
            original_primary_ipv6: dns_servers_ipv6.get(0).cloned(),
            original_secondary_ipv6: dns_servers_ipv6.get(1).cloned(),
        })
    }

    #[cfg(not(windows))]
    pub fn get_current_dns(&self, _adapter_name: &str) -> Result<DnsConfiguration> {
        anyhml::bail!("This function is only supported on Windows")
    }

    fn parse_dns_servers(&self, output: &str) -> Vec<String> {
        let mut servers: Vec<String> = Vec::new();
        let mut in_section = false;

        let ipv4_re = Regex::new(r"\b(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\b").unwrap();
        let ipv6_re = Regex::new(r"\b([0-9a-fA-F:]+(:[0-9a-fA-F:]+)*)\b").unwrap();

        for line in output.lines() {
            let trimmed = line.trim();

            if trimmed.contains("Statically Configured DNS Servers:") ||
               trimmed.contains("статически настроенных DNS-серверов:") {
                in_section = true;

                if let Some(after_colon) = line.splitn(2, ':').nth(1) {
                    let potential_ips: Vec<String> = ipv4_re.find_iter(after_colon)
                        .map(|m| m.as_str().to_string())
                        .chain(ipv6_re.find_iter(after_colon).map(|m| m.as_str().to_string()))
                        .collect();

                    if !potential_ips.is_empty() {
                        servers.extend(potential_ips.into_iter().filter(|ip| Self::is_valid_ip(ip)));
                    }
                }
                continue;
            }

            if in_section && (trimmed.contains("Register with") || trimmed.contains("Регистрация") || trimmed.is_empty()) {
                break;
            }

            if in_section && !trimmed.is_empty() {
                let leading_spaces = line.chars().take_while(|&c| c.is_whitespace()).count();
                if leading_spaces >= 30 || line.starts_with("   ") {
                    let ips: Vec<String> = ipv4_re.find_iter(trimmed)
                        .map(|m| m.as_str().to_string())
                        .chain(ipv6_re.find_iter(trimmed).map(|m| m.as_str().to_string()))
                        .filter(|ip| Self::is_valid_ip(ip) && !servers.contains(ip))
                        .collect();

                    if !ips.is_empty() {
                        servers.extend(ips);
                    }
                }
            }
        }

        if servers.is_empty() {
            debug!("No DNS servers found in static section, scanning entire output");
            let all_ips: Vec<String> = ipv4_re.find_iter(output)
                .map(|m| m.as_str().to_string())
                .chain(ipv6_re.find_iter(output).map(|m| m.as_str().to_string()))
                .filter(|ip| Self::is_valid_ip(ip))
                .collect();

            if !all_ips.is_empty() {
                servers = all_ips;
            }
        }

        debug!("Parsed DNS servers: {:?}", servers);
        servers
    }

    fn is_valid_ip(ip: &str) -> bool {
        if ip.contains(':') {
            return true;
        }

        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 { return false; }

        for &part in &parts {
            if part.parse::<u8>().is_err() {
                return false;
            }
        }

        !ip.starts_with("0.") && 
        !ip.starts_with("127.") && 
        ip != "255.255.255.255" && 
        ip != "0.0.0.0"
    }

    #[cfg(windows)]
    fn get_doh_settings(&self, adapter_name: &str) -> Result<(bool, Option<String>)> {
        debug!("Checking DoH settings for adapter: {}", adapter_name);
        
        let output = Command::new("netsh")
            .args(&["dns", "show", "encryption", &format!("name={}", adapter_name)])
            .creation_flags(CREATE_NO_WINDOW)
            .output();

        match output {
            Ok(out) => {
                let output_str = String::from_utf8_lossy(&out.stdout);
                
                let doh_enabled = output_str.contains("yes") || 
                                 output_str.contains("on") ||
                                 output_str.contains("enabled");
                
                let template = if doh_enabled {
                    output_str.lines()
                        .find(|line| line.contains("https://"))
                        .and_then(|line| {
                            line.split_whitespace()
                                .find(|s| s.starts_with("https://"))
                                .map(String::from)
                        })
                } else {
                    None
                };
                
                Ok((doh_enabled, template))
            }
            Err(e) => {
                debug!("Could not retrieve DoH settings: {}", e);
                Ok((false, None))
            }
        }
    }

    #[cfg(windows)]
    pub fn set_dns(
        &self,
        adapter_name: &str,
        ipv4_servers: Vec<String>,
        ipv6_servers: Vec<String>,
        doh_template: Option<String>,
    ) -> Result<()> {
        info!("🔧 Configuring DNS for adapter: '{}'", adapter_name);
        info!("📋 IPv4 servers: {:?}", ipv4_servers);
        if !ipv6_servers.is_empty() {
            info!("📋 IPv6 servers: {:?}", ipv6_servers);
        }
        if let Some(ref template) = doh_template {
            info!("🔒 DoH template: {}", template);
        }

        // Reset DNS to DHCP before setting new values
        debug!("Resetting IPv4 DNS to DHCP...");
        Command::new("netsh")
            .args(&["interface", "ipv4", "set", "dnsservers", adapter_name, "dhcp"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .context("Failed to reset IPv4 DNS")?;

        // Set Primary DNS
        if let Some(primary) = ipv4_servers.get(0) {
            if !primary.is_empty() {
                info!("✅ Setting IPv4 primary DNS: {}", primary);
                let output = Command::new("netsh")
                    .args(&[
                        "interface", "ipv4", "set", "dnsservers", 
                        adapter_name, "static", primary, "primary"
                    ])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output()
                    .context("Failed to execute netsh for IPv4 primary DNS")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    error!("❌ Failed to set IPv4 primary DNS");
                    error!("stderr: {}", stderr);
                    error!("stdout: {}", stdout);
                    anyhow::bail!("Failed to set IPv4 primary DNS: {}", stderr);
                }
            }
        }

        // Set Secondary DNS
        if let Some(secondary) = ipv4_servers.get(1) {
            if !secondary.is_empty() {
                info!("✅ Setting IPv4 secondary DNS: {}", secondary);
                let output = Command::new("netsh")
                    .args(&[
                        "interface", "ipv4", "add", "dnsservers", 
                        adapter_name, secondary, "index=2"
                    ])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output()
                    .context("Failed to execute netsh for IPv4 secondary DNS")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    error!("❌ Failed to set IPv4 secondary DNS: {}", stderr);
                    anyhow::bail!("Failed to set IPv4 secondary DNS: {}", stderr);
                }
            }
        }

        // IPv6 configuration
        if !ipv6_servers.is_empty() {
            debug!("Resetting IPv6 DNS to DHCP...");
            Command::new("netsh")
                .args(&["interface", "ipv6", "set", "dnsservers", adapter_name, "dhcp"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .context("Failed to reset IPv6 DNS")?;

            if let Some(primary) = ipv6_servers.get(0) {
                if !primary.is_empty() {
                    info!("✅ Setting IPv6 primary DNS: {}", primary);
                    let output = Command::new("netsh")
                        .args(&[
                            "interface", "ipv6", "set", "dnsservers", 
                            adapter_name, "static", primary, "primary"
                        ])
                        .creation_flags(CREATE_NO_WINDOW)
                        .output()
                        .context("Failed to execute netsh for IPv6 primary DNS")?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        error!("❌ Failed to set IPv6 primary DNS: {}", stderr);
                        anyhow::bail!("Failed to set IPv6 primary DNS: {}", stderr);
                    }
                }
            }

            if let Some(secondary) = ipv6_servers.get(1) {
                if !secondary.is_empty() {
                    info!("✅ Setting IPv6 secondary DNS: {}", secondary);
                    let output = Command::new("netsh")
                        .args(&[
                            "interface", "ipv6", "add", "dnsservers", 
                            adapter_name, secondary, "index=2"
                        ])
                        .creation_flags(CREATE_NO_WINDOW)
                        .output()
                        .context("Failed to execute netsh for IPv6 secondary DNS")?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        error!("❌ Failed to set IPv6 secondary DNS: {}", stderr);
                        anyhow::bail!("Failed to set IPv6 secondary DNS: {}", stderr);
                    }
                }
            }
        }

        // Enable DoH if supported
        if let Some(template) = doh_template.as_deref() {
            match self.enable_doh(adapter_name, template) {
                Ok(_) => info!("🔒 DoH enabled successfully"),
                Err(e) => warn!("⚠️  Could not enable DoH: {}", e),
            }
        }

        info!("✨ DNS configuration completed successfully!");
        Ok(())
    }

    #[cfg(not(windows))]
    pub fn set_dns(
        &self,
        _adapter_name: &str,
        _ipv4_servers: Vec<String>,
        _ipv6_servers: Vec<String>,
        _doh_template: Option<String>,
        _dot_hostname: Option<String>,
    ) -> Result<()> {
        anyhow::bail!("This function is only supported on Windows")
    }

    #[cfg(windows)]
    pub fn reset_to_dhcp(&self, adapter_name: &str) -> Result<()> {
        info!("🔄 Restoring DHCP DNS configuration for adapter: '{}'", adapter_name);
        
        let status_ipv4 = Command::new("netsh")
            .args(&["interface", "ipv4", "set", "dnsservers", adapter_name, "dhcp"])
            .creation_flags(CREATE_NO_WINDOW)
            .status()
            .context("Failed to restore IPv4 DNS to DHCP")?;

        if !status_ipv4.success() {
            error!("❌ Failed to restore IPv4 DNS to DHCP");
            anyhow::bail!("Failed to restore IPv4 DNS to DHCP");
        }
        info!("✅ IPv4 DNS restored to DHCP");

        let status_ipv6 = Command::new("netsh")
            .args(&["interface", "ipv6", "set", "dnsservers", adapter_name, "dhcp"])
            .creation_flags(CREATE_NO_WINDOW)
            .status()
            .context("Failed to restore IPv6 DNS to DHCP")?;

        if !status_ipv6.success() {
            error!("❌ Failed to restore IPv6 DNS to DHCP");
            anyhow::bail!("Failed to restore IPv6 DNS to DHCP");
        }
        info!("✅ IPv6 DNS restored to DHCP");

        match self.disable_doh(adapter_name) {
            Ok(_) => debug!("DoH disabled"),
            Err(e) => debug!("Could not disable DoH: {}", e),
        }

        info!("✨ DNS successfully restored to DHCP!");
        Ok(())
    }

    #[cfg(not(windows))]
    pub fn reset_to_dhcp(&self, _adapter_name: &str) -> Result<()> {
        anyhow::bail!("This function is only supported on Windows")
    }

    #[cfg(windows)]
    pub fn enable_doh(&self, adapter_name: &str, doh_template: &str) -> Result<()> {
        debug!("Enabling DoH for adapter: {} with template: {}", adapter_name, doh_template);
        
        let status = Command::new("netsh")
            .args(&["dns", "add", "encryption", &format!("server={}", adapter_name), "dohtemplate=auto", "autoupgrade=yes"])
            .creation_flags(CREATE_NO_WINDOW)
            .status()
            .context("Failed to enable DoH")?;

        if !status.success() {
            warn!("⚠️  DoH might not be supported on this system");
        }

        Ok(())
    }

    #[cfg(not(windows))]
    pub fn enable_doh(&self, _adapter_name: &str, _doh_template: &str) -> Result<()> {
        anyhow::bail!("This function is only supported on Windows")
    }

    #[cfg(windows)]
    pub fn disable_doh(&self, adapter_name: &str) -> Result<()> {
        debug!("Disabling DoH for adapter: {}", adapter_name);
        
        let _ = Command::new("netsh")
            .args(&["dns", "delete", "encryption", &format!("server={}", adapter_name)])
            .creation_flags(CREATE_NO_WINDOW)
            .status();

        Ok(())
    }

    #[cfg(not(windows))]
    pub fn disable_doh(&self, _adapter_name: &str) -> Result<()> {
        anyhow::bail!("This function is only supported on Windows")
    }

    pub async fn test_dns(&self, dns_server: &str) -> Result<DnsTestResult> {
        debug!("🔍 Testing DNS server: {}", dns_server);
        
        let start = Instant::now();
        let query = Self::create_dns_query();
        let socket = UdpSocket::bind("0.0.0.0:0").await
            .context("Failed to bind UDP socket")?;
        let timeout = Duration::from_secs(5);
        let dns_addr = format!("{}:53", dns_server);
        
        match tokio::time::timeout(timeout, socket.send_to(&query, &dns_addr)).await {
            Ok(Ok(_)) => {
                let mut buf = [0u8; 512];
                match tokio::time::timeout(timeout, socket.recv_from(&mut buf)).await {
                    Ok(Ok(_)) => {
                        let latency = start.elapsed();
                        info!("✅ DNS server {} responded in {}ms", dns_server, latency.as_millis());
                        Ok(DnsTestResult {
                            server: dns_server.to_string(),
                            is_available: true,
                            latency_ms: Some(latency.as_millis() as u64),
                            error: None,
                        })
                    }
                    Ok(Err(e)) => {
                        warn!("❌ DNS server {} receive error: {}", dns_server, e);
                        Ok(DnsTestResult {
                            server: dns_server.to_string(),
                            is_available: false,
                            latency_ms: None,
                            error: Some(format!("Receive error: {}", e)),
                        })
                    }
                    Err(_) => {
                        warn!("❌ DNS server {} timeout (no response)", dns_server);
                        Ok(DnsTestResult {
                            server: dns_server.to_string(),
                            is_available: false,
                            latency_ms: None,
                            error: Some("Timeout waiting for response".to_string()),
                        })
                    }
                }
            }
            Ok(Err(e)) => {
                warn!("❌ DNS server {} send error: {}", dns_server, e);
                Ok(DnsTestResult {
                    server: dns_server.to_string(),
                    is_available: false,
                    latency_ms: None,
                    error: Some(format!("Send error: {}", e)),
                })
            }
            Err(_) => {
                warn!("❌ DNS server {} timeout (send)", dns_server);
                Ok(DnsTestResult {
                    server: dns_server.to_string(),
                    is_available: false,
                    latency_ms: None,
                    error: Some("Timeout sending request".to_string()),
                })
            }
        }
    }

    fn create_dns_query() -> Vec<u8> {
        vec![
            0x00, 0x00, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
            0x03, 0x63, 0x6f, 0x6d,
            0x00, 0x00, 0x01, 0x00, 0x01,
        ]
    }

    #[cfg(windows)]
    pub fn flush_dns_cache(&self) -> Result<()> {
        info!("🧹 Flushing DNS cache...");
        
        let status = Command::new("ipconfig")
            .args(&["/flushdns"])
            .creation_flags(CREATE_NO_WINDOW)
            .status()
            .context("Failed to flush DNS cache")?;

        if !status.success() {
            error!("❌ Failed to flush DNS cache");
            anyhow::bail!("Failed to flush DNS cache");
        }

        info!("✨ DNS cache flushed successfully!");
        Ok(())
    }

    #[cfg(not(windows))]
    pub fn flush_dns_cache(&self) -> Result<()> {
        anyhow::bail!("This function is only supported on Windows")
    }
}