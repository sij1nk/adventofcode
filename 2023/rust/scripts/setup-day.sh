#!/usr/bin/sh

[ -z $1 ] && echo "Usage: setup-day.sh DAY" && exit 1

script_path=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd $script_path/..

year=2023
day=$(echo $1 | sed 's/^0.+//')
if ! [ $day -ge 1 -a $day -le 25 ]
then
  echo Day should be in [1..25]
  exit 1
fi
grep -E "day0+${day}" src/lib.rs > /dev/null
[ $? -eq 0 ] && echo "Day ${day} source files are already present" && exit 1

filename=../day${day}.txt
api_key=$(cat .api_key 2> /dev/null)
[ $? -ne 0 ] && echo ".api_key file is missing" && exit 1

wget -q -O ${filename} --header="Cookie: session=${api_key}" https://adventofcode.com/${year}/day/${day}/input
if [ $? -ne 0 ]
then
  echo Failed to fetch inputs for day $day
  rm ${filename}
  exit 1
fi
