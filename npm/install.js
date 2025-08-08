#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { spawn } = require('child_process');

const RELEASE_VERSION = 'v0.1.3';
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

// Download file from URL
function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading Wake from ${url}...`);
    
    const file = fs.createWriteStream(dest);
    
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        https.get(response.headers.location, (response) => {
          if (response.statusCode !== 200) {
            reject(new Error(`Failed to download: ${response.statusCode}`));
            return;
          }
          
          const totalSize = parseInt(response.headers['content-length'], 10);
          let downloadedSize = 0;
          
          response.on('data', (chunk) => {
            downloadedSize += chunk.length;
            const progress = ((downloadedSize / totalSize) * 100).toFixed(1);
            process.stdout.write(`\rDownloading... ${progress}%`);
          });
          
          response.pipe(file);
          
          file.on('finish', () => {
            file.close();
            console.log('\nDownload complete!');
            resolve();
          });
        }).on('error', reject);
      } else if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      } else {
        const totalSize = parseInt(response.headers['content-length'], 10);
        let downloadedSize = 0;
        
        response.on('data', (chunk) => {
          downloadedSize += chunk.length;
          const progress = ((downloadedSize / totalSize) * 100).toFixed(1);
          process.stdout.write(`\rDownloading... ${progress}%`);
        });
        
        response.pipe(file);
        
        file.on('finish', () => {
          file.close();
          console.log('\nDownload complete!');
          resolve();
        });
      }
    }).on('error', reject);
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
    // Download the binary
    await downloadFile(downloadUrl, binPath);
    
    // Make it executable on Unix-like systems
    if (process.platform !== 'win32') {
      fs.chmodSync(binPath, 0o755);
      console.log('✅ Made binary executable');
    }
    
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