# 🚀 npm Package Update Instructions

## Quick Update (if already published before)

```bash
cd Wake/npm
npm publish
```

That's it! The package will be updated to version 0.1.4.

## First Time Publishing

If this is your first time publishing:

```bash
# 1. Login to npm
npm login

# 2. Navigate to npm directory
cd Wake/npm

# 3. Publish the package
npm publish --access public
```

## What's Been Fixed in v0.1.4

✅ **Windows Compatibility Fixed**
- The "spawn UNKNOWN" error on Windows has been resolved
- Binary wrapper now properly handles Windows process spawning
- Better error messages for debugging

✅ **Improved Installation**
- Better binary download verification
- Shows binary size after download
- Skips re-downloading if binary already exists
- Better error handling for network issues

✅ **Enhanced Debugging**
- Set `WAKE_DEBUG=1` environment variable for verbose output
- Shows binary path and existence status on errors

## Testing the Fix

After publishing, test on Windows:

```bash
# Uninstall old version
npm uninstall -g @trywind/wake

# Install new version
npm install -g @trywind/wake

# Test it works
wake --help
```

## Auto-Update Information

**For users who already have the package installed:**
- They need to manually update: `npm update -g @trywind/wake`
- npm does NOT auto-update global packages
- Consider notifying users about the update

**To check current npm version:**
```bash
npm view @trywind/wake version
```

## Troubleshooting

If you get any errors during publishing:

1. **"You cannot publish over the previously published versions"**
   - The version has already been published
   - Bump the version in package.json to 0.1.5

2. **"402 Payment Required"**
   - Add `--access public` flag when publishing

3. **"You do not have permission"**
   - Make sure you're logged in: `npm whoami`

## After Publishing

1. Verify the package: https://www.npmjs.com/package/@trywind/wake
2. Test installation on a clean system
3. Update any documentation if needed

---

**Current Status:**
- GitHub Release: ✅ v0.1.4 created with binaries
- npm Package: 📦 Ready to publish v0.1.4
- Binaries: ✅ Linux and Windows available