
# Exercice de screening pour XXXX

tu as une liste de joueurs d'Échecs avec leurs ages et scores (elo).
tu dois extraire de la liste les "champions"

Un joueur est dit champion si il n'y a personne d'autre dans la liste qui l'élimine, c'est à dire: 

* personne n'est  la fois strictement plus fort, et plus jeune ou même age et personne n'est à la fois plus jeune et plus fort ou même score

ta mission: dans le language de ton choix, coder la fonction permettant de trouver les champions de la liste.

On attachera une importance particulire aux points suivants:

* l'exactitude des résultats: Le candidat a-t-il pensé à la logique d'ensemble et aux cas particuliers ?

* la performance: comment se comportte l'algorithme à mesure que le nombre de joueurs grandit ?

* la clareté et la lisibilité du code

## désambiguïsation donné

* logique attendue : [{Jean, 10 ans, 1000pts},{Marie, 9 ans, 1100 pts}, {Pierre, 11 ans, 1200 pts}] => [Marie, Pierre]