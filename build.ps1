# Script de build multiplateforme pour npm_scan (Windows)

Write-Host "Building npm_scan for Windows..." -ForegroundColor Green

# Build en mode release
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "Build successful!" -ForegroundColor Green
    
    # Copier le fichier de configuration
    Copy-Item "malicious_packages.json" "target\release\"
    
    # Créer le dossier RELEASE s'il n'existe pas
    if (!(Test-Path ".\RELEASE")) {
        New-Item -ItemType Directory -Path ".\RELEASE"
    }
    
    # Copier l'exécutable vers le dossier RELEASE
    Copy-Item "target\release\rust_npm_scan.exe" ".\RELEASE\npm_scan.exe"
    Write-Host "Windows executable copied to RELEASE/npm_scan.exe" -ForegroundColor Yellow
    
    Copy-Item "malicious_packages.json" ".\RELEASE\"
    Copy-Item "README.txt" ".\RELEASE\"
    Write-Host "Configuration file and README copied to RELEASE/" -ForegroundColor Yellow
} else {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}
