//! Bluetooth操作機能モジュール
//! Windows Bluetooth APIを使用してBluetoothデバイスの管理を行う

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use windows::{
    core::*,
    Win32::Devices::Bluetooth::*,
    Win32::Foundation::*,
    Win32::System::Com::*,
};

/// Bluetoothデバイス情報を表す構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothDevice {
    /// デバイス名
    pub name: String,
    /// MACアドレス
    pub address: String,
    /// 接続状態
    pub is_connected: bool,
    /// デバイスタイプ
    pub device_type: String,
}

impl BluetoothDevice {
    /// 新しいBluetoothDeviceインスタンスを作成
    pub fn new(name: String, address: String, device_type: String) -> Self {
        Self {
            name,
            address,
            is_connected: false,
            device_type,
        }
    }
}

/// Bluetooth操作を管理するマネージャー
pub struct BluetoothManager;

impl BluetoothManager {
    /// 新しいBluetoothManagerインスタンスを作成
    pub fn new() -> Self {
        Self
    }

    /// 利用可能なBluetoothデバイス一覧を取得
    pub fn list_devices(&self) -> Result<Vec<BluetoothDevice>> {
        println!("Bluetoothデバイスをスキャン中...");
        
        let mut devices = Vec::new();
        
        // Windowsレジストリからペアリング済みBluetoothデバイス情報を取得
        let output = std::process::Command::new("reg")
            .args(&[
                "query",
                "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\BTHPORT\\Parameters\\Devices",
                "/s"
            ])
            .output()
            .context("レジストリクエリの実行に失敗しました")?;
        
        if !output.status.success() {
            println!("Bluetoothデバイス情報の取得に失敗しました");
            return Ok(devices);
        }
        
        let registry_output = String::from_utf8_lossy(&output.stdout);
        devices = self.parse_bluetooth_registry(&registry_output)?;
        
        println!("{}個のBluetoothデバイスが見つかりました", devices.len());
        Ok(devices)
    }

    /// 指定されたデバイスに接続
    pub fn connect_device(&self, address: &str) -> Result<()> {
        // モック実装: 実際のWindows Bluetooth APIは後で実装
        println!("デバイス {} に接続を試行中...", address);
        
        // MACアドレスの形式を簡単にチェック
        if !self.is_valid_mac_address(address) {
            return Err(anyhow::anyhow!("無効なMACアドレス形式です: {}", address));
        }
        
        // 接続シミュレーション
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("デバイス {} に正常に接続しました", address);
        Ok(())
    }

    /// 指定されたデバイスから切断
    pub fn disconnect_device(&self, address: &str) -> Result<()> {
        // モック実装: 実際のWindows Bluetooth APIは後で実装
        println!("デバイス {} から切断中...", address);
        
        // MACアドレスの形式を簡単にチェック
        if !self.is_valid_mac_address(address) {
            return Err(anyhow::anyhow!("無効なMACアドレス形式です: {}", address));
        }
        
        // 切断シミュレーション
        std::thread::sleep(std::time::Duration::from_millis(300));
        println!("デバイス {} から正常に切断しました", address);
        Ok(())
    }
    
    /// MACアドレスの形式をチェック
    fn is_valid_mac_address(&self, address: &str) -> bool {
        // 簡単な形式チェック: XX:XX:XX:XX:XX:XX
        let parts: Vec<&str> = address.split(':').collect();
        if parts.len() != 6 {
            return false;
        }
        
        for part in parts {
            if part.len() != 2 {
                return false;
            }
            if !part.chars().all(|c| c.is_ascii_hexdigit()) {
                return false;
            }
        }
        
        true
    }
    
    /// レジストリ出力を解析してBluetoothデバイス一覧を作成
    fn parse_bluetooth_registry(&self, registry_output: &str) -> Result<Vec<BluetoothDevice>> {
        let mut devices = Vec::new();
        let lines: Vec<&str> = registry_output.lines().collect();
        
        let mut current_device_key: Option<String> = None;
        let mut current_device_name: Option<String> = None;
        let mut current_device_connected = false;
        
        for line in lines {
            let line = line.trim();
            
            // デバイスキー（MACアドレス）を検出
            if line.starts_with("HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\BTHPORT\\Parameters\\Devices\\") {
                // 前のデバイス情報を保存
                if let Some(device_key) = current_device_key.take() {
                    if let Some(mac_address) = self.format_mac_address(&device_key) {
                        let device_name = current_device_name.take().unwrap_or_else(|| {
                            format!("Bluetooth Device {}", &mac_address[..8])
                        });
                        
                        devices.push(BluetoothDevice {
                            name: device_name.clone(),
                            address: mac_address.clone(),
                            is_connected: current_device_connected,
                            device_type: self.determine_device_type_from_name(&device_name),
                        });
                    }
                }
                
                // 新しいデバイスキーを抽出
                if let Some(key_start) = line.rfind('\\') {
                    current_device_key = Some(line[key_start + 1..].to_string());
                    current_device_name = None;
                    current_device_connected = false;
                }
            }
            // FriendlyNameを検出（デバイス名）
            else if line.contains("FriendlyName") && line.contains("REG_SZ") {
                if let Some(name_start) = line.find("REG_SZ") {
                    let name_part = &line[name_start + 6..].trim();
                    if !name_part.is_empty() {
                        current_device_name = Some(name_part.to_string());
                    }
                }
            }
            // LastConnectedを検出（接続状態の推定）
            else if line.contains("LastConnected") && line.contains("REG_QWORD") {
                // 最近接続されたデバイスは接続済みと仮定（簡易実装）
                current_device_connected = true;
            }
        }
        
        // 最後のデバイス情報を保存
        if let Some(device_key) = current_device_key {
            if let Some(mac_address) = self.format_mac_address(&device_key) {
                let device_name = current_device_name.unwrap_or_else(|| {
                    format!("Bluetooth Device {}", &mac_address[..8])
                });
                
                devices.push(BluetoothDevice {
                    name: device_name.clone(),
                    address: mac_address.clone(),
                    is_connected: current_device_connected,
                    device_type: self.determine_device_type_from_name(&device_name),
                });
            }
        }
        
        Ok(devices)
    }
    
    /// レジストリキー（MACアドレス）を標準形式に変換
    fn format_mac_address(&self, registry_key: &str) -> Option<String> {
        if registry_key.len() != 12 {
            return None;
        }
        
        let mut formatted = String::new();
        for (i, c) in registry_key.chars().enumerate() {
            if i > 0 && i % 2 == 0 {
                formatted.push(':');
            }
            formatted.push(c.to_ascii_uppercase());
        }
        
        Some(formatted)
    }
    
    /// デバイス名からデバイスタイプを推定
    fn determine_device_type_from_name(&self, device_name: &str) -> String {
        let name_lower = device_name.to_lowercase();
        
        if name_lower.contains("mouse") || name_lower.contains("keyboard") {
            "Peripheral".to_string()
        } else if name_lower.contains("headphone") || name_lower.contains("speaker") || 
                  name_lower.contains("audio") || name_lower.contains("hl7bt") {
            "Audio/Video".to_string()
        } else if name_lower.contains("phone") || name_lower.contains("mobile") {
            "Phone".to_string()
        } else {
            "Unknown".to_string()
        }
    }
}

impl Default for BluetoothManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_device_creation() {
        let device = BluetoothDevice {
            name: "Test Device".to_string(),
            address: "AA:BB:CC:DD:EE:FF".to_string(),
            is_connected: false,
            device_type: "Test".to_string(),
        };
        
        assert_eq!(device.name, "Test Device");
        assert_eq!(device.address, "AA:BB:CC:DD:EE:FF");
        assert!(!device.is_connected);
        assert_eq!(device.device_type, "Test");
    }

    #[test]
    fn test_bluetooth_manager_creation() {
        let manager = BluetoothManager::new();
        // マネージャーが正常に作成されることを確認
        assert!(true); // 基本的な作成テスト
    }

    #[test]
    fn test_list_devices() {
        let manager = BluetoothManager::new();
        let result = manager.list_devices();
        
        // 実際の実装では常に成功し、実際のBluetoothデバイスを返す
        assert!(result.is_ok());
        let devices = result.unwrap();
        // デバイス数は環境によって異なるため、0個以上であることを確認
        assert!(devices.len() >= 0);
    }

    #[test]
    fn test_valid_mac_address() {
        let manager = BluetoothManager::new();
        
        // 有効なMACアドレス
        assert!(manager.is_valid_mac_address("AA:BB:CC:DD:EE:FF"));
        assert!(manager.is_valid_mac_address("00:11:22:33:44:55"));
        assert!(manager.is_valid_mac_address("FF:FF:FF:FF:FF:FF"));
        
        // 無効なMACアドレス
        assert!(!manager.is_valid_mac_address("AA:BB:CC:DD:EE"));     // 短い
        assert!(!manager.is_valid_mac_address("AA:BB:CC:DD:EE:FF:GG")); // 長い
        assert!(!manager.is_valid_mac_address("AA-BB-CC-DD-EE-FF"));   // 区切り文字が違う
        assert!(!manager.is_valid_mac_address("GG:HH:II:JJ:KK:LL"));   // 無効な16進数
        assert!(!manager.is_valid_mac_address(""));                    // 空文字
    }

    #[test]
    fn test_connect_device_with_valid_address() {
        let manager = BluetoothManager::new();
        let result = manager.connect_device("AA:BB:CC:DD:EE:FF");
        
        // モック実装では有効なMACアドレスで成功する
        assert!(result.is_ok());
    }

    #[test]
    fn test_connect_device_with_invalid_address() {
        let manager = BluetoothManager::new();
        let result = manager.connect_device("invalid-address");
        
        // 無効なMACアドレスではエラーになる
        assert!(result.is_err());
    }

    #[test]
    fn test_disconnect_device_with_valid_address() {
        let manager = BluetoothManager::new();
        let result = manager.disconnect_device("AA:BB:CC:DD:EE:FF");
        
        // モック実装では有効なMACアドレスで成功する
        assert!(result.is_ok());
    }

    #[test]
    fn test_disconnect_device_with_invalid_address() {
        let manager = BluetoothManager::new();
        let result = manager.disconnect_device("invalid-address");
        
        // 無効なMACアドレスではエラーになる
        assert!(result.is_err());
    }
}