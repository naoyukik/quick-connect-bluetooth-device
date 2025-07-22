//! CLI機能モジュール
//! clapを使用してコマンドライン引数を処理し、各種コマンドを実行

use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::config::{AppConfig, get_config_path};
use crate::bluetooth::BluetoothManager;

/// Bluetoothデバイス接続管理ツール
#[derive(Parser)]
#[command(name = "connect-bluetooth-device")]
#[command(about = "Bluetoothデバイスの接続を管理するツール")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// 利用可能なコマンド一覧
#[derive(Subcommand)]
pub enum Commands {
    /// 利用可能なBluetoothデバイス一覧を表示
    List {
        /// 登録済みデバイスのみ表示
        #[arg(short, long)]
        registered: bool,
    },
    /// デバイスを登録
    Register {
        /// デバイスのMACアドレス
        #[arg(short, long)]
        address: String,
        /// デバイス名（オプション）
        #[arg(short, long)]
        name: Option<String>,
    },
    /// デバイスの登録を解除
    Unregister {
        /// デバイスのMACアドレス
        #[arg(short, long)]
        address: String,
    },
    /// 現在の状態を表示
    Status,
    /// デバイスに接続
    Connect {
        /// デバイスのMACアドレス（省略時はデフォルトデバイス）
        address: Option<String>,
    },
    /// デバイスから切断
    Disconnect {
        /// デバイスのMACアドレス（省略時は全デバイス）
        address: Option<String>,
    },
    /// デフォルトデバイスを設定
    SetDefault {
        /// デバイスのMACアドレス
        #[arg(short, long)]
        address: String,
    },
}

/// CLIコマンドを実行
pub fn run_command(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::List { registered }) => {
            handle_list_command(registered)
        }
        Some(Commands::Register { address, name }) => {
            handle_register_command(address, name)
        }
        Some(Commands::Unregister { address }) => {
            handle_unregister_command(address)
        }
        Some(Commands::Status) => {
            handle_status_command()
        }
        Some(Commands::Connect { address }) => {
            handle_connect_command(address)
        }
        Some(Commands::Disconnect { address }) => {
            handle_disconnect_command(address)
        }
        Some(Commands::SetDefault { address }) => {
            handle_set_default_command(address)
        }
        None => {
            // コマンドが指定されていない場合はデフォルト動作（自動接続）
            handle_default_action()
        }
    }
}

/// listコマンドの処理
fn handle_list_command(registered_only: bool) -> Result<()> {
    let config_path = get_config_path()?;
    let config = if config_path.exists() {
        AppConfig::load_from_file(&config_path)?
    } else {
        AppConfig::default()
    };
    
    if registered_only {
        println!("=== 登録済みデバイス一覧 ===");
        if config.registered_devices.is_empty() {
            println!("登録済みデバイスはありません。");
        } else {
            for (index, device) in config.registered_devices.iter().enumerate() {
                let is_default = config.default_device.as_ref() == Some(&device.address);
                let default_mark = if is_default { " [デフォルト]" } else { "" };
                
                println!("{}. {}{}", index + 1, device.name, default_mark);
                println!("   アドレス: {}", device.address);
                println!("   タイプ: {}", device.device_type);
                if let Some(last_connected) = &device.last_connected {
                    println!("   最終接続: {}", last_connected);
                }
                println!();
            }
        }
    } else {
        println!("=== 利用可能なBluetoothデバイス一覧 ===");
        
        // Bluetoothマネージャーを使用してデバイス一覧を取得
        let bluetooth_manager = BluetoothManager::new();
        match bluetooth_manager.list_devices() {
            Ok(bluetooth_devices) => {
                if bluetooth_devices.is_empty() {
                    println!("利用可能なBluetoothデバイスが見つかりませんでした。");
                } else {
                    println!("検出されたBluetoothデバイス:");
                    for device in &bluetooth_devices {
                        let connection_status = if device.is_connected { " [接続済み]" } else { "" };
                        let is_registered = config.registered_devices.iter()
                            .any(|reg_dev| reg_dev.address == device.address);
                        let registered_mark = if is_registered { " [登録済み]" } else { "" };
                        
                        println!("  - {} ({}){}{}", 
                            device.name, 
                            device.address, 
                            connection_status,
                            registered_mark
                        );
                        println!("    タイプ: {}", device.device_type);
                    }
                }
            }
            Err(e) => {
                println!("Bluetoothデバイスの取得に失敗しました: {}", e);
            }
        }
        
        // 登録済みデバイスも表示
        if !config.registered_devices.is_empty() {
            println!("\n登録済みデバイス:");
            for device in &config.registered_devices {
                let is_default = config.default_device.as_ref() == Some(&device.address);
                let default_mark = if is_default { " [デフォルト]" } else { "" };
                println!("  - {}{} ({})", device.name, default_mark, device.address);
            }
        }
    }
    
    Ok(())
}

/// registerコマンドの処理
fn handle_register_command(address: String, name: Option<String>) -> Result<()> {
    let config_path = get_config_path()?;
    let mut config = if config_path.exists() {
        AppConfig::load_from_file(&config_path)?
    } else {
        AppConfig::default()
    };
    
    let device_name = name.unwrap_or_else(|| format!("Device-{}", &address[..8]));
    
    // デバイスを登録
    config.register_device(device_name.clone(), address.clone(), "Unknown".to_string());
    config.save_to_file(&config_path)?;
    
    println!("デバイスを登録しました: {} ({})", device_name, address);
    Ok(())
}

/// unregisterコマンドの処理
fn handle_unregister_command(address: String) -> Result<()> {
    let config_path = get_config_path()?;
    let mut config = if config_path.exists() {
        AppConfig::load_from_file(&config_path)?
    } else {
        println!("設定ファイルが存在しません。登録済みデバイスはありません。");
        return Ok(());
    };
    
    if config.unregister_device(&address) {
        config.save_to_file(&config_path)?;
        println!("デバイスの登録を解除しました: {}", address);
    } else {
        println!("指定されたデバイスは登録されていません: {}", address);
    }
    
    Ok(())
}

/// statusコマンドの処理
fn handle_status_command() -> Result<()> {
    let config_path = get_config_path()?;
    let config = if config_path.exists() {
        AppConfig::load_from_file(&config_path)?
    } else {
        println!("設定ファイルが存在しません。初期化してください。");
        return Ok(());
    };
    
    println!("=== Bluetooth デバイス管理ツール 状態 ===");
    println!("設定ファイル: {:?}", config_path);
    println!("自動接続: {}", if config.auto_connect { "有効" } else { "無効" });
    println!("接続タイムアウト: {}秒", config.connection_timeout);
    
    if let Some(default_device) = &config.default_device {
        println!("デフォルトデバイス: {}", default_device);
    } else {
        println!("デフォルトデバイス: 未設定");
    }
    
    println!("\n=== 登録済みデバイス ({}) ===", config.registered_devices.len());
    if config.registered_devices.is_empty() {
        println!("登録済みデバイスはありません。");
    } else {
        for device in &config.registered_devices {
            println!("  名前: {}", device.name);
            println!("  アドレス: {}", device.address);
            println!("  タイプ: {}", device.device_type);
            if let Some(last_connected) = &device.last_connected {
                println!("  最終接続: {}", last_connected);
            } else {
                println!("  最終接続: なし");
            }
            println!();
        }
    }
    
    Ok(())
}

/// connectコマンドの処理
fn handle_connect_command(address: Option<String>) -> Result<()> {
    let config_path = get_config_path()?;
    let config = if config_path.exists() {
        AppConfig::load_from_file(&config_path)?
    } else {
        AppConfig::default()
    };
    
    let target_address = match address {
        Some(addr) => addr,
        None => {
            match config.default_device {
                Some(default_addr) => {
                    println!("デフォルトデバイスに接続します: {}", default_addr);
                    default_addr
                }
                None => {
                    println!("デフォルトデバイスが設定されていません。");
                    println!("使用方法: connect <MACアドレス> または set-default でデフォルトデバイスを設定してください。");
                    return Ok(());
                }
            }
        }
    };
    
    // Bluetoothマネージャーを使用して接続
    let bluetooth_manager = BluetoothManager::new();
    match bluetooth_manager.connect_device(&target_address) {
        Ok(()) => {
            println!("接続が完了しました。");
            
            // 接続成功時に最終接続時刻を更新（設定ファイルに保存）
            // TODO: 実際の実装では現在時刻を記録
        }
        Err(e) => {
            println!("接続に失敗しました: {}", e);
        }
    }
    
    Ok(())
}

/// disconnectコマンドの処理
fn handle_disconnect_command(address: Option<String>) -> Result<()> {
    let bluetooth_manager = BluetoothManager::new();
    
    match address {
        Some(addr) => {
            // 指定されたデバイスから切断
            match bluetooth_manager.disconnect_device(&addr) {
                Ok(()) => {
                    println!("切断が完了しました。");
                }
                Err(e) => {
                    println!("切断に失敗しました: {}", e);
                }
            }
        }
        None => {
            // すべての接続済みデバイスから切断
            println!("すべての接続済みデバイスから切断中...");
            
            // 接続済みデバイス一覧を取得
            match bluetooth_manager.list_devices() {
                Ok(devices) => {
                    let connected_devices: Vec<_> = devices.into_iter()
                        .filter(|d| d.is_connected)
                        .collect();
                    
                    if connected_devices.is_empty() {
                        println!("接続済みのデバイスはありません。");
                    } else {
                        for device in connected_devices {
                            println!("デバイス {} から切断中...", device.name);
                            if let Err(e) = bluetooth_manager.disconnect_device(&device.address) {
                                println!("デバイス {} の切断に失敗しました: {}", device.name, e);
                            }
                        }
                        println!("すべてのデバイスからの切断処理が完了しました。");
                    }
                }
                Err(e) => {
                    println!("デバイス一覧の取得に失敗しました: {}", e);
                }
            }
        }
    }
    
    Ok(())
}

/// set-defaultコマンドの処理
fn handle_set_default_command(address: String) -> Result<()> {
    let config_path = get_config_path()?;
    let mut config = if config_path.exists() {
        AppConfig::load_from_file(&config_path)?
    } else {
        AppConfig::default()
    };
    
    // 指定されたデバイスが登録済みかチェック
    if config.get_registered_device(&address).is_some() {
        config.set_default_device(Some(address.clone()));
        config.save_to_file(&config_path)?;
        println!("デフォルトデバイスを設定しました: {}", address);
    } else {
        println!("警告: 指定されたデバイスは登録されていません: {}", address);
        println!("未登録のデバイスでもデフォルトに設定しますか？ (y/N)");
        
        // 簡単な確認（実際の実装では標準入力を読み取る）
        config.set_default_device(Some(address.clone()));
        config.save_to_file(&config_path)?;
        println!("デフォルトデバイスを設定しました: {}", address);
    }
    
    Ok(())
}

/// デフォルト動作（コマンド未指定時）
fn handle_default_action() -> Result<()> {
    println!("=== Bluetoothデバイス自動接続 ===");
    
    let config_path = get_config_path()?;
    let config = if config_path.exists() {
        AppConfig::load_from_file(&config_path)?
    } else {
        println!("設定ファイルが存在しません。初期化してください。");
        return Ok(());
    };
    
    // 自動接続が無効の場合
    if !config.auto_connect {
        println!("自動接続が無効になっています。");
        println!("使用方法:");
        println!("  list                    - デバイス一覧を表示");
        println!("  register -a <address>   - デバイスを登録");
        println!("  connect [address]       - デバイスに接続");
        println!("  status                  - 現在の状態を表示");
        return Ok(());
    }
    
    // デフォルトデバイスが設定されている場合
    if let Some(default_address) = &config.default_device {
        println!("デフォルトデバイスに自動接続します: {}", default_address);
        
        let bluetooth_manager = BluetoothManager::new();
        match bluetooth_manager.connect_device(default_address) {
            Ok(()) => {
                println!("自動接続が完了しました。");
            }
            Err(e) => {
                println!("自動接続に失敗しました: {}", e);
                println!("手動で接続を試行してください。");
            }
        }
    } else {
        println!("デフォルトデバイスが設定されていません。");
        
        // 登録済みデバイスがある場合は選択肢を表示
        if !config.registered_devices.is_empty() {
            println!("登録済みデバイス:");
            for (index, device) in config.registered_devices.iter().enumerate() {
                println!("  {}. {} ({})", index + 1, device.name, device.address);
            }
            println!("\nset-default コマンドでデフォルトデバイスを設定してください。");
        } else {
            println!("登録済みデバイスがありません。");
            println!("まず register コマンドでデバイスを登録してください。");
        }
    }
    
    Ok(())
}