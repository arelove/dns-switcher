use crate::types::DnsPreset;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct CustomPresetsManager;

impl CustomPresetsManager {
    /// Получить путь к файлу с кастомными пресетами
    fn get_presets_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?;
        
        let app_dir = config_dir.join("dns-changer");
        
        // Создаём директорию если её нет
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }
        
        Ok(app_dir.join("custom_presets.json"))
    }
    
    /// Загрузить кастомные пресеты
    pub fn load_custom_presets() -> Result<Vec<DnsPreset>> {
        let file_path = Self::get_presets_file_path()?;
        
        if !file_path.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&file_path)
            .context("Failed to read custom presets file")?;
        
        let presets: Vec<DnsPreset> = serde_json::from_str(&content)
            .context("Failed to parse custom presets")?;
        
        Ok(presets)
    }
    
    /// Сохранить кастомные пресеты
    pub fn save_custom_presets(presets: &[DnsPreset]) -> Result<()> {
        let file_path = Self::get_presets_file_path()?;
        
        let content = serde_json::to_string_pretty(presets)
            .context("Failed to serialize custom presets")?;
        
        fs::write(&file_path, content)
            .context("Failed to write custom presets file")?;
        
        Ok(())
    }
    
    /// Добавить новый кастомный пресет
    pub fn add_custom_preset(preset: DnsPreset) -> Result<()> {
        let mut presets = Self::load_custom_presets()?;
        
        // Проверяем, что ID уникален
        if presets.iter().any(|p| p.id == preset.id) {
            anyhow::bail!("Preset with this ID already exists");
        }
        
        presets.push(preset);
        Self::save_custom_presets(&presets)?;
        
        Ok(())
    }
    
    /// Удалить кастомный пресет по ID
    pub fn delete_custom_preset(id: &str) -> Result<()> {
        let mut presets = Self::load_custom_presets()?;
        
        presets.retain(|p| p.id != id);
        Self::save_custom_presets(&presets)?;
        
        Ok(())
    }
    
    /// Обновить кастомный пресет
    pub fn update_custom_preset(preset: DnsPreset) -> Result<()> {
        let mut presets = Self::load_custom_presets()?;
        
        if let Some(existing) = presets.iter_mut().find(|p| p.id == preset.id) {
            *existing = preset;
            Self::save_custom_presets(&presets)?;
            Ok(())
        } else {
            anyhow::bail!("Preset not found");
        }
    }
}