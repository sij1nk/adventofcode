use regex::RegexSet;

fn parse_passports<'a, I, S>(lines: I) -> Vec<String>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut passports = vec![String::from("")];
    for line in lines.into_iter() {
        let line = line.as_ref();
        if line == "" {
            passports.push("".into());
        } else {
            if let Some(mut last) = passports.last_mut() {
                last.push_str(line);
                last.push_str(" ");
            }
        }
    }

    passports
}

pub fn part1<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let passports = parse_passports(lines);
    let set = RegexSet::new(&["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]).unwrap();

    passports
        .into_iter()
        .filter(|p| set.matches(p).into_iter().count() == set.len())
        .count()
}

pub fn part2<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let passports = parse_passports(lines);
    let set = RegexSet::new(&[
        r"byr:19[2-9][0-9]|200[0-2] ",
        r"iyr:201[0-9]|2020 ",
        r"eyr:202[0-9]|2030 ",
        r"hgt:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in ",
        r"hcl:#[0-9a-f]{6} ",
        r"ecl:amb|blu|brn|gry|grn|hzl|oth ",
        r"pid:[0-9]{9} ",
    ])
    .unwrap();

    passports
        .into_iter()
        .filter(|p| set.matches(p).into_iter().count() == set.len())
        .count()
}
