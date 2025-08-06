# Wake Release Guide

This guide explains how to create and manage releases for Wake on GitHub.

## 📦 Creating a New Release

### Prerequisites
- Ensure all code changes are committed and pushed
- Update version number in `Cargo.toml` files
- Update CHANGELOG.md with release notes
- Run tests: `cargo test`
- Build release binary: `cargo build --release`

### Step-by-Step Release Process

#### 1. Update Version Numbers
Update version in all Cargo.toml files:
```bash
# wake-cli/Cargo.toml
# wake-core/Cargo.toml
# wake-llm/Cargo.toml
# wake-macros/Cargo.toml
version = "0.2.0"  # Example new version
```

#### 2. Create Git Tag
```bash
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```

#### 3. Build Release Binaries
Build for multiple platforms:

**Linux:**
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

**macOS:**
```bash
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin  # M1/M2 Macs
```

**Windows:**
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

#### 4. Create GitHub Release

1. Go to https://github.com/Try-Wind/Wake/releases
2. Click "Draft a new release"
3. Choose the tag you created (e.g., v0.2.0)
4. Set release title: "Wake v0.2.0 - Hardware-First Coding Agent"
5. Add release notes (use template below)
6. Upload compiled binaries:
   - `wake-linux-x64`
   - `wake-macos-x64`
   - `wake-macos-arm64`
   - `wake-windows-x64.exe`
7. Check "Set as latest release"
8. Click "Publish release"

### Release Notes Template

```markdown
# Wake v0.2.0

## 🎉 What's New
- Feature 1: Description
- Feature 2: Description
- Hardware support for [specific boards/protocols]

## 🐛 Bug Fixes
- Fixed issue with...
- Resolved problem where...

## 🔧 Improvements
- Enhanced performance of...
- Better error handling for...

## 📦 Installation

### Quick Install (Linux/macOS)
\```bash
curl -fsSL https://raw.githubusercontent.com/Try-Wind/Wake/main/install.sh | bash
\```

### Manual Download
Download the appropriate binary for your platform from the assets below.

### From Source
\```bash
git clone https://github.com/Try-Wind/Wake.git
cd Wake
cargo install --path wake-cli
\```

## 📚 Documentation
- [User Guide](https://github.com/Try-Wind/Wake/blob/main/docs/USER_GUIDE.md)
- [Hardware Examples](https://github.com/Try-Wind/Wake/blob/main/docs/HARDWARE_EXAMPLES.md)

## 🙏 Contributors
Thanks to everyone who contributed to this release!

## 📝 Full Changelog
See [CHANGELOG.md](https://github.com/Try-Wind/Wake/blob/main/CHANGELOG.md)
```

## 🔄 Automated Release with GitHub Actions

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build Release
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            binary: wake
            name: wake-linux-x64
          - os: macos-latest
            target: x86_64-apple-darwin
            binary: wake
            name: wake-macos-x64
          - os: macos-latest
            target: aarch64-apple-darwin
            binary: wake
            name: wake-macos-arm64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            binary: wake.exe
            name: wake-windows-x64.exe

    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
          override: true
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Prepare Binary
        run: |
          cd target/${{ matrix.target }}/release
          mv ${{ matrix.binary }} ../../../${{ matrix.name }}
      
      - name: Upload Artifact
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.name }}
          path: ${{ matrix.name }}

  release:
    name: Create Release
    needs: build
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Download Artifacts
        uses: actions/download-artifact@v3
        
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            wake-linux-x64/wake-linux-x64
            wake-macos-x64/wake-macos-x64
            wake-macos-arm64/wake-macos-arm64
            wake-windows-x64.exe/wake-windows-x64.exe
          draft: false
          prerelease: false
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## 📊 Version Numbering

We follow Semantic Versioning (SemVer):
- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

## 🔍 Pre-release Checklist

- [ ] All tests pass: `cargo test`
- [ ] Code formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version numbers updated
- [ ] README.md reviewed
- [ ] Examples tested

## 📝 Post-release Tasks

1. **Update Documentation**
   - Update installation instructions if needed
   - Add release to documentation site

2. **Announce Release**
   - Post on project discussions
   - Update social media if applicable
   - Notify major users/contributors

3. **Monitor Issues**
   - Watch for bug reports
   - Be ready to create patch release if critical issues found

## 🆘 Troubleshooting

### Build Failures
- Ensure all dependencies are up to date
- Check Rust toolchain version
- Verify target architecture is installed

### Upload Issues
- Check GitHub token permissions
- Ensure file paths are correct
- Verify binary names match

### Platform-specific Issues
- **macOS**: May need to sign binaries for distribution
- **Windows**: Consider code signing certificate
- **Linux**: Check glibc compatibility

## 📚 Additional Resources

- [GitHub Releases Documentation](https://docs.github.com/en/repositories/releasing-projects-on-github)
- [Cargo Release](https://github.com/crate-ci/cargo-release)
- [Cross-compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)

---

Built with ❤️ by Wind