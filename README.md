# connect-bluetooth-device

Windows環境でBluetoothデバイスの接続を管理するCLIアプリケーション

## 概要

`connect-bluetooth-device`は、Windows環境でBluetoothデバイスの接続・切断・管理を効率的に行うためのコマンドラインツールです。デバイスの登録、自動接続、状態管理などの機能を提供します。

## 特徴

- 🔍 **デバイス検出**: 利用可能なBluetoothデバイスの一覧表示
- 📝 **デバイス登録**: よく使用するデバイスの登録・管理
- 🔗 **簡単接続**: 登録済みデバイスへの簡単接続
- ⚙️ **設定管理**: 設定ファイルによる永続的な設定保存
- 🚀 **自動接続**: デフォルトデバイスへの自動接続機能

## インストール

### 前提条件

- Windows 10/11
- Rust toolchain (2024 edition)

### ビルド方法

```bash
# リポジトリをクローン
git clone <repository-url>
cd connect-bluetooth-device

# 依存関係をインストールしてビルド
cargo build --release

# 実行ファイルは target/release/ に生成されます
```

## 使用方法

### 基本的な使い方

```bash
# ヘルプを表示
connect-bluetooth-device --help

# 利用可能なBluetoothデバイス一覧を表示
connect-bluetooth-device list

# デバイスを登録
connect-bluetooth-device register -a AA:BB:CC:DD:EE:FF -n "My Bluetooth Mouse"

# 登録済みデバイス一覧を表示
connect-bluetooth-device list --registered

# デバイスに接続
connect-bluetooth-device connect AA:BB:CC:DD:EE:FF

# 現在の状態を表示
connect-bluetooth-device status

# デフォルトデバイスを設定
connect-bluetooth-device set-default -a AA:BB:CC:DD:EE:FF

# デフォルトデバイスに自動接続（コマンド未指定時）
connect-bluetooth-device
```

## コマンド一覧

### `list` - デバイス一覧表示

利用可能なBluetoothデバイスまたは登録済みデバイスの一覧を表示します。

```bash
# すべての利用可能なデバイスを表示
connect-bluetooth-device list

# 登録済みデバイスのみ表示
connect-bluetooth-device list --registered
```

### `register` - デバイス登録

Bluetoothデバイスを登録します。

```bash
# MACアドレスを指定してデバイスを登録
connect-bluetooth-device register -a AA:BB:CC:DD:EE:FF

# デバイス名も指定して登録
connect-bluetooth-device register -a AA:BB:CC:DD:EE:FF -n "My Device"
```

**オプション:**
- `-a, --address <ADDRESS>`: デバイスのMACアドレス（必須）
- `-n, --name <NAME>`: デバイス名（オプション、未指定時は自動生成）

### `unregister` - デバイス登録解除

登録済みデバイスの登録を解除します。

```bash
connect-bluetooth-device unregister -a AA:BB:CC:DD:EE:FF
```

### `connect` - デバイス接続

指定されたデバイスまたはデフォルトデバイスに接続します。

```bash
# 指定したデバイスに接続
connect-bluetooth-device connect AA:BB:CC:DD:EE:FF

# デフォルトデバイスに接続
connect-bluetooth-device connect
```

### `disconnect` - デバイス切断

指定されたデバイスまたはすべてのデバイスから切断します。

```bash
# 指定したデバイスから切断
connect-bluetooth-device disconnect AA:BB:CC:DD:EE:FF

# すべての接続済みデバイスから切断
connect-bluetooth-device disconnect
```

### `set-default` - デフォルトデバイス設定

デフォルトデバイスを設定します。

```bash
connect-bluetooth-device set-default -a AA:BB:CC:DD:EE:FF
```

### `status` - 状態表示

現在の設定と登録済みデバイスの状態を表示します。

```bash
connect-bluetooth-device status
```

## 設定ファイル

アプリケーションは実行ファイルと同じディレクトリに `config.toml` ファイルを作成し、設定を保存します。

### 設定ファイルの例

```toml
# デフォルト接続デバイスのMACアドレス
default_device = "AA:BB:CC:DD:EE:FF"

# 自動接続の有効/無効
auto_connect = true

# 接続タイムアウト（秒）
connection_timeout = 30

# 登録済みデバイス一覧
[[registered_devices]]
name = "My Bluetooth Mouse"
address = "AA:BB:CC:DD:EE:FF"
device_type = "Peripheral"
last_connected = "2024-07-22T22:42:00Z"

[[registered_devices]]
name = "My Bluetooth Headphones"
address = "11:22:33:44:55:66"
device_type = "Audio/Video"
last_connected = "2024-07-22T20:30:00Z"
```

### 設定項目

- `default_device`: デフォルト接続デバイスのMACアドレス
- `auto_connect`: 自動接続機能の有効/無効
- `connection_timeout`: 接続タイムアウト時間（秒）
- `registered_devices`: 登録済みデバイスの配列
  - `name`: デバイス名
  - `address`: MACアドレス
  - `device_type`: デバイスタイプ
  - `last_connected`: 最終接続日時

## 使用例

### 1. 初回セットアップ

```bash
# 1. 利用可能なデバイスを確認
connect-bluetooth-device list

# 2. よく使うデバイスを登録
connect-bluetooth-device register -a AA:BB:CC:DD:EE:FF -n "My Mouse"
connect-bluetooth-device register -a 11:22:33:44:55:66 -n "My Headphones"

# 3. デフォルトデバイスを設定
connect-bluetooth-device set-default -a AA:BB:CC:DD:EE:FF

# 4. 設定を確認
connect-bluetooth-device status
```

### 2. 日常的な使用

```bash
# デフォルトデバイスに自動接続
connect-bluetooth-device

# 特定のデバイスに接続
connect-bluetooth-device connect 11:22:33:44:55:66

# すべてのデバイスから切断
connect-bluetooth-device disconnect
```

### 3. デバイス管理

```bash
# 登録済みデバイス一覧を確認
connect-bluetooth-device list --registered

# 不要なデバイスの登録を解除
connect-bluetooth-device unregister -a 11:22:33:44:55:66

# 現在の状態を確認
connect-bluetooth-device status
```

## トラブルシューティング

### よくある問題

**Q: デバイスが見つからない**
- Bluetoothが有効になっているか確認してください
- デバイスがペアリング可能な状態になっているか確認してください
- Windows のBluetoothドライバーが最新であることを確認してください

**Q: 接続に失敗する**
- MACアドレスが正しいか確認してください
- デバイスが他のデバイスに接続されていないか確認してください
- 一度デバイスをWindowsの設定から削除して再ペアリングしてみてください

**Q: 設定ファイルが見つからない**
- 初回実行時に自動的に作成されます
- 実行ファイルと同じディレクトリに `config.toml` が作成されているか確認してください

## 開発情報

### テスト実行

```bash
# すべてのテストを実行
cargo test

# テスト出力を表示
cargo test -- --nocapture
```

### ビルド設定

- **Rust Edition**: 2024
- **主要依存関係**:
  - `clap`: コマンドライン引数解析
  - `windows`: Windows API アクセス
  - `serde`: シリアライゼーション
  - `toml`: 設定ファイル処理
  - `anyhow`: エラーハンドリング

## ライセンス

このプロジェクトのライセンス情報については、LICENSEファイルを参照してください。

## 貢献

バグ報告や機能要望は、GitHubのIssuesページでお知らせください。プルリクエストも歓迎します。

---

**注意**: このツールは現在開発中です。一部の機能はモック実装となっており、実際のBluetoothデバイスとの接続には追加の実装が必要な場合があります。