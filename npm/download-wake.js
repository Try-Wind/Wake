#!/usr/bin/env node

/**
 * Manual download script for Wake binary
 * Can be run directly: npx @trywind/wake download
 */

const installScript = require('./install.js');

// Run the installation
console.log('🚀 Downloading Wake binary...\n');

if (typeof installScript === 'function') {
  installScript();
} else {
  // If install.js doesn't export a function, run it directly
  require('./install.js');
}