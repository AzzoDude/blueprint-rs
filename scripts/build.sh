#!/bin/bash
# Move to project root
cd "$(dirname "$0")/.."

echo -e "\e[36m[BLUEPRINT-RS] Starting Cross-Platform Build...\e[0m"

# 1. Handle Virtual Env
if [ ! -d ".venv" ]; then
    echo -e "\e[33m[*] Creating .venv...\e[0m"
    python3 -m venv .venv
fi

# 2. Activate
source .venv/bin/activate

# 3. Handle Maturin
if ! command -v maturin &> /dev/null; then
    echo -e "\e[33m[*] Installing maturin...\e[0m"
    pip install maturin
fi

# 4. Build
if maturin develop --release; then
    echo -e "\e[32m[SUCCESS] Build complete!\e[0m"
else
    echo -e "\e[31m[ERROR] Build failed.\e[0m"
    exit 1
fi