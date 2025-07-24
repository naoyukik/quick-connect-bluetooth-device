### タスク計画書 (task.md)

#### 開発タスク一覧

**1. 環境セットアップ**
1. Cargo.tomlに依存関係追加（clap, windows-rs, serde, toml, anyhow）

**2. 設定管理機能**
1. プロジェクト構造作成（bluetooth/mod.rs, config/mod.rs, cli/mod.rs）
2. AppConfig構造体実装
3. config.toml読み書き機能
4. 設定ファイル初期化処理

**3. Bluetooth操作機能**
1. BluetoothDevice構造体実装
2. Windows Bluetooth APIラッパー
3. デバイス一覧取得機能
4. デバイス接続機能

**4. CLI機能**
1. clapコマンド定義
2. `list`コマンド（デバイス一覧）
3. `register`コマンド（デバイス登録）
4. `status`コマンド（状態表示）
5. デフォルト接続動作

**5. 仕上げ**
1. エラーハンドリング統一
2. テスト作成
3. ドキュメント作成
4. パフォーマンス最適化
