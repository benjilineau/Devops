# Devops

Serveur web développé en rust en utilisant le moins de dépendances possible. Le serveur est une API qui renvoit les headers d'une requête sur la route /ping (HTTP GET)). Toutes les autres routes renvoient vers une erreur 404 Le port d'écoute du serveur est modifiable. (variable PING_LISTEN_PORT).

## lancer le projet

- Voir tp précédent
- `kubectl apply -f .\pod.yaml `
- `kubectl port-forward pods/my-nginx-pod 8080:8080`

## Vérification

```
Benjamin@DESKTOP-ON7BJGL MINGW64 ~/Documents/ynov B3/Devops/WIK-DPS-TP04 (main)
$ curl localhost:8080/ping
{"host":"localhost:8080","user-agent":"curl/7.71.1","accept":"*/*"}
```
