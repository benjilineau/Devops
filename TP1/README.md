# Devops

Serveur web développé en rust en utilisant le moins de dépendances possible. Le serveur est une API qui renvoit les headers d'une requête sur la route /ping (HTTP GET)). Toutes les autres routes renvoient vers une erreur 404 Le port d'écoute du serveur est modifiable. (variable PING_LISTEN_PORT).

## lancer le serveur 

- Dans un premier temps intaller rust
- Ensuite, il faut cloner le projet `git clone https://github.com/benjilineau/Devops.git`
- Lancer le serveur avec la commande : `cargo run`
![](https://i.imgur.com/7m9PzNX.png)
- Rendez-vous maintenant dans votre navigateur a l'adresse suivante: `http://localhost:8080/ping`. Si toutes les étapes ont été respécté alors vous devriez voir le contenu de votre header sur la page.