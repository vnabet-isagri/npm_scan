Scanne du disque pour chercher les éventuels packages npm compromis.
La liste des packages à rechercher est configurée dans le fichier "malicious_packages.json".

L'exectutable lance une recherche sur C:\ ou / suivant la plateforme, windows ou linux

Pour préciser le chemin

./npm_scan.exe "D:\test"
ou
./npm_scan "/home"
