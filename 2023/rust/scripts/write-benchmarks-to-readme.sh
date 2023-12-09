#!/usr/bin/sh
# TODO: I can't get this to work anymore - final sed does not want to accept
# the escaped output

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

output_escaped=$(echo -e "$output" | sed -e 's/[^^]/[&]/g; s/\^/\\^/g; $!a\'$'\n''\\n' | tr -d '\n')

echo "$output_escaped"

sed -Ei "/^test/d;s/(\`\`\`text)/\1\n$output_escaped/" README.md
