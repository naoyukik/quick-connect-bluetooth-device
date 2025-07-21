### 設計書 (design.md)

#### 1. 技術スタック
- **言語:** Rust (2024 edition) - 高性能、メモリ安全性
- **CLI:** clap crate - 標準的なCLIライブラリ
- **Bluetooth:** windows-rs crate - Windows API直接アクセス
- **設定:** TOML形式 (serde + toml) - 可読性重視
- **エラー処理:** anyhow crate - 簡潔なエラーハンドリング

#### 2. システム構成
```
[CLI] → [Bluetooth Manager] → [Windows Bluetooth API]
   ↓
[Config Manager] → [config.toml]
```

#### 3. データ構造
```rust
pub struct BluetoothDevice {
    pub name: String,
    pub mac_address: String,
    pub device_id: String,
}

pub struct AppConfig {
    pub device: Option<BluetoothDevice>,
    pub timeout_seconds: u32,
}
```

#### 4. CLIコマンド
- `connect-bluetooth-device` - 登録デバイスに接続
- `connect-bluetooth-device list` - ペアリング済みデバイス一覧
- `connect-bluetooth-device register` - デバイス登録
- `connect-bluetooth-device status` - 現在の状態表示

#### 5. ディレクトリ構造
```
src/
├── main.rs              # エントリーポイント
├── bluetooth/mod.rs     # Bluetooth操作
├── config/mod.rs        # 設定管理
└── cli/mod.rs          # CLI処理
```
