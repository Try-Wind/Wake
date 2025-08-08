#!/bin/bash

# Wake npm Publishing Script
# This script helps publish the Wake npm package

set -e

echo "🚀 Wake npm Publishing Script"
echo "=============================="
echo ""

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install Node.js and npm first."
    exit 1
fi

# Check if user is logged in to npm
echo "📝 Checking npm login status..."
npm_user=$(npm whoami 2>/dev/null || echo "")

if [ -z "$npm_user" ]; then
    echo "❌ You are not logged in to npm."
    echo ""
    echo "Please login to npm first:"
    echo "  npm login"
    echo ""
    echo "Or if you need to create an account:"
    echo "  npm adduser"
    echo ""
    exit 1
fi

echo "✅ Logged in as: $npm_user"
echo ""

# Check package name availability
echo "🔍 Checking if package name is available..."
if npm view @trywind/wake &> /dev/null; then
    echo "⚠️  Package @trywind/wake already exists on npm"
    echo ""
    read -p "Do you want to publish a new version? (y/n): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Publishing cancelled"
        exit 1
    fi
    
    # Get current version
    current_version=$(npm view @trywind/wake version)
    echo "Current version on npm: $current_version"
    echo "Version in package.json: $(grep '"version"' package.json | cut -d'"' -f4)"
    echo ""
    echo "⚠️  Make sure to update the version in package.json before publishing!"
    read -p "Continue with publish? (y/n): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Publishing cancelled"
        exit 1
    fi
else
    echo "✅ Package name @trywind/wake is available!"
fi

echo ""
echo "📦 Publishing package to npm..."
echo "================================"

# Dry run first
echo "Running dry-run to check for issues..."
npm publish --dry-run

echo ""
read -p "Dry run complete. Proceed with actual publish? (y/n): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Publishing cancelled"
    exit 1
fi

# Actual publish
echo "🚀 Publishing to npm..."
npm publish --access public

echo ""
echo "✨ Success! Package published to npm!"
echo ""
echo "📦 Package URL: https://www.npmjs.com/package/@trywind/wake"
echo ""
echo "Users can now install Wake with:"
echo "  npm install -g @trywind/wake"
echo ""
echo "🎉 Congratulations!"