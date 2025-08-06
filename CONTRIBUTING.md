# Contributing to Wake

Thank you for your interest in contributing to Wake! We welcome contributions from the hardware and embedded systems community.

## 🎯 Our Focus

Wake is a hardware-first coding agent. We're particularly interested in contributions that:

- Improve hardware debugging capabilities
- Add support for new microcontrollers and development boards
- Enhance driver generation from datasheets
- Add new hardware communication protocols
- Improve embedded systems tooling
- Add IoT and robotics features

## 🚀 Getting Started

1. **Fork the repository**
   ```bash
   git clone https://github.com/Try-Wind/Wake.git
   cd Wake
   ```

2. **Set up your development environment**
   ```bash
   cargo build
   cargo test
   ```

3. **Create a branch for your feature**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 📝 Development Process

### Before You Start

- Check existing issues and pull requests
- Open an issue to discuss major changes
- For hardware-specific features, provide context about the use case

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add comments for hardware-specific logic
- Include examples for hardware features

### Testing

- Write tests for new features
- Test with actual hardware when possible
- Document hardware requirements for tests
- Run `cargo test` before submitting

## 🔧 Areas We Need Help

### High Priority

- **Datasheet Parser**: Improve OCR and parsing of hardware datasheets
- **Protocol Analyzers**: Add support for more communication protocols
- **Board Support**: Add templates for popular development boards
- **Driver Templates**: Expand driver generation templates

### Medium Priority

- **Documentation**: Hardware examples and tutorials
- **Testing**: Hardware simulation and testing frameworks
- **UI/UX**: Improve terminal interface for hardware debugging
- **Performance**: Optimize for embedded development workflows

### Always Welcome

- Bug fixes
- Documentation improvements
- Test coverage
- Performance optimizations

## 📋 Pull Request Process

1. **Update documentation** for new features
2. **Add tests** for your changes
3. **Run all tests** locally: `cargo test`
4. **Update the README** if needed
5. **Submit PR** with clear description

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Hardware support
- [ ] Documentation

## Hardware Tested (if applicable)
- Board/MCU:
- Protocol:
- Test scenario:

## Checklist
- [ ] Tests pass
- [ ] Documentation updated
- [ ] Code formatted (`cargo fmt`)
- [ ] Hardware tested (if applicable)
```

## 🐛 Reporting Issues

### Bug Reports

Include:
- Wake version
- Operating system
- Hardware setup (if relevant)
- Steps to reproduce
- Expected vs actual behavior

### Feature Requests

Include:
- Use case description
- Hardware/platform involved
- Example workflow
- Potential implementation ideas

## 💡 Hardware Contribution Guidelines

### Adding Board Support

1. Create template in `wake-core/src/hardware/boards/`
2. Include:
   - Pin mappings
   - Peripheral configuration
   - Common use cases
   - Example code

### Adding Protocol Support

1. Implement in `wake-core/src/hardware/protocols/`
2. Include:
   - Protocol specification
   - Common implementations
   - Debugging helpers
   - Test cases

### Driver Templates

1. Add to `wake-core/src/hardware/drivers/`
2. Follow the template structure:
   ```rust
   // Driver metadata
   // Register definitions
   // Initialization code
   // Core functionality
   // Helper functions
   ```

## 🌟 Recognition

Contributors will be:
- Listed in CONTRIBUTORS file
- Mentioned in release notes
- Given credit in documentation

## 📧 Communication

- **GitHub Issues**: For bugs and features
- **Discussions**: For general questions
- **Pull Requests**: For code contributions

## 📜 Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help build a positive community

## 🙏 Thank You!

Your contributions help make Wake better for the entire hardware development community. Whether you're fixing a bug, adding a feature, or improving documentation, every contribution matters!

---

Built with ❤️ by Wind and the Wake community