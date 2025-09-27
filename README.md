# envMatch 🦀

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub](https://img.shields.io/badge/GitHub-gonchihernandez%2FenvMatch-blue.svg)](https://github.com/gonchihernandez/envMatch)

**Environment Variable Manager** - A powerful CLI tool written in Rust for managing environment variables across different environments (development, staging, production, etc.). Named in honor of Rust's elegant `match` statement for pattern matching environments.

## ✨ Features

- 🔧 **Multi-Environment Support** - Manage separate variable sets for dev, staging, prod
- 🎨 **Beautiful TUI Interface** - Interactive terminal user interface with Ratatui
- 🔄 **Easy Environment Switching** - Switch between environments with a single command
- ✅ **Variable Validation** - Ensure required variables are set before deployment
- 📋 **Smart Listing** - View all variables in any environment
- 🎯 **Dual Interface** - Both CLI commands and interactive TUI available
- 📁 **Local Storage** - Variables stored securely in YAML files
- 🚀 **Fast & Reliable** - Built with Rust for performance and safety

## 🚀 Quick Start

### Installation

#### From Source (Recommended)
```bash
# Clone the repository
git clone https://github.com/gonchihernandez/envMatch.git
cd envMatch

# Build the project
cargo build --release

# (Optional) Install globally
cargo install --path .
```

#### From crates.io (Coming Soon)
```bash
# Install from crates.io (when published)
cargo install envMatch
```

#### Manual Installation
```bash
# Download the latest release from GitHub
# Extract and run the binary directly
./envMatch --help
```

### Basic Usage

#### Interactive TUI Mode (Default)
```bash
# Launch the interactive terminal interface (default when envMatch is initialized)
./envMatch
# or explicitly
./envMatch tui
```

The TUI provides a beautiful, interactive interface with:
- 📁 **Left Panel**: Environment list with current environment highlighted
- 🔧 **Right Panel**: Variable list for the selected environment
- ⌨️ **Keyboard Navigation**: 
  - `Tab` to switch between panels
  - `↑/↓` or `k/j` to navigate
  - `a` to add variables, `e` to edit, `d` to delete
  - `Enter` to switch environments
  - `h` or `F1` for help
  - `q` to quit

#### Command Line Mode
```bash
# Initialize envMatch in your project
./envMatch init

# Set environment variables
./envMatch set DATABASE_URL postgres://localhost/myapp
./envMatch set API_KEY your_secret_key
./envMatch set DEBUG true

# Create production environment
./envMatch set DATABASE_URL postgres://prod-server/myapp --env production
./envMatch set API_KEY prod_secret_key --env production
./envMatch set DEBUG false --env production

# Switch environments
./envMatch switch production

# List current environment variables
./envMatch list

# Validate required variables
./envMatch validate --required DATABASE_URL,API_KEY
```

## 📚 Commands

### Interactive TUI
```bash
./envMatch tui
# or simply
./envMatch
```
Launches the interactive terminal user interface for visual management of environments and variables.

### Initialize
```bash
./envMatch init
```
Creates `.envMatch` directory structure in your current project.

### Set Variables
```bash
# Set in current environment (default: development)
./envMatch set KEY value

# Set in specific environment
./envMatch set KEY value --env production
```

### Get Variables
```bash
# Get from current environment
./envMatch get KEY

# Get from specific environment
./envMatch get KEY --env production
```

### Remove Variables
```bash
# Remove from current environment
cargo run -- unset KEY

# Remove from specific environment
cargo run -- unset KEY --env production
```

### Environment Management
```bash
# Switch to different environment
cargo run -- switch production

# Show current environment
cargo run -- current

# List all available environments
cargo run -- envs
```

### List Variables
```bash
# List variables in current environment
cargo run -- list

# List variables in specific environment
cargo run -- list --env production
```

### Validation
```bash
# Check if required variables are set
cargo run -- validate --required DATABASE_URL,API_KEY

# General environment health check
cargo run -- validate
```

## 📁 Project Structure

After initialization, envMatch creates:

```
your-project/
└── .envMatch/
    ├── config.yaml              # Global configuration
    └── environments/
        ├── development.yaml      # Development variables
        ├── production.yaml       # Production variables
        └── staging.yaml          # Staging variables (if created)
```

## 💡 Example Workflow

```bash
# 1. Initialize in your project
cargo run -- init

# 2. Set up development environment
cargo run -- set DATABASE_URL postgres://localhost/myapp
cargo run -- set REDIS_URL redis://localhost:6379
cargo run -- set DEBUG true
cargo run -- set LOG_LEVEL debug

# 3. Set up production environment
cargo run -- set DATABASE_URL postgres://prod.example.com/myapp --env production
cargo run -- set REDIS_URL redis://prod.example.com:6379 --env production
cargo run -- set DEBUG false --env production
cargo run -- set LOG_LEVEL info --env production

# 4. Work in development
cargo run -- switch development
cargo run -- list
# Shows all dev variables

# 5. Deploy to production
cargo run -- switch production
cargo run -- validate --required DATABASE_URL,REDIS_URL
# ✅ All required variables are set in environment 'production'

# 6. Check what environments you have
cargo run -- envs
# 📁 Available environments:
# • development
# • production (current)
```

## 🛠️ Use Cases

### For Developers
- **Local Development**: Keep dev credentials separate from production
- **Testing**: Create isolated test environments
- **Onboarding**: New team members can quickly set up their environment

### For DevOps
- **Deployment Validation**: Ensure all required variables are set before deploy
- **Environment Parity**: Keep track of what variables exist in each environment
- **Configuration Management**: Centralized way to manage environment configs

### For Teams
- **Consistency**: Everyone uses the same variable names and structure
- **Documentation**: Variables are clearly organized and discoverable
- **Security**: Sensitive values aren't accidentally committed to git

## 🔒 Security Notes

- Variables are stored in local YAML files
- The `.envMatch` directory should be added to `.gitignore`
- For production use, consider additional encryption for sensitive values
- Never commit the `.envMatch` directory to version control

## � Testing

### Running Tests
```bash
# Run all tests (unit + integration)
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run tests with verbose output
cargo test -- --nocapture
```

### Test Coverage
- **Unit Tests**: 12 tests covering core functionality
- **Integration Tests**: 13 tests covering CLI behavior
- **Architecture**: Modular design with proper error handling
- **Test Coverage**: All major features and edge cases

## 🏗️ Architecture

The project follows Rust best practices with a modular architecture:

```
src/
├── main.rs           # CLI entry point and command routing
├── commands/         # Business logic for each command
│   └── mod.rs        # EnvMatchCommands implementation
├── config/           # Configuration management
│   └── mod.rs        # ConfigManager for file operations
└── error/            # Error handling
    └── mod.rs        # Custom error types with thiserror

tests/
└── integration_tests.rs  # End-to-end CLI testing
```

### Key Design Principles
- **Separation of Concerns**: Each module has a single responsibility
- **Error Handling**: Comprehensive error types with helpful messages
- **Testability**: Dependency injection for easy testing
- **Type Safety**: Strong typing with custom Result types
- **Documentation**: Extensive tests serve as documentation

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Commit changes: `git commit -am 'Add new feature'`
6. Push to branch: `git push origin feature/new-feature`
7. Submit a Pull Request

## 📋 Requirements

- Rust 1.70.0 or higher
- Cargo (comes with Rust)

## 📦 Dependencies

- `clap` - Command-line argument parsing
- `serde` - Serialization/deserialization
- `serde_yaml` - YAML file handling

## 🐛 Troubleshooting

### "envMatch not initialized"
```bash
# Run this in your project directory
cargo run -- init
```

### "Variable not found"
```bash
# Check what variables exist
cargo run -- list

# Check what environment you're in
cargo run -- current
```

### "Failed to read environment file"
```bash
# The environment might not exist yet, create it by setting a variable
cargo run -- set EXAMPLE_VAR example_value --env your_environment
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Created by [@gonchihernandez](https://github.com/gonchihernandez)
- Built with ❤️ using Rust
- Inspired by Rust's powerful `match` statement for pattern matching
- Thanks to the Rust community for amazing crates and tools

---

**Made with 🦀 Rust - Powered by `match` 💪**  
**Original Author: [@gonchihernandez](https://github.com/gonchihernandez)**
