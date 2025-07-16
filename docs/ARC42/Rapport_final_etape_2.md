Lien de mon repos : https://github.com/Barlow-Personal-Git/Projet_LOG430

# 1. Introduction et objectifs

Ce projet est une application de gestion d'inventaire pour un réseau de magasins, dans le cadre du cours LOG430. Elle permet de consulter, acheter, retourner des produits et synchroniser les inventaires locaux avec le centre logistique. La maison mère peut générer des rapports et consulter un tableau pour évoluer la performance des magasins. Ce laboratoire vise à familier les étudiants avec les outils nécessaires à la création d'un projet de haut niveau, tels que API Request, Grafana et Prometheus

# 2. Contraintes
- Langage principal : Rust
- Base de données : PostgreSQL
- Interfaces : CLI (ligne de commande) pour le magasin et application Web pour la maison mère  et le magasin en ligne 
- Framework web pour le serveur mère : Rocket
- Test : `cargo test`
- Style : `cargo fmt`
- Répertoire : Github

# 3. Principes architecturaux

Mon principe architectural repose sur le modèle MVC : 
- `models/` : Contient les structures représentant les entités de la base de données (ex.: produits, client, inventaires, transactions). 
- `templates/` ou `views` : Ce dossier est utilisé différemment selon l'interface. Pour le magasin, il sert à générer des phrases lisibles affichées dans le terminal. Pour la maison mère, il est utilisé avec Tera pour générer des pages HTML affichant le contenu. 
- `controllers/` : Contient les routes Rocket telles que Get et Post. Ils gèrent les requêtes et les interagissent avec les models. Ils retournent soit un JSON pour les api request ou soit des templates Tera.
L’application principale du laboratoire suit cette structure de manière organisée. L'application Centre Logistique ne suive pas cette architecture, puisqu'il n'a pas besoin d'un interface utilisateur.

Les fichiers sont organisés selon leur comportement ou rôle :
- Les documents d’architecture et les diagrammes sont dans le dossier `docs/`
- Les tests unitaires sont regroupés dans `tests/`
- Les données initiales (seed) sont placées dans `seed/`
  
# 4. Contexte

## Contexte fonctionnel
- Il y a deux bases de données : une locale (magasin) et une centrale
- Le client peut :
  - Rechercher un produit
  - Acheter un produit
  - Consulter l'inventaire
  - Retourner un vente

- Le serveur peut :
  - Obtenir un rapport
  - Consulter un tableau

- Les employeurs qui occupent le centre logistique peuvent consulter grafana et promeuteus pour voir les requêtes s'ils fonctionnent bien correctement.
  
TODO

## Contexte technique
- PostgreSQL pour les deux bases de données
- Exécution local avec CLI
- Serveur mère roule avec Flask

# 5. Scénarios d'utilisation

- UC1 – Rechercher un produit : Un client peut consulter les produits par identifiant, catégorie ou nom.
- UC2 – Acheter un produit : Un client peut ajouter des produits à une vente et confirmer son achat.
- UC3 – Retourner une vente : Un client peut retourner une vente effectuée précédemment dans le même magasin.
- UC4 – Consulter la liste des produits : Un client peut consulter l’inventaire complet du magasin.
- UC5 – Déclencher un réapprovisionnement : Un employé du magasin peut envoyer une demande de réapprovisionnement.
- UC6 – Générer un rapport de ventes : Un gestionnaire peut générer un rapport détaillé contenant des informations sur les magasins, les ventes et les inventaires.
- UC7 – Visualiser les magasins dans un tableau de bord : Un gestionnaire peut consulter un tableau de bord pour avoir une vue d’ensemble de la situation des magasins.

![alt text](../../images/cas_utilisation.png)

# 6. Structure logique

- `centre_logistique/` : Représente le lien entre les magasins et la maison mère.
  - `migration/` : Contient la création des tables.
  - `src/` : Contient le code source de l'application `centre_logistique`.
    - `controllers/` : Contient les routes et la logique de contrôle des requêtes.
    - `models/` : Contient les définitions des entités.
    - `seed/` : Contient l'insertion des données initiales.
    - `cache.rs` : Gestion du cache.
    - `db.rs` : Gestion de la connexion de la base de données.
    - `dto.rs` : Définitions des objets de transfert de données à un autre.
    - `main.rs` : Point d'entrée principal de l'application.
    - `metrics.rs` : Gestion des métriques de l'application.
    - `routes.rs` : Définition des routes de l'application.
    - `schema.rs` : Schéma de la base de données pour l'ORM.
  - `test_stress/` : Contient les tests de stress pour le centre logistique.
- `magasin/` : Contient le code pour le magasin CLI.
  - 
- `magasin_enligne/` : Contient le code pour le magasin en ligne.
- `maison_mere/` : Contient le code pour la maison mère.




- `app/` : backend du magasin local
  - `models/` : classes de données
  - `controllers/` : Logique des opérations
  - `views/` : les valeurs entrées/sorties dans le console
  
- `main_server/` : serveur mère
  - `models/` : classes de données

# 7. Structure de développement

- `docs/` : UML 4+1, ADR et rapport final
- `seed/` : les données initiales pour la BD
- `tests/` : tests unitaires
- `github/` : workflows CI/CD
  
# 8. Déploiement

- Dépendances gérées par `requirements.txt`
- Configuration via `.env`
- Lancement : `python3 app.py` (magasin), `python3 -m main_server.server` (serveur)
- Docker : `docker compose up`

# 9. Crosscutting concepts

Dans l’application côté magasin, les entrées sont validées afin d’assurer l’intégrité des données saisies par l’utilisateur.
Les utilisateurs sont classés selon deux rôles :
- "user" pour les clients
- "admin" pour les employés du magasin
Ces rôles déterminent les actions accessibles à chaque type d'utilisateur. Par exemple, seuls les administrateurs peuvent envoyer des demandes de réapprovisionnement.
Par ailleurs, les requêtes critiques sont encapsulées dans des blocs `try/except` .

Ensuite, j’ai implémenté le patron de conception Singleton pour le client, puisqu’il est utilisé à plusieurs reprises tout au long de l’exécution de l’application. Cela permet d’éviter de devoir passer l’instance du client manuellement à chaque fois et garantit qu’une seule instance est utilisée de manière cohérente.

# 10. Décisions d’architecture (ADR)

## Titre 1
Choix de mécanisme de base de données (SQL vs NoSQL, local vs serveur)

## Contexte
Je dois choisir un mécanisme de base de donner pour l'application de système de caissier dans un magasin :
- Je veux que les données sont classées dans des tableaux. 
- Je veux qu'il soit facile à maintenir.
- Je veux que les données puissent être utilisées localement.

## Décision
J'ai décidé d'utiliser une base de données SQL, car elle est plus approprié pour structurer les données sous forme de tables avec des relations entre elles. Cela permet de faciliter la maintenance.  La base de donnée sera en serveur, car l'application sera utilisée sur plusieurs magasins.

## Statut
Accepté

## Conséquences
1. Plusieurs tables relationnelles seront créées pour modéliser les entités de l'application.
2. Les données seront sauvegradées localement sur l'appareil. Il faudra mettre en place une méthode de sauvegarde régulière pour en cas de changement d'appareil.
3. Le système ne dépendra pas à un réseau de connexion.
4. Changer de mécanisme de base de données sera un défi, car les requêtes nécessaires pour obtenir les mêmes résultats ne seront pas identiques.



## Titre 2
Choix de mécanisme de base de données (SQL vs NoSQL, local vs serveur)

## Contexte
Je dois choisir un mécanisme de base de donner pour l'application de système de caissier dans un magasin :
- Je veux que les données sont classées dans des tableaux. 
- Je veux qu'il soit facile à maintenir.
- Je veux que les données puissent être utilisées localement.

## Décision
J'ai décidé d'utiliser une base de données SQL, car elle est plus approprié pour structurer les données sous forme de tables avec des relations entre elles. Cela permet de faciliter la maintenance.  La base de donnée sera en serveur, car l'application sera utilisée sur plusieurs magasins.

## Statut
Accepté

## Conséquences
1. Plusieurs tables relationnelles seront créées pour modéliser les entités de l'application.
2. Les données seront sauvegradées localement sur l'appareil. Il faudra mettre en place une méthode de sauvegarde régulière pour en cas de changement d'appareil.
3. Le système ne dépendra pas à un réseau de connexion.
4. Changer de mécanisme de base de données sera un défi, car les requêtes nécessaires pour obtenir les mêmes résultats ne seront pas identiques.
   

# 11. Qualité

## Maintenabilité
- Le projet suit une architecture inspirée de MVC pour faciliter la maintenance.
- La séparation des responsabilités entre models, controllers et views permet de facilement modifier le code.

## Réutilisabilité
- En utilisant le patron de conception Singleton pour le client permet d'assurer la limite de la duplication d'instance.

# 12. Risques

- Plusieurs risques sont présentes actuallement dans le projet : 
- Je n'ai pas configuré l'application magasin en ligne avec des caches ni mis en place un meilleur système de protection contre les malwares pour empêcher un utilisateur de consulter les pages s'il n'est pas connecté.
- Je n'ai pas effectué de tests unitaires.
- Je n'ai pas réalisé de tests de stress de meilleur qualité pour évaluer la performance de l'application.
- Je n'ai pas testé le programme sur un autre serveur comme celui fourni par le chargé de laboratoire.
- Je n'ai pas optimisé la taille de l'image Docker.
- J'ai fortement utilisé ChatGPT pour réaliser ce travail, mais plusieurs fichiers ont été faits par moi-même, avec l'aide de l'IA pour améliorer mon code.

