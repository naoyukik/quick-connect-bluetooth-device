//! 設定管理機能モジュール
//! アプリケーションの設定ファイル（config.toml）の読み書きを管理

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// アプリケーション設定を表す構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 登録済みデバイス一覧
    pub registered_devices: Vec<RegisteredDevice>,
    /// デフォルト接続デバイスのアドレス
    pub default_device: Option<String>,
    /// 自動接続設定
    pub auto_connect: bool,
    /// 接続タイムアウト（秒）
    pub connection_timeout: u32,
}

/// 登録済みデバイス情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredDevice {
    /// デバイス名
    pub name: String,
    /// MACアドレス
    pub address: String,
    /// デバイスタイプ
    pub device_type: String,
    /// 最後に接続した日時
    pub last_connected: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            registered_devices: Vec::new(),
            default_device: None,
            auto_connect: false,
            connection_timeout: 30,
        }
    }
}

impl AppConfig {
    /// 設定ファイルから設定を読み込み
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("設定ファイル {:?} の読み込みに失敗しました", path.as_ref()))?;
        
        let config: AppConfig = toml::from_str(&content)
            .with_context(|| "設定ファイルの解析に失敗しました")?;
        
        Ok(config)
    }

    /// 設定を設定ファイルに保存
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .with_context(|| "設定のシリアライズに失敗しました")?;
        
        fs::write(&path, content)
            .with_context(|| format!("設定ファイル {:?} の書き込みに失敗しました", path.as_ref()))?;
        
        Ok(())
    }

    /// デバイスを登録
    pub fn register_device(&mut self, name: String, address: String, device_type: String) {
        // 既存のデバイスを更新するか、新しいデバイスを追加
        if let Some(device) = self.registered_devices.iter_mut().find(|d| d.address == address) {
            device.name = name;
            device.device_type = device_type;
        } else {
            self.registered_devices.push(RegisteredDevice {
                name,
                address,
                device_type,
                last_connected: None,
            });
        }
    }

    /// デバイスの登録を解除
    pub fn unregister_device(&mut self, address: &str) -> bool {
        let initial_len = self.registered_devices.len();
        self.registered_devices.retain(|d| d.address != address);
        self.registered_devices.len() != initial_len
    }

    /// 登録済みデバイスを取得
    pub fn get_registered_device(&self, address: &str) -> Option<&RegisteredDevice> {
        self.registered_devices.iter().find(|d| d.address == address)
    }

    /// デフォルトデバイスを設定
    pub fn set_default_device(&mut self, address: Option<String>) {
        self.default_device = address;
    }
}

/// 設定ファイルのパスを取得
pub fn get_config_path() -> Result<std::path::PathBuf> {
    let mut path = std::env::current_exe()
        .with_context(|| "実行ファイルのパスを取得できませんでした")?;
    path.pop(); // 実行ファイル名を削除
    path.push("config.toml");
    Ok(path)
}

/// 設定ファイルを初期化（存在しない場合のみ）
pub fn initialize_config_if_needed() -> Result<AppConfig> {
    let config_path = get_config_path()?;
    
    if config_path.exists() {
        AppConfig::load_from_file(&config_path)
    } else {
        let config = AppConfig::default();
        config.save_to_file(&config_path)?;
        println!("設定ファイルを初期化しました: {:?}", config_path);
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn create_temp_config_path() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("test_config_{}.toml", std::process::id()));
        path
    }

    fn cleanup_temp_file(path: &PathBuf) {
        if path.exists() {
            let _ = fs::remove_file(path);
        }
    }

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        
        assert!(config.registered_devices.is_empty());
        assert!(config.default_device.is_none());
        assert!(!config.auto_connect);
        assert_eq!(config.connection_timeout, 30);
    }

    #[test]
    fn test_register_device() {
        let mut config = AppConfig::default();
        
        config.register_device(
            "Test Device".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            "Test Type".to_string()
        );
        
        assert_eq!(config.registered_devices.len(), 1);
        let device = &config.registered_devices[0];
        assert_eq!(device.name, "Test Device");
        assert_eq!(device.address, "AA:BB:CC:DD:EE:FF");
        assert_eq!(device.device_type, "Test Type");
        assert!(device.last_connected.is_none());
    }

    #[test]
    fn test_register_device_update_existing() {
        let mut config = AppConfig::default();
        
        // 最初のデバイス登録
        config.register_device(
            "Old Name".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            "Old Type".to_string()
        );
        
        // 同じアドレスで再登録（更新）
        config.register_device(
            "New Name".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            "New Type".to_string()
        );
        
        // デバイス数は1つのまま
        assert_eq!(config.registered_devices.len(), 1);
        let device = &config.registered_devices[0];
        assert_eq!(device.name, "New Name");
        assert_eq!(device.device_type, "New Type");
    }

    #[test]
    fn test_unregister_device() {
        let mut config = AppConfig::default();
        
        config.register_device(
            "Test Device".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            "Test Type".to_string()
        );
        
        // 存在するデバイスの削除
        assert!(config.unregister_device("AA:BB:CC:DD:EE:FF"));
        assert!(config.registered_devices.is_empty());
        
        // 存在しないデバイスの削除
        assert!(!config.unregister_device("11:22:33:44:55:66"));
    }

    #[test]
    fn test_get_registered_device() {
        let mut config = AppConfig::default();
        
        config.register_device(
            "Test Device".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            "Test Type".to_string()
        );
        
        // 存在するデバイスの取得
        let device = config.get_registered_device("AA:BB:CC:DD:EE:FF");
        assert!(device.is_some());
        assert_eq!(device.unwrap().name, "Test Device");
        
        // 存在しないデバイスの取得
        let device = config.get_registered_device("11:22:33:44:55:66");
        assert!(device.is_none());
    }

    #[test]
    fn test_set_default_device() {
        let mut config = AppConfig::default();
        
        // デフォルトデバイスの設定
        config.set_default_device(Some("AA:BB:CC:DD:EE:FF".to_string()));
        assert_eq!(config.default_device, Some("AA:BB:CC:DD:EE:FF".to_string()));
        
        // デフォルトデバイスのクリア
        config.set_default_device(None);
        assert!(config.default_device.is_none());
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_path = create_temp_config_path();
        
        // 設定を作成して保存
        let mut config = AppConfig::default();
        config.register_device(
            "Test Device".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            "Test Type".to_string()
        );
        config.set_default_device(Some("AA:BB:CC:DD:EE:FF".to_string()));
        config.auto_connect = true;
        config.connection_timeout = 60;
        
        let save_result = config.save_to_file(&temp_path);
        assert!(save_result.is_ok());
        
        // 設定を読み込み
        let loaded_config = AppConfig::load_from_file(&temp_path);
        assert!(loaded_config.is_ok());
        
        let loaded_config = loaded_config.unwrap();
        assert_eq!(loaded_config.registered_devices.len(), 1);
        assert_eq!(loaded_config.default_device, Some("AA:BB:CC:DD:EE:FF".to_string()));
        assert!(loaded_config.auto_connect);
        assert_eq!(loaded_config.connection_timeout, 60);
        
        cleanup_temp_file(&temp_path);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let nonexistent_path = PathBuf::from("nonexistent_config.toml");
        let result = AppConfig::load_from_file(&nonexistent_path);
        assert!(result.is_err());
    }
}