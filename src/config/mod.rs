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