#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { spawn } = require('child_process');

const RELEASE_VERSION = 'v0.1.5';
const REPO = 'Try-Wind/Wake';

// Detect platform and architecture
function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;
  
  if (platform === 'darwin') {
    if (arch === 'x64') {
      return { binary: 'wake-macos-x64', supported: false, message: 'macOS x64 binary not yet available' };
    } else if (arch === 'arm64') {
      return { binary: 'wake-macos-arm64', supported: false, message: 'macOS ARM64 binary not yet available' };
    }
  } else if (platform === 'linux' && arch === 'x64') {
    return { binary: 'wake-linux-x64', supported: true };
  } else if (platform === 'win32' && arch === 'x64') {
    return { binary: 'wake-windows-x64.exe', supported: true };
  }
  
  return { 
    supported: false, 
    message: `Unsupported platform: ${platform} ${arch}. Please build from source.` 
  };
}

// Download file from URL with better error handling
function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading Wake from ${url}...`);
    
    // Delete existing file if it exists
    if (fs.existsSync(dest)) {
      fs.unlinkSync(dest);
    }
    
    const file = fs.createWriteStream(dest, { mode: 0o755 });
    
    const download = (downloadUrl) => {
      https.get(downloadUrl, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          // Follow redirect
          file.close();
          fs.unlinkSync(dest);
          return download(response.headers.location);
        } else if (response.statusCode !== 200) {
          file.close();
          fs.unlinkSync(dest);
          reject(new Error(`Failed to download: HTTP ${response.statusCode}`));
          return;
        }
        
        const totalSize = parseInt(response.headers['content-length'], 10);
        let downloadedSize = 0;
        
        response.on('data', (chunk) => {
          downloadedSize += chunk.length;
          if (totalSize) {
            const progress = ((downloadedSize / totalSize) * 100).toFixed(1);
            process.stdout.write(`\rDownloading... ${progress}%`);
          }
        });
        
        response.pipe(file);
        
        file.on('finish', () => {
          file.close(() => {
            console.log('\nDownload complete!');
            
            // Verify the file was written correctly
            if (!fs.existsSync(dest)) {
              reject(new Error('Binary file was not created'));
              return;
            }
            
            const stats = fs.statSync(dest);
            if (stats.size === 0) {
              fs.unlinkSync(dest);
              reject(new Error('Downloaded file is empty'));
              return;
            }
            
            console.log(`Binary size: ${(stats.size / 1024 / 1024).toFixed(2)} MB`);
            resolve();
          });
        });
        
        file.on('error', (err) => {
          file.close();
          fs.unlinkSync(dest);
          reject(err);
        });
      }).on('error', (err) => {
        file.close();
        if (fs.existsSync(dest)) {
          fs.unlinkSync(dest);
        }
        reject(err);
      });
    };
    
    download(url);
  });
}

// Main installation function
async function install() {
  console.log('Installing Wake CLI...\n');
  
  const platformInfo = getPlatform();
  
  if (!platformInfo.supported) {
    console.error(`\n❌ Error: ${platformInfo.message}`);
    console.log('\n📦 Alternative installation methods:');
    console.log('1. Build from source:');
    console.log('   git clone https://github.com/Try-Wind/Wake.git');
    console.log('   cd Wake && cargo install --path wake-cli');
    console.log('\n2. Download manually from:');
    console.log(`   https://github.com/${REPO}/releases/tag/${RELEASE_VERSION}`);
    
    if (platformInfo.binary && platformInfo.binary.includes('macos')) {
      console.log('\n🍎 Note: macOS binaries will be available in future releases.');
      console.log('   You can help by contributing to our GitHub Actions workflow!');
    }
    
    process.exit(1);
  }
  
  const binaryName = platformInfo.binary;
  const downloadUrl = `https://github.com/${REPO}/releases/download/${RELEASE_VERSION}/${binaryName}`;
  const binDir = path.join(__dirname, 'bin');
  const binPath = path.join(binDir, process.platform === 'win32' ? 'wake.exe' : 'wake');
  
  // Create bin directory if it doesn't exist
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }
  
  try {
    // Check if binary already exists and skip download if it does
    if (fs.existsSync(binPath)) {
      const stats = fs.statSync(binPath);
      if (stats.size > 1000000) { // Binary should be at least 1MB
        console.log('✅ Wake binary already exists, skipping download');
        console.log(`Binary size: ${(stats.size / 1024 / 1024).toFixed(2)} MB`);
      } else {
        console.log('⚠️  Existing binary seems corrupted, re-downloading...');
        await downloadFile(downloadUrl, binPath);
      }
    } else {
      // Download the binary
      await downloadFile(downloadUrl, binPath);
    }
    
    // Make it executable on Unix-like systems
    if (process.platform !== 'win32') {
      fs.chmodSync(binPath, 0o755);
      console.log('✅ Made binary executable');
    } else {
      // On Windows, ensure the file has proper attributes
      try {
        fs.chmodSync(binPath, 0o755);
      } catch (e) {
        // chmod might not work on Windows, that's okay
      }
    }
    
    // Verify the binary exists and is valid
    if (!fs.existsSync(binPath)) {
      throw new Error('Binary was not installed correctly');
    }
    
    const finalStats = fs.statSync(binPath);
    console.log(`\n✅ Binary installed: ${binPath}`);
    console.log(`   Size: ${(finalStats.size / 1024 / 1024).toFixed(2)} MB`);
    
    console.log('\n✨ Wake has been successfully installed!');
    console.log('\n📘 Usage:');
    console.log('   wake --help     Show help');
    console.log('   wake auth       Configure authentication');
    console.log('   wake            Start Wake CLI\n');
    
  } catch (error) {
    console.error('\n❌ Installation failed:', error.message);
    console.log('\n🔧 Troubleshooting:');
    console.log('1. Check your internet connection');
    console.log('2. Try installing from source instead:');
    console.log('   git clone https://github.com/Try-Wind/Wake.git');
    console.log('   cd Wake && cargo install --path wake-cli');
    console.log(`3. Download manually from: https://github.com/${REPO}/releases`);
    process.exit(1);
  }
}

// Check if running as part of npm install
if (require.main === module || process.env.npm_lifecycle_event === 'postinstall') {
  install().catch(error => {
    console.error('Installation error:', error);
    process.exit(1);
  });
}