# web_power

**GridMaster** est un jeu de gestion d'un réseau électrique régional, dans le navigateur, sur un plateau d'hexagones en style cartoon (three.js).

Vous produisez, transportez et distribuez l'électricité d'une région qui grandit. Le but est d'y arriver sans coupure, sans faillite, et avec une électricité bas carbone en 2050.

## Jouer

Le jeu tient dans un seul fichier, `index.html`, sans installation ni serveur obligatoire.

- **Localement** : ouvrez `index.html` dans un navigateur récent. Une connexion internet est nécessaire pour charger three.js et les polices.
- **En ligne** : activez GitHub Pages sur la branche `main` (Settings → Pages). Le jeu sera alors disponible à l'adresse `https://pilou33620.github.io/web_power/`.

La partie s'enregistre automatiquement dans le navigateur à chaque jour de jeu.

## Contenu

- **Plateau** : une installation par hexagone, avec des règles de pose par terrain (barrage sur rivière, nucléaire au bord de l'eau, éolien en mer sur la côte…). Les villes occupent plusieurs hexagones et s'agrandissent avec les années.
- **Réseau** : lignes 63, 225 et 400 kV tracées d'hexagone en hexagone, dans l'ordre centrale → 225 kV → poste 225/63 kV → 63 kV → ville.
  - Flux de puissance calculés ligne par ligne (courant continu), avec les pertes.
  - Contrôle du critère N-1, déclenchement des lignes en surcharge, îlotage.
- **Dispatch** : appel des centrales par coût marginal (modifiable), batteries, valeur de l'eau des barrages, import et export, fréquence, délestage.
- **Météo** : saisons, vent, soleil, débit de la rivière, prévisions à 3 jours.
- **Économie** : tarifs, combustibles, taxe carbone qui augmente chaque année, prêts.
- **R&D** : 28 technologies réparties en 4 branches et 5 niveaux.
- **Événements** : pannes, tempêtes, vagues de froid, canicules, sécheresses, crise du gaz.
- **Cartes** :
  - Vallée de Valmont ;
  - Côte d'Émeraude ;
  - Grande plaine agricole ;
  - Île de Kerlann (aucune interconnexion) ;
  - Bassin de Forgeval (industriels exigeants) ;
  - Carrefour des frontières (marché ouvert) ;
  - carte aléatoire, définie par une graine.
- **Difficultés** : Facile, Normal, Difficile.

## Banc d'équilibrage

Le banc joue des parties complètes de 2027 à 2050 avec trois stratégies programmées (aucune IA) :

- **Passif** : ne construit rien et garde le charbon.
- **Tout gaz** : remplace le charbon et suit la croissance avec du gaz.
- **Transition bas carbone** : renouvelables, stockage, puis petits réacteurs.

Il sert à vérifier l'équilibrage après une modification.

**Dans le jeu** : Menu → Banc d'équilibrage.

**En ligne de commande**, avec plusieurs parties en parallèle et un rapport année par année :

```bash
cd banc
```

```bash
npm install
```

```bash
npx playwright install chromium
```

```bash
node banc.js diffs=normal strats=passif,gaz,vert paralleles=4
```

Options disponibles :

| Option | Valeurs |
|---|---|
| `cartes=` | `valmont`, `littoral`, `plaine`, `ile`, `industrie`, `marche`, `aleatoire` |
| `diffs=` | `facile`, `normal`, `difficile` |
| `strats=` | `passif`, `gaz`, `vert` |
| `graine=` | graine de la carte aléatoire |
| `sortie=` | préfixe des fichiers produits |

Le programme écrit `banc-resultats.json` et `banc-rapport.md`. Les résultats de référence sont dans [`banc/rapport-reference.md`](banc/rapport-reference.md).
