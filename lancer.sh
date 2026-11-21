#!/bin/sh
# GridMaster (Linux, macOS, Raspberry Pi) : obtient l'exécutable du serveur (binaires.sh) puis le lance.
#   ./lancer.sh [--port 8794] [--local] [--sans-navigateur] [--sans-pause] [--sans-maj]
cd "$(dirname "$0")" || exit 1
. ./binaires.sh
# Le serveur sort avec le code 75 quand il vient de se mettre à jour (git) :
# on reprend l'exécutable de la nouvelle version, puis on le relance.
export WEB_POWER_RELANCEUR=1
while :; do
  obtenir_binaires && bin/power "$@"
  code=$?
  [ "$code" -eq 75 ] || break
done
exit "$code"
