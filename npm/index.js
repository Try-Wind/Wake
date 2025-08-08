/**
 * Wake CLI - A hardware-first coding agent by Wind
 * 
 * This package provides the Wake CLI tool for interacting with hardware
 * devices and assisting in embedded systems development.
 * 
 * Installation:
 *   npm install -g @trywind/wake
 * 
 * Usage:
 *   wake --help
 *   wake auth
 *   wake
 * 
 * For more information, visit:
 *   https://github.com/Try-Wind/Wake
 */

module.exports = {
  name: 'wake',
  version: '0.1.3',
  description: 'Wake - A hardware-first coding agent by Wind',
  
  // This package is primarily a CLI tool
  // The actual binary is installed separately
  // This file exists mainly for package metadata
  
  /**
   * Get the path to the Wake binary
   * @returns {string} Path to the Wake executable
   */
  getBinaryPath: function() {
    const path = require('path');
    const binaryName = process.platform === 'win32' ? 'wake.exe' : 'wake';
    return path.join(__dirname, 'bin', binaryName);
  },
  
  /**
   * Check if Wake is properly installed
   * @returns {boolean} True if the binary exists
   */
  isInstalled: function() {
    const fs = require('fs');
    try {
      return fs.existsSync(this.getBinaryPath());
    } catch {
      return false;
    }
  }
};