# Publishing Wake to npm

This guide will help you publish the Wake CLI tool to npm so users can install it with a single command.

## 📋 Prerequisites

1. **Node.js and npm installed** (version 14 or higher)
2. **npm account** - Create one at https://www.npmjs.com/signup
3. **Access to publish under @trywind scope** (optional - can publish under personal account)

## 🚀 Quick Publish

If you're already set up with npm:

```bash
cd npm
npm login  # If not already logged in
npm publish --access public
```

## 📝 Step-by-Step Guide

### 1. Install Node.js and npm

If not already installed:
- **Windows/macOS**: Download from https://nodejs.org/
- **Linux**: 
  ```bash
  curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
  sudo apt-get install -y nodejs
  ```

### 2. Create npm Account

If you don't have an npm account:
```bash
npm adduser
# Follow the prompts to create an account
```

### 3. Login to npm

```bash
npm login
# Enter your username, password, and email
```

### 4. Navigate to npm Directory

```bash
cd Wake/npm
```

### 5. Publish the Package

#### Option A: Use the publish script (Recommended)
```bash
./publish.sh
```

#### Option B: Manual publish
```bash
# Dry run first (to check everything is OK)
npm publish --dry-run

# If everything looks good, publish for real
npm publish --access public
```

## 🏢 Publishing Under Organization (Optional)

If you want to publish under the @trywind organization:

### 1. Create the Organization

1. Go to https://www.npmjs.com/org/create
2. Create an organization named "trywind"
3. Add team members if needed

### 2. Update Package Name (if needed)

The package is already configured as `@trywind/wake`. If you want to use a different org or personal account:

Edit `package.json`:
```json
{
  "name": "wake-cli",  // For personal account
  // OR
  "name": "@yourorg/wake",  // For your organization
}
```

### 3. Publish

```bash
npm publish --access public
```

## 🔄 Updating the Package

When you release a new version:

### 1. Update Version Number

Edit `npm/package.json`:
```json
{
  "version": "0.1.4",  // Increment version
}
```

Also update the version in `npm/install.js`:
```javascript
const RELEASE_VERSION = 'v0.1.4';  // Match GitHub release tag
```

### 2. Update Binary URLs

Make sure the GitHub release exists with the new binaries, then update `install.js` if needed.

### 3. Publish Update

```bash
cd npm
npm publish
```

## 🧪 Testing Before Publishing

### Test Local Installation

```bash
cd npm
npm install -g .  # Install locally
wake --help       # Test it works
npm uninstall -g @trywind/wake  # Clean up
```

### Test the Install Script

```bash
cd npm
node install.js  # This will download and install the binary
```

## 🎯 What Happens After Publishing?

Once published, users can install Wake with:

```bash
# If published as @trywind/wake
npm install -g @trywind/wake

# If published under personal account
npm install -g wake-cli
```

The package will:
1. Download automatically when users run `npm install`
2. Detect their OS and architecture
3. Download the appropriate binary from GitHub releases
4. Set up the `wake` command globally
5. Make the binary executable (on Unix systems)

## 📊 Package Statistics

After publishing, you can view statistics at:
- Package page: https://www.npmjs.com/package/@trywind/wake
- Download stats: https://npm-stat.com/charts.html?package=@trywind/wake

## 🛠️ Troubleshooting

### "Package name too similar to existing package"
- Try a different name or add a scope (@yourname/wake)

### "You do not have permission to publish"
- Make sure you're logged in: `npm whoami`
- For scoped packages, you might need to create the organization first

### "402 Payment Required"
- Scoped packages must be published with `--access public` for free accounts

### Binary download fails during installation
- Check that the GitHub release exists with the correct tag
- Verify the binary URLs in `install.js` are correct

## 📚 Additional Resources

- [npm Documentation](https://docs.npmjs.com/)
- [Publishing Packages](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry)
- [Package Scopes](https://docs.npmjs.com/about-scopes)

## 🎉 Success!

Once published, Wake will be available to millions of developers worldwide through npm!

Users will be able to install it with just:
```bash
npm install -g @trywind/wake
wake --help
```

---

Need help? Open an issue at https://github.com/Try-Wind/Wake/issues