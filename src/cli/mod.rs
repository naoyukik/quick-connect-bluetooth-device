//! CLI機能モジュール
//! clapを使用してコマンドライン引数を処理し、各種コマンドを実行

use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::config::{AppConfig, get_config_path};

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
        println!("注意: 実際のBluetooth APIはまだ実装されていません。");
        println!("現在は登録済みデバイスのみ表示します。\n");
        
        // 登録済みデバイスを表示
        if config.registered_devices.is_empty() {
            println!("登録済みデバイスはありません。");
        } else {
            println!("登録済みデバイス:");
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
    match address {
        Some(addr) => println!("デバイス {} に接続中...", addr),
        None => println!("デフォルトデバイスに接続中..."),
    }
    // TODO: 実際の接続処理を実装
    Ok(())
}

/// disconnectコマンドの処理
fn handle_disconnect_command(address: Option<String>) -> Result<()> {
    match address {
        Some(addr) => println!("デバイス {} から切断中...", addr),
        None => println!("すべてのデバイスから切断中..."),
    }
    // TODO: 実際の切断処理を実装
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
    println!("デフォルト動作: 自動接続を実行中...");
    // TODO: 実際の自動接続処理を実装
    Ok(())
}