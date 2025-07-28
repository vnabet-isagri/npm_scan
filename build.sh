#!/bin/bash

# Script de build multiplateforme pour npm_scan

echo "Building npm_scan for current platform..."

# Build en mode release
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    
    # Copier le fichier de configuration
    cp malicious_packages.json target/release/
    
    # Copier l'exécutable vers le dossier RELEASE
    mkdir -p ./RELEASE
    
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        # Windows
        cp target/release/rust_npm_scan.exe ./RELEASE/npm_scan.exe
        echo "Windows executable copied to RELEASE/npm_scan.exe"
    else
        # Linux/Unix
        cp target/release/rust_npm_scan ./RELEASE/npm_scan
        chmod +x ./RELEASE/npm_scan
        echo "Linux executable copied to RELEASE/npm_scan"
    fi
    
    cp malicious_packages.json ./RELEASE/
    cp README.txt ./RELEASE/
    echo "Configuration file and README copied to RELEASE/"
else
    echo "Build failed!"
    exit 1
fi
