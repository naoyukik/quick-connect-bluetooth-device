//! Bluetoothデバイス接続管理ツール
//! Windows環境でBluetoothデバイスの接続を管理するCLIアプリケーション

mod bluetooth;
mod config;
mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, run_command};

fn main() -> Result<()> {
    // 設定ファイルの初期化
    let _config = config::initialize_config_if_needed()?;
    
    // CLIコマンドの解析と実行
    let cli = Cli::parse();
    run_command(cli)?;
    
    Ok(())
}
