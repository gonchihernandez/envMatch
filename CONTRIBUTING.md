# Contributing to envMatch 🦀

Thank you for your interest in contributing to envMatch! This project is open source and welcomes contributions from the community.

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/envMatch.git
   cd envMatch
   ```
3. **Create a new branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 🛠️ Development Setup

### Prerequisites
- Rust 1.70.0 or higher
- Cargo (comes with Rust)

### Building the Project
```bash
# Build in development mode
cargo build

# Build optimized release
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy
```

### Testing Your Changes
```bash
# Test the CLI
cargo run -- init
cargo run -- set TEST_VAR test_value
cargo run -- list
```

## 📝 Contribution Guidelines

### Code Style
- Follow Rust naming conventions (snake_case for functions/variables)
- Use `cargo fmt` to format your code
- Ensure `cargo clippy` passes without warnings
- Add comments for complex logic

### Commit Messages
Use clear and descriptive commit messages:
```
feat: add export command for environment variables
fix: handle missing config file gracefully
docs: update README with new command examples
```

### Pull Request Process
1. **Update documentation** if you're adding new features
2. **Add tests** for new functionality
3. **Ensure all tests pass**: `cargo test`
4. **Update CHANGELOG.md** if applicable
5. **Submit your pull request** with a clear description

## 🐛 Reporting Issues

When reporting issues, please include:
- **Environment details**: OS, Rust version, envMatch version
- **Steps to reproduce** the issue
- **Expected vs actual behavior**
- **Error messages** (if any)

## 💡 Feature Requests

We welcome feature requests! Please:
- **Check existing issues** to avoid duplicates
- **Describe the use case** clearly
- **Explain why** this feature would be valuable
- **Consider the scope** - keep it focused

## 🔍 Areas for Contribution

- **New features**: Export/import functionality, encryption, shell integration
- **Documentation**: Improve README, add examples, create tutorials
- **Testing**: Add more test cases, improve coverage
- **Performance**: Optimize file I/O, improve CLI responsiveness
- **Platform support**: Ensure compatibility across different OS

## 📋 Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers and be patient with questions
- Keep discussions on-topic and professional

## 🙏 Recognition

Contributors will be recognized in:
- The project's README
- Release notes for significant contributions
- GitHub's contributor graph

## 📞 Questions?

- **Open an issue** for technical questions
- **Start a discussion** for broader topics
- **Contact the maintainer**: [@gonchihernandez](https://github.com/gonchihernandez)

---

Thank you for contributing to envMatch! 🦀✨
