use std::collections::{HashMap, HashSet};
use std::fs;
use walkdir::WalkDir;
use serde::Deserialize;
use std::io::Write;
use chrono::Local;
use reqwest;
use std::error::Error;

#[derive(Deserialize)]
struct PackageLock {
    packages: Option<HashMap<String, PackageInfo>>,
}

#[derive(Deserialize)]
struct PackageInfo {
    version: Option<String>,
}

#[derive(Deserialize)]
struct MaliciousConfig {
    malicious_packages: HashMap<String, HashSet<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Définir le répertoire racine par défaut selon la plateforme
    let default_root = if cfg!(windows) {
        "C:\\"
    } else {
        "/"
    };

    let root_dir = std::env::args().nth(1).unwrap_or(default_root.to_string());

    // Récupérer le chemin de l'exécutable
    let exe_path = std::env::current_exe().expect("Impossible de récupérer le chemin de l'exécutable");
    let exe_dir = exe_path.parent().expect("Impossible de récupérer le dossier de l'exécutable");

    // Remplacez la lecture locale par un téléchargement HTTP
    let url = "https://raw.githubusercontent.com/vnabet-isagri/npm_scan/refs/heads/main/malicious_packages.json";
    let response = reqwest::blocking::get(url)?;
    let config_content = response.text()?;
    // Utilisez config_content comme avant (ex: serde_json::from_str(&config_content)?)
    let config: MaliciousConfig = serde_json::from_str(&config_content)
        .expect("Erreur de parsing du fichier de configuration");

    let malicious_packages = config.malicious_packages;

    // Préparer le fichier de log daté
    let log_filename = format!(
        "scan_log_{}.txt",
        Local::now().format("%Y%m%d_%H%M%S")
    );
    let log_path = exe_dir.join(log_filename);
    let mut log_file = fs::File::create(&log_path)
        .expect("Impossible de créer le fichier de log");

    // Fonction utilitaire pour écrire dans la console et le log (sans couleur dans le log)
    fn log_print(log_file: &mut fs::File, msg: &str, color: Option<&str>) {
        if let Some(c) = color {
            print!("{}{}{}", c, msg, "\x1b[0m");
        } else {
            print!("{}", msg);
        }
        // Supprimer les codes couleurs ANSI pour le log
        let clean_msg = msg.replace("\x1b[31m", "")
                           .replace("\x1b[33m", "")
                           .replace("\x1b[0m", "");
        let _ = writeln!(log_file, "{}", clean_msg);
    }

    log_print(&mut log_file, &format!("\n[+] Début d’analyse des fichiers package-lock.json sur {} ...\n", root_dir), None);

    // Afficher le nom de la machine
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "inconnu".to_string());
    log_print(&mut log_file, &format!("Nom de la machine : {}\n", hostname), None);

    for entry in WalkDir::new(&root_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == "package-lock.json" {
            let lock_path = entry.path();
            let content = match fs::read_to_string(lock_path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let lock_file: PackageLock = match serde_json::from_str(&content) {
                Ok(lf) => lf,
                Err(_) => {
                    println!("[!] Erreur de parsing : {:?}", lock_path);
                    continue;
                }
            };
            if let Some(packages) = lock_file.packages {
                for (pkg_path, info) in packages.iter() {
                    let pkg_name = pkg_path.split("node_modules/").last().unwrap_or(pkg_path);
                    if let Some(versions) = malicious_packages.get(pkg_name) {
                        if let Some(version) = &info.version {
                            if versions.contains(version.as_str()) {
                                // Vérification package.json
                                let internal_path = if cfg!(windows) {
                                    pkg_path.replace("/", "\\")
                                } else {
                                    pkg_path.to_string()
                                };
                                let package_dir = entry.path().parent().unwrap().join(&internal_path);
                                let package_json_path = package_dir.join("package.json");
                                let confirmed = if package_json_path.exists() {
                                    if let Ok(pkg_json_content) = fs::read_to_string(&package_json_path) {
                                        if let Ok(pkg_json) = serde_json::from_str::<serde_json::Value>(&pkg_json_content) {
                                            pkg_json.get("version").and_then(|v| v.as_str()) == Some(version)
                                        } else { false }
                                    } else { false }
                                } else { false };

                                if confirmed {
                                    log_print(&mut log_file, "\n Infection confirmée !\n", Some("\x1b[31m"));
                                    log_print(&mut log_file, &format!("    Fichier lock    : {}\n", lock_path.display()), None);
                                    log_print(&mut log_file, &format!("    Package         : {}\n", pkg_name), None);
                                    log_print(&mut log_file, &format!("    Version trouvée : {}\n", version), None);
                                    log_print(&mut log_file, &format!("    Chemin interne  : {}\n", pkg_path), None);
                                    log_print(&mut log_file, "    Version package.json confirmée.\n", None);
                                } else {
                                    log_print(&mut log_file, "\n Infection détectée (non confirmée dans package.json) !\n", Some("\x1b[33m"));
                                    log_print(&mut log_file, &format!("    Fichier lock    : {}\n", lock_path.display()), None);
                                    log_print(&mut log_file, &format!("    Package         : {}\n", pkg_name), None);
                                    log_print(&mut log_file, &format!("    Version trouvée : {}\n", version), None);
                                    log_print(&mut log_file, &format!("    Chemin interne  : {}\n", pkg_path), None);
                                    log_print(&mut log_file, "    package.json absent ou version différente.\n", None);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    log_print(&mut log_file, "\n[OK] Analyse terminée.\n", None);

    // Attendre une entrée utilisateur pour garder la fenêtre ouverte
    println!("Appuyez sur Entrée pour quitter...");
    let _ = std::io::stdin().read_line(&mut String::new());

    Ok(())
}
