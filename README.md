# NPM Scanner - Détecteur de packages malveillants

Un outil multiplateforme écrit en Rust pour détecter les packages npm malveillants dans les fichiers `package-lock.json`.

## Fonctionnalités

- Scan récursif des fichiers `package-lock.json`
- Détection de packages malveillants basée sur une liste configurable
- Vérification croisée avec les fichiers `package.json`
- Logs détaillés avec horodatage
- Support Windows et Linux

## Compilation

### Windows (PowerShell)

```powershell
.\build.ps1
```

### Linux/Unix (Bash)

```bash
chmod +x build.sh
./build.sh
```

### Compilation manuelle

```bash
cargo build --release
```

## Utilisation

### Windows

```cmd
# Scanner depuis C:\ (par défaut)
npm_scan.exe

# Scanner un répertoire spécifique
npm_scan.exe "C:\mon\projet"
```

### Linux

```bash
# Scanner depuis / (par défaut)
./npm_scan

# Scanner un répertoire spécifique
./npm_scan "/home/user/projets"
```

## Configuration

Le fichier `malicious_packages.json` doit être présent dans le même répertoire que l'exécutable. Il contient la liste des packages malveillants à détecter.

Format :

```json
{
  "malicious_packages": {
    "nom_du_package": ["version1", "version2"],
    "autre_package": ["version_malveillante"]
  }
}
```

## Sorties

- **Console** : Affichage en temps réel avec codes couleur
- **Fichier log** : `scan_log_YYYYMMDD_HHMMSS.txt` dans le répertoire de l'exécutable

## Codes couleur

- 🔴 **Rouge** : Infection confirmée (package.json vérifié)
- 🟡 **Jaune** : Infection détectée mais non confirmée dans package.json

## Dépannage

### Erreur "cannot execute: required file not found" sur Linux

Si vous obtenez cette erreur lors de l'exécution de `./build.sh` sur Linux, cela est dû aux fins de ligne Windows. Voici comment la corriger :

```bash
# Option 1: Convertir les fins de ligne avec dos2unix
sudo apt-get install dos2unix  # Ubuntu/Debian
dos2unix build.sh
chmod +x build.sh
./build.sh

# Option 2: Utiliser sed pour convertir les fins de ligne
sed -i 's/\r$//' build.sh
chmod +x build.sh
./build.sh

# Option 3: Compilation directe sans script
cargo build --release
cp target/release/rust_npm_scan ../RELEASE/npm_scan
cp malicious_packages.json ../RELEASE/
```
