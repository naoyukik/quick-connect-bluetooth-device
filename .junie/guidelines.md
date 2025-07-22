# Development Guidelines for connect-bluetooth-device
- 日本語で会話をします。

## Build/Configuration Instructions

### Prerequisites
- Rust toolchain (tested with Rust 2024 edition)
- Cargo package manager (comes with Rust)

### Building the Project
```bash
# Standard development build
cargo build

# Release build (optimized)
cargo build --release

# Run the application
cargo run
```

### Build Configuration
- **Edition**: Rust 2024 (latest edition as of project creation)
- **Profile**: Uses standard Cargo profiles (`dev` for development, `release` for production)
- **Dependencies**: Currently no external dependencies - pure Rust project
- **Build Time**: Typical build time is under 1 second for this minimal project

### Platform Considerations
- Project is cross-platform compatible
- Windows paths use backslashes (`\`) in build output
- No platform-specific dependencies currently

## Testing Information

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output (shows println! statements)
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in verbose mode
cargo test --verbose
```

### Test Structure
- **Unit Tests**: Located in `#[cfg(test)]` modules within source files
- **Test Location**: Currently in `src/main.rs` under the `tests` module
- **Test Framework**: Uses Rust's built-in testing framework

### Adding New Tests
1. Add test functions within existing `#[cfg(test)]` modules:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_test_name() {
        // Test implementation
        assert_eq!(expected, actual);
    }
}
```

2. For integration tests, create files in `tests/` directory:
```rust
// tests/integration_test.rs
use connect_bluetooth_device::*;

#[test]
fn integration_test_example() {
    // Integration test code
}
```

### Test Example (Working)
The project includes a working test example:
```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(0, 0), 0);
}
```

## Additional Development Information

### Project Structure
```
connect-bluetooth-device/
├── src/
│   └── main.rs          # Main application entry point
├── target/              # Build artifacts (auto-generated)
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Dependency lock file (auto-generated)
└── .junie/
    └── guidelines.md   # This file
```

### Code Style Guidelines
- Follow standard Rust formatting conventions
- Use `cargo fmt` to automatically format code
- Use `cargo clippy` for linting and best practices
- Function names should use `snake_case`
- Constants should use `SCREAMING_SNAKE_CASE`
- Types should use `PascalCase`

### Development Workflow
1. **Code Changes**: Make changes to source files
2. **Format**: Run `cargo fmt` to format code
3. **Lint**: Run `cargo clippy` to check for issues
4. **Test**: Run `cargo test` to ensure tests pass
5. **Build**: Run `cargo build` to verify compilation
6. **Run**: Use `cargo run` to execute the application

### Debugging
- Use `println!` for simple debugging output
- For more advanced debugging, consider using the `dbg!` macro
- Tests can include debug output with `println!` (use `--nocapture` flag)
- Use `cargo build` to check for compilation errors

### Future Development Notes
- Project name suggests Bluetooth functionality - consider adding dependencies like `btleplug` or `bluer` when implementing Bluetooth features
- Current project is minimal - expand functionality as needed
- Consider adding error handling with `Result<T, E>` types for robust applications
- For Bluetooth development, platform-specific considerations may apply

### Performance Considerations
- Current build is very fast (~0.7s) due to minimal dependencies
- Adding Bluetooth libraries will increase build time
- Use `cargo build --release` for optimized builds in production

### Maintenance
- Keep dependencies updated with `cargo update`
- Regularly run `cargo audit` if security-sensitive dependencies are added
- Monitor Rust edition updates for language improvements

## コミット生成
### 1. 目的

このガイドラインは、本プロジェクトにおけるバージョン管理の品質を最高水準に維持することを目的とする。AIエージェント（以下、私）が生成するすべてのコミットは、Googleのエンジニアリングプラクティスに準拠した「小
さく、自己完結し、目的が明確な」単位でなければならない。プロジェクトに参加するすべての者は、このガイドラインを理解し、遵守すること。

### 2. 私が遵守するコミットの三原則

私は、以下の三原則に厳格に従い、すべての変更をコミットする。

#### 1. 単一責任の原則 (Single Responsibility)
    * 1つのコミットは、ただ1つの論理的な関心事にのみ集中する。機能追加、バグ修正、リファクタリング、テスト追加、ドキュメント更新は、決して1つのコミットに混在させない。

#### 2. 自己完結の原則 (Self-Contained)
    * すべてのコミットは、それ単体でプロジェクトが安定した状態（ビルド成功、テスト通過）を保つものでなければならない。ビルドを破壊するような中途半端な変更は決してコミットしない。

#### 3. 明瞭性の原則 (Clarity in Messaging)
    * コミットメッセージは、変更の「何（What）」だけでなく、「なぜ（Why）」を明確に記述する。変更の背景、解決される問題、技術的選択の理由を伝える。

##### 3. タスク指示に関する規定

私に対するタスク指示は、上記の三原則を助長するものでなければならない。君の指示の粒度と質が、そのままコミットの粒度と質に直結することを常に意識せよ。

###### 3.1. 指示は具体的かつ焦点を絞ること

論理的な変更単位で、具体的かつ明確な指示を与えること。

* 推奨される指示（良い例）:
    * feat: 「usersテーブルにlast_login_atカラムを追加し、ログイン時に現在時刻を記録する機能を実装せよ。」
    * refactor: 「AuthenticationServiceから認証ロジックをCredentialValidatorクラスに切り出せ。」
    * fix: 「パスワードリセット機能で、有効期限切れのトークンが使用可能なバグを修正せよ。」
    * test: 「CredentialValidatorクラスに対する単体テストを作成せよ。」

* 却下される指示（悪い例）:
    * 曖昧: 「この辺のコードをいい感じにしておけ。」
    * 過大: 「ユーザー認証システムを実装せよ。」
    * 複合: 「UIを修正し、ついでにデータベースのパフォーマンスも改善せよ。」

###### 3.2. 大規模タスクは対話を通じて分割すること

「認証システムの実装」のような大規模なタスクを指示する場合、以下のプロセスを遵守すること。

1. 君: 大規模な目標を私に提示する。
2. 私: その目標を達成するための、論理的なサブタスクのリストと実行計画を提案する。
3. 君: 提示された計画をレビューし、承認する。
4. 私: 承認された計画に基づき、サブタスクを1つずつ実行し、完了ごとに原則に基づいたコミットを作成する。

この対話的アプローチにより、大規模な変更であっても、クリーンで追跡可能なコミット履歴を維持する。
