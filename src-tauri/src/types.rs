use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfiguration {
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub primary_ipv6: Option<String>,
    pub secondary_ipv6: Option<String>,
    pub doh_enabled: bool,
    pub doh_template: Option<String>,
    pub dot_hostname: Option<String>,
    pub is_dhcp: bool,
    pub original_primary: Option<String>,
    pub original_secondary: Option<String>,
    pub original_primary_ipv6: Option<String>,
    pub original_secondary_ipv6: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAdapter {
    pub name: String,
    pub description: String,
    pub is_connected: bool,
    pub current_dns: DnsConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsPreset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub servers_ipv4: Vec<String>,
    pub servers_ipv6: Vec<String>,
    pub doh_template: Option<String>,
    pub dot_hostname: Option<String>,
    pub supports_doh: bool,
    pub supports_dot: bool,
    pub icon: String,
    pub color: String,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsTestResult {
    pub server: String,
    pub latency_ms: Option<u64>,
    pub is_available: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsVersion {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub supports_doh: bool, // Windows 11+ (build >= 22000)
}