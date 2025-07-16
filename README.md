# Projet_LOG430

Cette application comporte deux bases de données distinctes, correspondant à différents niveaux couches.
Une couche correspond à la base de données du magasin local, et l’autre à la base de données du serveur mère. Dans l'application du magasin, le client peut rechercher des produits disponibles en magasin, acheter des produits, consulter la liste complète des produits, ainsi que retourner une vente. De plus, le magasin local met à jour ses inventaires dans la base de données mère afin que, lorsqu’un gestionnaire consulte un rapport, les données soient à jour. Le gestionnaire peut également visualiser les performances des magasins à l’aide d’un tableau de bord. Les employés du magasin, en tant qu’administrateurs, peuvent envoyer une demande d'approvisionnement pour un produit via l’application dans la catégorie `consultation`.


## Prérequis
Avant de commencer, assurez-vous d'avoir les éléments suivants :
- Python 3.x
- PostgreSQL

## Installation
1. Cloner le repo
```
git clone https://github.com/Barlow-Personal-Git/LOG430_Barlow_Etape_1.git
```
2. Créer et activer un environnement virtuel:
```
python3 -m venv env
source env/bin/activate
```

Note : Si vous avez besoin de déactive
```
deactive
```

3. Installer les dépendances:
```
pip install -r requirements.txt
```

4. Configurer PostgreSQL
Créer une base de donnée PostgreSQL
```
psql -U postgres
CREATE DATABASE log430_lab;
```

5. Configurer les variables d'environnement
Copier le fichier d'exemple et renommez-le :
```
cp .env-example .env
```
Ensuite, ouvrez le fichier `.env` et modifiez la variable `DATABASE_URL` :
- Remplacez `user` par le nom de votre utilisateur
- Remplacez `password` par le mot de passe de cet utilisateur
- Remplez `table` par le nom de votre base de données (ex.: log430_lab)

Note: la base de donnée mère est aussi dans ce fichier et le nom du magasin.

6. Création des tables dans la base de données
L’exécution pour la première fois crée les tables.
```
python3 app.py
ctrl + c
```

7. Seed de la base de données
```
python3 -m seed.run_seed
```

## Exécution
1. Exécuter le programme
```
python3 app.py
```

## Serveur mère

1. Configurer les variables d'environnement
Copier le fichier d'exemple et renommez-le :
```
cd main_server
cp .env-example .env
```

2. Configurer PostgreSQL
Créer une base de donnée PostgreSQL
```
psql -U postgres
CREATE DATABASE log430_lab_mere;
```

3. Configurer PostgreSQL
Créer une base de donnée PostgreSQL pour la serveur mère
```
psql -U postgres
createdb -U user log430_lab_mere
```

4. Créaction des tables
Exécuter pour la première fois le programme. Assurez-vous que vous êtes n'est plus dans `main_server/`
```
python3 -m main_server.server 
```

5. Ajouter les données seed 
```
python3 -m main_server.ed.run_seed
```

## Exécution du serveur mère
1. Exécuter le programme
```
python3 -m main_server.server 
```
Note : Il est important que ce serveur reste toujours ouvert afin que la base de données mère soit constamment à jour et que les rapports générés contiennent les dernières données.

## Générer un rapport consolidé des ventes
Pour générer le rapport, accédez à l'URL suivante dans votre navigateur web :
```
http://127.0.0.1:5000/rapport
```
Notes : Le fichier PDF sera sauvegardé dans le dossier contenant le code source.

## Générer un rapport consolidé des ventes
Pour générer le rapport, accédez à l'URL suivante dans votre navigateur web :
```
http://127.0.0.1:5000/rapport
```
Notes : Le fichier PDF sera sauvegardé dans le dossier contenant le code source.

## Visualiser les performances des magasins dans un tableau de bord
Pour les performances des magasins dans un tableau de bord, accédez à l'URL suivante dans votre navigateur web :
```
http://127.0.0.1:5000/dashboard
```
Notes : Assurez-vous que vous avez faites des achats avec l'exécution principal


## Construire et lancer le conteneur à l'aide de Docker Compose

Pour créer l'image Docker à l'aide de la configuration `docker-compose.yml` et du fichier `Dockerfile`, exécutez la commande suivante dans le terminal : 

```bash
docker compose build
```
Cette commande crée une image Docker contenant l'application et ses dépendances.

Ensuite, pour lancer l'application dans un conteneur Docker, utilisez la commande suivante : 
```bash
docker compose up
```
Cette commande permet de lancer le conteneur basé sur l'image constuite et affiche les logs de l'application.


## Illustration d'une exécusion réussie de la pipeline CI/CD
![Illustration d'une exécusion réussie de la pipeline CI/CD](images/image-1.png)

## Illustration d'une image créée automatiquement avec Docker
![Illustration d'une image créée automatiquement avec Docker](images/image.png)

# La structure de l'application

L’organisation de cette application suit une approche MVC (Modèle-Vue-Contrôleur), inspirée par des frameworks comme Laravel et ASP.NET MVC, adaptée à un contexte Python/Flask.

Arborescence et rôles des dossiers principaux
`app/` : Contient l’ensemble des éléments backend liés à la logique métier et aux traitements :
- `controllers/` : gestion des contrôleurs.
- `models/` : définition des modèles de données et accès à la base.
- `routes/` (à venir) : gestion des routes HTTP et API.
- `api/` (à venir) : gestion des requêtes API externes ou internes.

`docs/` : Documentation technique regroupant :
- Diagrammes UML 4+1 (architecture logique, processus, déploiement…).
- ADR (Architecture Decision Records) détaillant les choix techniques.

`github/` : Contient les workflows et fichiers de configuration pour l’intégration continue et le déploiement automatisé.

`seed/` : Données initiales pour la base, permettant le peuplement automatique lors de l’installation ou du développement.

`tests/` : Tests unitaires et fonctionnels assurant la qualité et la stabilité du code.

`views/` : Partie Vue du MVC, destinée à l’affichage et à l’interaction utilisateur.
Note : Pour l’instant, ce dossier contient principalement les fonctions d’entrée (input) et d’affichage (print).

`main_server` : Dossier dédié au serveur mère. Sa structure actuelle est fonctionnelle, mais elle sera améliorée prochainement pour adopter une architecture MVC complète.

## Point d'exécution
`app.py` :Programme principal pour l’application magasin local.
`main_server/server.py` : Programme principal pour le serveur mère, responsable de la base de données centrale et des rapports consolidés.

# Choix des applications

`Python` : Parmi tous les langages que j'ai utilisés au fil des années, Python est celui avec lequel j'ai le moins d'expérience, en excluant Rust. Étant donné que de nombreuses entreprises demandent aujourd’hui des compétences en Python, il est pertinent pour moi de me familiariser davantage avec ce langage à travers ce projet.

`Console CLI` : Initialement, je prévoyais de créer une application web complète. Cependant, la ligne de commande (CLI) permet de simplifier la majorité des tâches.

`Github` : Je suis plus familier avec GitHub qu’avec GitLab, tant pour le versionnage que pour l’intégration de workflows CI/CD.

`pylint` : C'est un outil proposé pour l’analyse statique de code afin de maintenir un bon niveau de qualité, détecter les erreurs potentielles et appliquer un style de code..

`pytest` : Un framework simple pour effectuer les tests unitaires.

`SQLAlchemy` : C'est un ORM pour interagir avec la base de données.

`PostgreSQL` : C’est une base de données relationnelle simple à utiliser et plus cohérente avec les besoins du projet qu’une base de données NoSQL.


