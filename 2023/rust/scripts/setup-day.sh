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

# Begin patching files...
lib_file="src/lib.rs"
main_file="src/main.rs"
bench_file="benches/criterion.rs"
src_file="src/day${day}.rs"
template_file=".template.rs"

bench_call_template="$(cat << 'EOF'
c.bench_function("day %DAY% part 1", |b| b.iter(||
day%DAY%::part1(black_box(DAY%DAY%))));
c.bench_function("day %DAY% part 2", |b| b.iter(||
day%DAY%::part2(black_box(DAY%DAY%))));
EOF
)"
bench_call_template="$(echo "${bench_call_template}" | sed ':a;N;$!ba;s/\n/\\n/g' | sed 's/\$/\\$/g')"

main_call_template="$(cat << 'EOF'
if args.is_empty() || args.contains("%DAY%") {
println!("Day %DAY%");
println!("{:?}", day%DAY%::part1(DAY%DAY%)?);
println!("{:?}", day%DAY%::part2(DAY%DAY%)?);
}
EOF
)"
main_call_template="$(echo "${main_call_template}" | sed ':a;N;$!ba;s/\n/\\n/g' | sed 's/\$/\\$/g')"

cp $template_file $src_file

sed -i "s/\/\*%lib.rs_module%\*\//pub mod day%DAY%;\n\/\*%lib.rs_module%\*\//" $lib_file
sed -i "s/\/\*%main.rs_call%\*\//$main_call_template\n\/\*%main.rs_call%\*\//" $main_file
sed -i "s/\/\*%main.rs_import%\*\//day%DAY%, \/\*%main.rs_import%\*\//" $main_file
sed -i "s/\/\*%criterion.rs_call%\*\//$bench_call_template\n\/\*%criterion.rs_call%\*\//" $bench_file
sed -i "s/\/\*%criterion.rs_import%\*\//day%DAY%, \/\*%criterion.rs_import%\*\//" $bench_file

sed -i "s/%DAY%/${day}/g" $lib_file
sed -i "s/%DAY%/${day}/g" $main_file
sed -i "s/%DAY%/${day}/g" $bench_file

rustfmt $lib_file $main_file $bench_file
