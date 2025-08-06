# GitHub Release Guide for Wake

This guide explains how to create and manage releases for the Wake project on GitHub.

## Prerequisites

- GitHub account with write access to the repository
- Git installed locally
- Rust toolchain installed
- GitHub CLI (optional but recommended): `brew install gh`

## Creating a New Release

### 1. Prepare the Release

1. **Update Version Number**
   - Edit `wake-cli/Cargo.toml` and update the version
   - Update other `Cargo.toml` files if needed
   - Update version in README if displayed

2. **Update Changelog**
   ```markdown
   ## [1.0.0] - 2024-01-XX
   ### Added
   - Hardware-specific tools for embedded development
   - Driver generation from datasheets
   - Protocol debugging capabilities
   
   ### Changed
   - Complete rebrand from Shai to Wake
   - Focus shifted to hardware-first development
   
   ### Fixed
   - Various bug fixes and improvements
   ```

3. **Test Everything**
   ```bash
   cargo test
   cargo build --release
   ```

### 2. Create Release Builds

Build for multiple platforms:

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

### 3. Create GitHub Release

#### Using GitHub Web Interface

1. Go to https://github.com/Try-Wind/Wake/releases
2. Click "Draft a new release"
3. Click "Choose a tag" and create new tag (e.g., `v1.0.0`)
4. Set release title: "Wake v1.0.0 - Hardware-First Coding Agent"
5. Write release notes (use changelog content)
6. Upload binary files:
   - `wake-linux-x86_64`
   - `wake-macos-x86_64`
   - `wake-macos-arm64`
   - `wake-windows-x86_64.exe`
7. Check "Set as the latest release"
8. Click "Publish release"

#### Using GitHub CLI

```bash
# Create tag
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0

# Create release with GitHub CLI
gh release create v1.0.0 \
  --title "Wake v1.0.0 - Hardware-First Coding Agent" \
  --notes-file RELEASE_NOTES.md \
  ./target/release/wake-* \
  --latest
```

### 4. Release Notes Template

```markdown
# Wake v1.0.0 - Hardware-First Coding Agent

## 🚀 Highlights

Wake is now a powerful hardware-aware coding assistant built by Wind, designed specifically for embedded systems, IoT development, and hardware programming.

## ✨ Features

- **Driver Generation**: Generate production-ready drivers from component specifications
- **Protocol Debugging**: Analyze and fix I2C, SPI, UART communication issues
- **Circuit Analysis**: Review and optimize electronic circuits
- **Multi-Provider Support**: Works with OpenAI, Anthropic, Google, and local models

## 📦 Installation

### Quick Install (Linux/macOS)
\`\`\`bash
curl -fsSL https://github.com/Try-Wind/Wake/releases/download/v1.0.0/install.sh | bash
\`\`\`

### Manual Download
Download the appropriate binary for your platform from the assets below.

## 🔧 Configuration

Run \`wake auth\` to configure your AI provider.

## 📚 Documentation

- [Getting Started](https://github.com/Try-Wind/Wake#readme)
- [Hardware Examples](https://github.com/Try-Wind/Wake/docs/HARDWARE_EXAMPLES.md)
- [Contributing](https://github.com/Try-Wind/Wake/CONTRIBUTING.md)

## 🙏 Credits

Built with ❤️ by Wind (founded by Adhyaay Karnwal)
```

## Automated Releases with GitHub Actions

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            binary: wake
          - os: macos-latest
            target: x86_64-apple-darwin
            binary: wake
          - os: macos-latest
            target: aarch64-apple-darwin
            binary: wake
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            binary: wake.exe
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
          override: true
      
      - name: Build Release
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Upload Release Asset
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          upload_url: ${{ github.event.release.upload_url }}
          asset_path: ./target/${{ matrix.target }}/release/${{ matrix.binary }}
          asset_name: wake-${{ matrix.target }}
          asset_content_type: application/octet-stream
```

## Version Numbering

Follow Semantic Versioning (SemVer):
- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes

## Pre-releases

For testing releases:
1. Use tags like `v1.0.0-beta.1`, `v1.0.0-rc.1`
2. Mark as "Pre-release" in GitHub
3. Don't set as "Latest release"

## Post-Release Checklist

- [ ] Verify all binaries are downloadable
- [ ] Test installation script
- [ ] Update documentation if needed
- [ ] Announce release on social media/forums
- [ ] Monitor issues for any problems
- [ ] Update homebrew formula (if applicable)

## Troubleshooting

### Cross-compilation Issues

For cross-compilation, install required targets:
```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-msvc
```

### Binary Signing (macOS)

For macOS releases, consider signing:
```bash
codesign --sign "Developer ID Application: Your Name" wake
```

### Linux Dependencies

Ensure static linking for Linux:
```toml
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]
```

## Support

For help with releases:
- Open an issue on GitHub
- Contact the maintainers

---

Built with ❤️ by Wind