Scanne du disque pour chercher les éventuels packages npm compromis.
L'exécutable s'occupe d'aller chercher automatiquement le fichier de configuration des npm malveillants depuis le repository git.
Ce fichier sera mis à jour régulièrement et vous n'avez rien d'autre à faire que de lancer l'outil.

L'exécutable lance une recherche sur C:\ ou / suivant la plateforme, windows ou linux

Pour préciser le chemin

./npm_scan.exe "D:\test"
ou
./npm_scan "/home"
