Lien de mon repos : https://github.com/Barlow-Personal-Git/Projet_LOG430

# 1. Introduction et objectifs

Ce projet est une application de gestion d'inventaire pour un réseau de magasins, dans le cadre du cours LOG430. Elle permet de consulter, acheter, retourner des produits et synchroniser les inventaires locaux avec le centre logistique. La maison mère peut générer des rapports et consulter un tableau pour évoluer la performance des magasins. Ce laboratoire vise à familier les étudiants avec les outils nécessaires à la création d'un projet de haut niveau, tels que API Request, Grafana et Prometheus.

Pour les laboratoires 3 et 4, les métriques et Swagger ont été implémentés dans Centre Logistique, car au moment où je travaillais dessus, seul Centre Logistique disposait d’API Request.

L'objectif de l'étape 2 est d'implémenter Swagger, des outils d’observation, d’appliquer l’architecture microservice et d’ajouter une API Gateway.

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

Après être arrivé au laboratoire 5, j’ai remarqué que c’était le magasin qu’il fallait améliorer et non centre logistique. Il était donc trop tard pour changer.

# 4. Contexte

## Contexte fonctionnel
- Il y a deux bases de données : une locale (magasin) et une centrale
- Le client peut :
  - Rechercher un produit
  - Acheter un produit
  - Consulter l'inventaire
  - Retourner un vente

- Le serveur côté centre_logistique peut :
  - Faire des appels API Request
  - Vérifier les Swaggers

- Le serveur côté maison mère peut :
  - Obtenir un rapport
  - Consulter un tableau

- Les employeurs qui occupent le centre logistique peuvent consulter grafana et promeuteus pour voir les requêtes s'ils fonctionnent bien correctement.

## Contexte technique
- PostgreSQL pour les deux bases de données
- Exécution local avec CLI
- Serveur maison mere et centre logistique roulent avec Rocket (Rust)

![Deploiement](../images/deploiement.png)

Le magasin en ligne et le magasin ont le même comportement. Cependant, le magasin en ligne est une application web, tandis que le magasin est une application CLI. Ce système repose sur une architecture DDD, où chaque domaine constitue un monolithe organisé selon le modèle MVC.

# 5. Scénarios d'utilisation

- UC1 – Rechercher un produit : Un client peut consulter les produits par identifiant, catégorie ou nom.
- UC2 – Acheter un produit : Un client peut ajouter des produits à une vente et confirmer son achat.
- UC3 – Retourner une vente : Un client peut retourner une vente effectuée précédemment dans le même magasin.
- UC4 – Consulter la liste des produits : Un client peut consulter l’inventaire complet du magasin.
- UC5 – Déclencher un réapprovisionnement : Un employé du magasin peut envoyer une demande de réapprovisionnement.
- UC6 – Générer un rapport de ventes : Un gestionnaire peut générer un rapport détaillé contenant des informations sur les magasins, les ventes et les inventaires.
- UC7 – Visualiser les magasins dans un tableau de bord : Un gestionnaire peut consulter un tableau de bord pour avoir une vue d’ensemble de la situation des magasins.
- UC8 - Visualiser les métriques du centre logistique : un gestionnaire peut consulter les métriques et tester les API requests avec Swagger.
- UC9 - Visualiser les tableaux dans Grafana : un gestionnaire peut observer les performances de l'application.

![Cas utilisation](../images/cas_utilisation_new.png)

Un exemple du fonctionnement lorsque l'utilisateur achete un produit
![diagramme sequence](../images/diagramme_classe_magasin.png)

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
  - `migration/` : Contient la création des tables.
  - `src/` : Contient le code source de l'application `magasin`.
    - `controllers/` : Contient les routes et la logique de contrôle des requêtes.
    - `models/` : Contient les définitions des entités.
    - `seed/` : Contient l'insertion des données initiales.
    - `db.rs` : Gestion de la connexion de la base de données.
    - `dto.rs` : Définitions des objets de transfert de données à un autre.
    - `main.rs` : Point d'entrée principal de l'application.
    - `session` : Contient la structure de base du client singleton.
    - `schema.rs` : Schéma de la base de données pour l'ORM.

- `magasin_enligne/` : Contient le code pour le magasin en ligne.
  - `migration/` : Contient la création des tables.
  - `src/` : Contient le code source de l'application `magasin_enligne`.
    - `controllers/` : Contient les routes et la logique de contrôle des requêtes.
    - `models/` : Contient les définitions des entités.
    - `seed/` : Contient l'insertion des données initiales.
    - `db.rs` : Gestion de la connexion de la base de données.
    - `dto.rs` : Définitions des objets de transfert de données à un autre.
    - `main.rs` : Point d'entrée principal de l'application.
    - `session` : Contient la structure de base du client singleton.
    - `routes.rs` : Définition des routes de l'application.
    - `schema.rs` : Schéma de la base de données pour l'ORM.
  - `templates/` : Contient les fichier Tera/html

- `maison_mere/` : Contient le code pour la maison mère.
  - `fonts/` : Contient les fonts pour la génération des pdfs.
  - `src/` : Contient le code source de l'application `maison_mere`.
    - `controllers/` : Contient les routes et la logique de contrôle des requêtes.
    - `models/` : Contient les définitions des entités.
    - `seed/` : Contient l'insertion des données initiales.
    - `db.rs` : Gestion de la connexion de la base de données.
    - `dto.rs` : Définitions des objets de transfert de données à un autre.
    - `main.rs` : Point d'entrée principal de l'application.
    - `session` : Contient la structure de base du client singleton.
    - `routes.rs` : Définition des routes de l'application.
    - `schema.rs` : Schéma de la base de données pour l'ORM.
  - `templates/` : Contient les fichier Tera/html

![Diagramme de classe](../images/diagramme_class.png)

Les modèles sont représentés comme illustré dans le diagramme de classes du magasin.

# 7. Structure de développement

- `docs/` : UML 4+1, ADR et rapport final
- `seed/` : les données initiales pour la BD
- `github/` : workflows CI/CD
- `migration/` : les scripts SQL de migration de base de données
  

  ![Implementation](../images/implementation.png)

# 8. Déploiement

- Dépendances gérées par `Cargo.toml` et `Cargo.lock`
- Configuration via `.env`
- Lancement : 
  - Lancement en local :
  - Magasin CLI** : `cargo run -p magasin login`
  - **Magasin en ligne : `cargo run -p magasin_enligne`
  - **Centre logistique : `cargo run -p centre_logistique`
  - Maison mère : `cargo run -p maison_mere`
- Docker : `docker compose up`

# 9. Crosscutting concepts

Dans l’application côté magasin, les entrées sont validées afin d’assurer l’intégrité des données saisies par l’utilisateur.
Les utilisateurs sont classés selon deux rôles :
- "user" pour les clients
- "admin" pour les employés du magasin

Dans le coté centre logistique, il utilie `tracing` pour la journalisation. Lorsqu'un utilisateur utilise l'application, un fichier `logs.json` est créé et tous les logs y sont enregistrés. Les metrics Prometheus sont exporté via le fichier `metrics.rs`.

Pour la sécurité des entrées côté serveur n'est pas encore implémenté. 

L'application utilise `.env` pour faciliter la configuration du projet.

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
La stratégie de persistence

## Contexte
Je dois choisir une base de donnée pour l'application de système de caissier dans un magasin :
- Je veux que l'utilisation est simple.
- Je veux pouvoir accéder rapidement aux données, pour garantir une utilisation fluide.
- Je veux que les données puissent être utilisées même si le réseau est hors service, afin d'assurer la continuité du service.

## Décision
J'ai décidé d'utiliser PostgreSQL comme base de données locale puisque le système se déroulera sur plusieurs magasin et une maison mère. Une base de donné NoSQL comme MongoDB ne serait pas idéale pour cette application, car elle est mieux adaptée à des systèmes distribués ou à forte volumétrie. PostgreSQL est simple à utiliser et peut être facilement configurée dans la machine virtuelle Linux fournie par l'école.

## Statut
Accepté

## Conséquences
1. Le système sera fonctionnelle hors ligne, ce qui évitera toute interruption en case de coupure du réseau.
2. L'utilisation de PostgreSQL est simple et permet une manipulation facile des données.
3. La gestion des connexions simultanées devra être surveillée si le système évolue vers une architecture multi-clients avec beaucoup de trafic.
4. PostgreSQL consomme plus de ressources qu’une base embarquée comme SQLite, ce qui peut poser problème
5. Le déploiement sur plusieurs machines nécessitera une synchronisation manuelle ou via un serveur central.

## Titre 3
Choix du langage de programmation : Rust vs Python

## Contexte
Pour développer le système du magasin et le centre logistique, je dois choisir un langage de programmation adapté aux exigences suivants :
- Je veux un langage peu utilisé dans le cours pour découvrir de nouvelles technologies.
- Je veux un langage qui facilite la maintenance du code
- Je veux un langage qui permette de développer aussi bien le frontend que le backend.
- Je veux un langage qui s’intègre facilement avec la base de données.

## Décision
J’ai décidé d’utiliser Rust pour la deuxième étape. Python était intéressant au début, mais j’ai rencontré des difficultés pour héberger une application sur un serveur. Avec Rust et Rocket, je peux exécuter l’application sans avoir à séparer le frontend et le backend. De plus, l’implémentation pour se connecter à la base de données PostgreSQL est nettement plus simple.

## Statut
Accepté

## Conséquences
1. Le développement demandera plus d'efforts initiaux, car Rust est une nouvelle langage pour moi.
2. Le code effectué à l'étape 1 doit reeffecuter mais avec langage Rust.
3. La création des images Docker pourrait représenter un défi.
4. La sécurité de l'application sera meilleure qu’avec Python grâce à l’absence de ramasse-miettes.


## Titre 4
Choix du Load Balancer : NGINX avec Rust

## Contexte
Pour appliquer un load balacer, il doit répondre à ces critères :
- Je veux un outil qui ne demande pas trop de travail pour être mis en place.
- Je veux un outil qui s’intègre facilement dans un environnement Docker.
- Je veux un outil simple à configurer.
- Je veux un outil capable de gérer facilement le nombre d’utilisateurs.

## Décision
J’ai choisi d’utiliser NGINX comme load balancer car il figurait en premier dans la liste proposée par l’énoncé du laboratoire. J'ai trouvé qu'il était facile à configurer dans le docker.

## Statut
Accepté

## Conséquences
1. La configuration du load balancer sera simple.
2. Le load balancer sera déployé en même temps que les dockers images.
3. L’utilisation de NGINX facilite la montée en charge pour gérer un plus grand nombre d’utilisateurs.
4. NGINX bénéficie d’une documentation abondante et d’exemples nombreux disponibles en ligne (ex. https://github.com/nginx/njs-examples/).
  
## Titre 5
Choix du Test de charge : JMeter vs k6

## Contexte
Pour ma décision du test de charge, il doit répondre à ces critères :
- Je souhaite un outil compatible avec Grafana.
- Je souhaite un outil qui facilite la simulation de tests de stress.
- Je souhaite un outil qui ne demande pas trop de travail.

## Décision
J’ai choisi d’utiliser k6, car c’était l’un des deux outils de test de charge que je n’avais pas essayés lors du cours de DevOps. J’avais utilisé JMeter pendant le cours et je l’avais trouvé plus lourd et plus exigeant à utiliser que k6.

## Statut
Accepté

## Conséquences
1. La mise en place des tests de charge sera plus rapide grâce à la simplicité de k6.
2. L'intégration avec Grafana permettra de visualiser facilement les résultats de tests en temps réel.
3. Les scripts de test sont facilent à implimenter car k6 utilise des fichiers JavaScript.
4. Le test de stress sera limitée par la capacité de la version k6 installée.


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
