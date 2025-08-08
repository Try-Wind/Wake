# Wake npm Package Troubleshooting Guide

## Common Issues and Solutions

### 1. Binary Not Found After Installation

**Symptom:** After running `npm install -g @trywind/wake`, the `wake` command shows "binary not found".

**Solutions:**

#### Option A: Auto-download on first run (v0.1.5+)
Just run `wake` - it will automatically download the binary if missing:
```bash
wake
# The binary will be downloaded automatically on first run
```

#### Option B: Manual download command
```bash
# Navigate to the global npm modules directory
cd $(npm root -g)/@trywind/wake

# Run the download script
node download-wake.js
```

#### Option C: Complete reinstall
```bash
# Uninstall
npm uninstall -g @trywind/wake

# Clear npm cache
npm cache clean --force

# Reinstall with verbose logging
npm install -g @trywind/wake --verbose
```

### 2. Postinstall Script Not Running

**Symptom:** The installation completes but no binary is downloaded.

**Cause:** npm might skip postinstall scripts for security reasons, especially in corporate environments.

**Solutions:**

1. **Enable scripts globally:**
   ```bash
   npm config set ignore-scripts false
   ```

2. **Run install with scripts enabled:**
   ```bash
   npm install -g @trywind/wake --unsafe-perm
   ```

3. **Manual binary download:**
   ```bash
   # After installation, manually trigger the download
   cd $(npm root -g)/@trywind/wake
   npm run download
   ```

### 3. Windows-Specific Issues

#### "spawn UNKNOWN" Error
**Solution:** Update to version 0.1.5 or later:
```bash
npm update -g @trywind/wake
```

#### Permission Denied
**Solution:** Run Command Prompt or PowerShell as Administrator:
```bash
# In Administrator mode
npm install -g @trywind/wake
```

### 4. Corporate Proxy/Firewall Issues

**Symptom:** Cannot download binary from GitHub.

**Solutions:**

1. **Configure npm proxy:**
   ```bash
   npm config set proxy http://your-proxy:port
   npm config set https-proxy http://your-proxy:port
   ```

2. **Manual download:**
   - Download the binary manually from: https://github.com/Try-Wind/Wake/releases/latest
   - Place it in: `$(npm root -g)/@trywind/wake/bin/`
   - Rename to `wake.exe` (Windows) or `wake` (Linux/Mac)

### 5. macOS Issues

**Current Status:** macOS binaries are not yet available.

**Workaround:** Build from source:
```bash
git clone https://github.com/Try-Wind/Wake.git
cd Wake
cargo install --path wake-cli
```

## Debugging Commands

### Check Installation Path
```bash
# Find where Wake is installed
npm list -g @trywind/wake

# Navigate to installation directory
cd $(npm root -g)/@trywind/wake
```

### Enable Debug Mode
```bash
# Windows
set WAKE_DEBUG=1
wake

# Linux/Mac
WAKE_DEBUG=1 wake
```

### Check Binary Existence
```bash
# Windows PowerShell
Test-Path "$env:APPDATA\npm\node_modules\@trywind\wake\bin\wake.exe"

# Linux/Mac
ls -la $(npm root -g)/@trywind/wake/bin/
```

### Manual Binary Installation
```bash
# 1. Download the correct binary from GitHub
# Linux: wake-linux-x64
# Windows: wake-windows-x64.exe

# 2. Find npm global directory
npm root -g

# 3. Copy binary to the correct location
# Windows example:
copy wake-windows-x64.exe "%APPDATA%\npm\node_modules\@trywind\wake\bin\wake.exe"

# Linux example:
cp wake-linux-x64 $(npm root -g)/@trywind/wake/bin/wake
chmod +x $(npm root -g)/@trywind/wake/bin/wake
```

## Version History

- **v0.1.5**: Auto-download on first run, better error handling
- **v0.1.4**: Fixed Windows compatibility issues
- **v0.1.3**: Initial npm release

## Still Having Issues?

1. **Check GitHub Issues:** https://github.com/Try-Wind/Wake/issues
2. **Report New Issue:** Include:
   - Your OS and version
   - Node.js version: `node --version`
   - npm version: `npm --version`
   - Error messages
   - Output of `WAKE_DEBUG=1 wake`

## Alternative Installation Methods

If npm installation continues to fail:

### Direct Binary Download
1. Go to: https://github.com/Try-Wind/Wake/releases/latest
2. Download the appropriate binary for your platform
3. Add to your system PATH

### Build from Source
```bash
# Requirements: Rust and Cargo
git clone https://github.com/Try-Wind/Wake.git
cd Wake
cargo install --path wake-cli
```

---

**Note:** The npm package is designed to simplify installation, but direct binary download or building from source are always available as fallback options.