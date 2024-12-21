#!/bin/sh

set -e

rootdir="$(dirname $(realpath $0))"

day=$1
day_padded=$(printf "%02d" "$day")
id="day$day_padded"

cleanup() {
  [ $? -eq 0 ] && exit
  echo "Cleaning up"
  cd "$rootdir"
  rm -rf "$id"
}

trap cleanup EXIT

[ -d "$id" ] && echo "Day $day_padded is already initialized" && exit 0

api_key=$(cat .api-key)

cp -r .template "$id"
cd "$id" || exit 1
fdfind -t f -x sed -i "s/%DAY%/$id/g;s/%DAY_NICE%/Day $day_padded/g"

wget "https://adventofcode.com/2024/day/${day}/input" \
  --header="Cookie: session=$api_key" \
  -O "${id}.txt"

cd "$rootdir"
dasel put -f Cargo.toml -r toml -t string -v "$id" 'workspace.members.append()'

# dasel formats toml in a weird way, let's fix it by hand
sed -Ei 's/\s+(.*)/\1/g' Cargo.toml
