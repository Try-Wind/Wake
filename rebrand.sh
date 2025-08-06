#!/bin/bash

# Replace in all Rust files
find . -type f -name "*.rs" -exec sed -i 's/wake-macros/wake-macros/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/wake-llm/wake-llm/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/wake-core/wake-core/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/wake-cli/wake-cli/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/wake_macros/wake_macros/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/wake_llm/wake_llm/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/wake_core/wake_core/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/wake_cli/wake_cli/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/\.wake\.config/.wake.config/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/Wake/Wake/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/wake/wake/g' {} \;
find . -type f -name "*.rs" -exec sed -i 's/WAKE/WAKE/g' {} \;

# Replace in all Markdown files
find . -type f -name "*.md" -exec sed -i 's/Wake/Wake/g' {} \;
find . -type f -name "*.md" -exec sed -i 's/wake/wake/g' {} \;
find . -type f -name "*.md" -exec sed -i 's/WAKE/WAKE/g' {} \;
find . -type f -name "*.md" -exec sed -i 's/OVHcloud/Wind/g' {} \;
find . -type f -name "*.md" -exec sed -i 's/OVH/Wind/g' {} \;

# Replace in shell scripts
find . -type f -name "*.sh" -exec sed -i 's/wake/wake/g' {} \;
find . -type f -name "*.sh" -exec sed -i 's/Wake/Wake/g' {} \;
find . -type f -name "*.sh" -exec sed -i 's/WAKE/WAKE/g' {} \;

# Replace in YAML files
find . -type f -name "*.yml" -exec sed -i 's/wake/wake/g' {} \;
find . -type f -name "*.yml" -exec sed -i 's/Wake/Wake/g' {} \;
find . -type f -name "*.yml" -exec sed -i 's/WAKE/WAKE/g' {} \;

echo "Rebranding complete!"