# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-09-09

### Added
- Initial release of envMatch 🦀
- Multi-environment support (development, production, staging, etc.)
- Environment variable management with YAML storage
- CLI commands:
  - `init` - Initialize envMatch in project directory
  - `set` - Set environment variables with optional environment specification
  - `get` - Retrieve environment variables
  - `unset` - Remove environment variables
  - `switch` - Switch between environments
  - `list` - List all variables in current or specified environment
  - `current` - Show active environment
  - `validate` - Validate required variables are set
  - `envs` - List all available environments
- Secure local storage in `.envMatch` directory
- Environment validation with required variable checking
- Smart error handling and user-friendly messages
- Cross-platform support (Windows, macOS, Linux)

### Features
- 🔧 Multi-Environment Support
- 🔄 Easy Environment Switching  
- ✅ Variable Validation
- 📋 Smart Listing
- 🎯 Simple CLI
- 📁 Local YAML Storage
- 🚀 Fast & Reliable (Built with Rust)

---

**Project created by [@gonchihernandez](https://github.com/gonchihernandez)**  
**Named in honor of Rust's elegant `match` statement** 🦀
