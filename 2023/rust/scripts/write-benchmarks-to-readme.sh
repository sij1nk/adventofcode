#!/usr/bin/sh

script_path=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd $script_path/..

rg -F '//' benches/criterion.rs > /dev/null
if [ $? -eq 0 ]
then
  echo "Some benchmarks are commented out! Aborting"
  exit 1
fi

output=$(cargo bench --bench criterion -- --output-format=bencher -n |\
  tee /dev/tty |\
  sed '/^[[:space:]]*$/d;/^test/!d')

sed -Ei "/^test/d;s/(%BENCH_START%)/\1\n$output_escaped/" README.md
