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