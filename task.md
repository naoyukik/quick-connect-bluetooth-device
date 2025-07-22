### タスク計画書 (task.md)

#### 開発タスク一覧

**1. `環境セットアップ`**
- Cargo.tomlに依存関係追加（clap, windows-rs, serde, toml, anyhow）
- プロジェクト構造作成（bluetooth/mod.rs, config/mod.rs, cli/mod.rs）

**2. 設定管理機能**
- AppConfig構造体実装
- config.toml読み書き機能
- 設定ファイル初期化処理

**3. Bluetooth操作機能**
- BluetoothDevice構造体実装
- Windows Bluetooth APIラッパー
- デバイス一覧取得機能
- デバイス接続機能

**4. CLI機能**
- clapコマンド定義
- `list`コマンド（デバイス一覧）
- `register`コマンド（デバイス登録）
- `status`コマンド（状態表示）
- デフォルト接続動作

**5. 仕上げ**
- エラーハンドリング統一
- テスト作成
- ドキュメント作成
- パフォーマンス最適化
