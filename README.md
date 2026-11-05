# web_power

**GridMaster** est un jeu de gestion d'un réseau électrique régional, dans le navigateur, sur un plateau d'hexagones en style cartoon (three.js).

La partie est une campagne historique de 1900 à 2050. En 1900, une seule ville découvre l'électricité, alimentée par une petite usine au charbon et une usine au fil de l'eau, en 15 kV. Au fil des époques, les villes s'étendent, de nouveaux villages apparaissent et deviennent des villes, et les technologies arrivent à leur date historique : 63 kV, grands barrages, 225 kV et interconnexion, fioul, 400 kV, nucléaire, gaz, éolien, solaire, batteries, petits réacteurs, fusion.

La région a deux réseaux à gérer : l'électricité et l'eau. Le but est de les étendre à mesure que la région se peuple. En 2050, l'électricité doit être bas carbone, sans coupure ni faillite, et l'eau potable servie et conforme aux normes.

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

## Trois vues

- **Carte** (touche 1) : toute la région, les deux réseaux à la fois, sans les flux.
- **Électricité** (touche 2) : centrales et postes en surbrillance dorée, courant dans les lignes ; les ouvrages de l'eau passent en fantôme.
- **Eau** (touche 3) : captages, usines, châteaux d'eau et épurations en surbrillance bleue, conduites en transparence avec l'eau qui y coule ; les centrales passent en fantôme.

On construit dans la vue de chaque réseau.

## Gestion de l'eau

- **Ouvrages**, datés comme les centrales :
  - captage de source, forage en nappe, prise d'eau en rivière (1900) ;
  - usine de potabilisation (1905) ;
  - château d'eau, réservoir sur colline (1900) ;
  - station de pompage (1910) ;
  - lagunage (1900) ;
  - forage profond (1955) ;
  - station d'épuration (1950) ;
  - dessalement (1985).
- **Conduites et égouts** : tracés d'hexagone en hexagone ; une nouvelle conduite suit de préférence les conduites existantes. Dans une ville raccordée, la distribution se dessine toute seule de quartier en quartier.
- **Simulation**, par réseau connexe et par pas de 15 minutes :
  - demande des habitants et des industries, avec les pointes du matin et du soir et la chaleur ;
  - débit de la rivière, irrigation des fermes l'été, débit réservé à partir de 1984, nappe qui se recharge l'hiver ;
  - stocks des châteaux d'eau, fuites qui augmentent avec l'âge des conduites, casses.
- **Qualité** : six paramètres (turbidité, microbes, matières organiques, nitrates, pesticides, micropolluants). Leurs normes entrent en vigueur au fil du siècle. La chaîne de traitement de chaque usine se complète par étapes : filtre à sable et chlore, charbon actif, ozonation, dénitratation, ultrafiltration.
- **Assainissement** : les eaux usées non épurées polluent la rivière en aval et dégradent les prises d'eau. Elles coûtent des amendes à partir de 1992.
- **Économie** : prix du m³ réglable, restrictions d'usage, renouvellement des vieilles conduites. Le pompage et le traitement consomment de l'électricité.

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
- **Ambiance** : fermes sur les plaines et les prairies, routes entre les villes (en terre, puis goudronnées en 1930), charrettes puis voitures, bateaux, oiseaux, vaches, cerfs et tracteurs.
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
