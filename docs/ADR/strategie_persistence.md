## Titre
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
