#!/bin/sh
[ $# -ne 1 ] && exit
root_dir="$(dirname "$0")"
day=$1

cookie="${root_dir}/../../.cookie"
input_file="${root_dir}/../day${day}.txt"

wget "https://adventofcode.com/2020/day/${day}/input"\
    --header="Cookie: session=$(cat $cookie)"\
    -O ${input_file}

lib_file="${root_dir}/src/lib.rs"
main_file="${root_dir}/src/main.rs"
bench_file="${root_dir}/benches/criterion.rs"
src_file="${root_dir}/src/day${day}.rs"
template_file="${root_dir}/template.rs"

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
println!(
"{:?}",
day%DAY%::part1(DAY%DAY%).ok_or_else(|| util::to_ioerror(util::Error))?
);
println!(
"{:?}",
day%DAY%::part2(DAY%DAY%).ok_or_else(|| util::to_ioerror(util::Error))?
);
}
EOF
)"
main_call_template="$(echo "${main_call_template}" | sed ':a;N;$!ba;s/\n/\\n/g' | sed 's/\$/\\$/g')"

cp $template_file $src_file

sed -i "s/\/\*%MODULE%\*\//pub mod day%DAY%;\n\/\*%MODULE%\*\//" $lib_file
sed -i "s/\/\*%CALL%\*\//$main_call_template\n\/\*%CALL%\*\//" $main_file
sed -i "s/\/\*%IMPORT%\*\//day%DAY%, \/\*%IMPORT%\*\//" $main_file
sed -i "s/\/\*%CALL%\*\//$bench_call_template\n\/\*%CALL%\*\//" $bench_file
sed -i "s/\/\*%IMPORT%\*\//day%DAY%, \/\*%IMPORT%\*\//" $bench_file

sed -i "s/%DAY%/${day}/g" $lib_file
sed -i "s/%DAY%/${day}/g" $main_file
sed -i "s/%DAY%/${day}/g" $bench_file

rustfmt $lib_file $main_file $bench_file
