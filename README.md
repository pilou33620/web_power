# web_power

**GridMaster** est un jeu de gestion d'un réseau électrique régional, dans le navigateur, sur un plateau d'hexagones en style cartoon (three.js).

La partie est une campagne historique de 1900 à 2050. En 1900, une seule ville découvre l'électricité, alimentée par une petite usine au charbon et une usine au fil de l'eau, en 15 kV. Au fil des époques, les villes s'étendent, de nouveaux villages apparaissent et deviennent des villes, et les technologies arrivent à leur date historique : 63 kV, grands barrages, 225 kV et interconnexion, fioul, 400 kV, nucléaire, gaz, éolien, solaire, batteries, petits réacteurs, fusion.

Le but est d'électrifier la région à mesure qu'elle se peuple, puis d'atteindre en 2050 une électricité bas carbone, sans coupure et sans faillite.

| Époque | À partir de | Ce qui change |
|---|---|---|
| Les pionniers | 1900 | usines au charbon et au fil de l'eau, lignes 15 kV |
| L'entre-deux-guerres | 1920 | 63 kV, grands barrages (1922), centrales au charbon (1925) |
| Le réseau régional | 1935 | 225 kV, postes 225/63 kV, interconnexion (1938) |
| La reconstruction | 1946 | fioul (1952), grandes centrales au charbon (1955), forte croissance |
| Les Trente Glorieuses | 1960 | 400 kV, turbines à combustion (1965) |
| Le choc pétrolier | 1974 | fioul hors de prix, nucléaire |
| Le marché | 1990 | cycles combinés au gaz, éolien (1996), biomasse |
| Le climat | 2005 | taxe carbone, solaire, éolien en mer, batteries |
| La transition | 2020 | sortie du charbon obligatoire avant 2032 |
| Le futur | 2035 | petits réacteurs (2030), fusion (2042) |

## Jouer

Le jeu tient dans un seul fichier, `index.html`, sans installation ni serveur obligatoire.

- **Localement** : ouvrez `index.html` dans un navigateur récent. Une connexion internet est nécessaire pour charger three.js et les polices.
- **En ligne** : activez GitHub Pages sur la branche `main` (Settings → Pages). Le jeu sera alors disponible à l'adresse `https://pilou33620.github.io/web_power/`.

La partie s'enregistre automatiquement dans le navigateur à chaque jour de jeu.

## Contenu

- **Plateau** : une installation par hexagone, avec des règles de pose par terrain (barrage sur rivière, nucléaire au bord de l'eau, éolien en mer sur la côte…). Les villes occupent plusieurs hexagones et s'agrandissent avec les années.
- **Réseau** : lignes 15, 63, 225 et 400 kV tracées d'hexagone en hexagone. Au début, les usines alimentent directement les villes en 15 kV ; plus tard, l'ordre devient centrale → 225 kV → poste 225/63 kV → 63 kV → ville.
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

Le banc joue des campagnes complètes de 1900 à 2050 avec trois stratégies programmées (aucune IA) :

- **Passif** : ne construit rien.
- **Fossile** : charbon, puis fioul, puis gaz, au moindre coût.
- **Bas carbone** : hydraulique, puis nucléaire, renouvelables, stockage et petits réacteurs.

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
node banc.js diffs=normal strats=passif,fossile,vert paralleles=4
```

Options disponibles :

| Option | Valeurs |
|---|---|
| `cartes=` | `valmont`, `littoral`, `plaine`, `ile`, `industrie`, `marche`, `aleatoire` |
| `diffs=` | `facile`, `normal`, `difficile` |
| `strats=` | `passif`, `fossile`, `vert` |
| `graine=` | graine de la carte aléatoire |
| `sortie=` | préfixe des fichiers produits |

Le programme écrit `banc-resultats.json` et `banc-rapport.md`. Les résultats de référence sont dans [`banc/rapport-reference.md`](banc/rapport-reference.md).
