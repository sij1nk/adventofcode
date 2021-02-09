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
            if let Some(last) = passports.last_mut() {
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

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLES: &[&str] = &[
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
        "byr:1937 iyr:2017 cid:147 hgt:183cm",
        "",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
        "hcl:#cfa07d byr:1929",
        "",
        "hcl:#ae17e1 iyr:2013",
        "eyr:2024",
        "ecl:brn pid:760753108 byr:1931",
        "hgt:179cm",
        "",
        "hcl:#cfa07d eyr:2025 pid:166559648",
        "iyr:2011 ecl:brn hgt:59in",
        "",
        "eyr:1972 cid:100",
        "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        "",
        "iyr:2019",
        "hcl:#602927 eyr:1967 hgt:170cm",
        "ecl:grn pid:012533040 byr:1946",
        "",
        "hcl:dab227 iyr:2012",
        "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "",
        "hgt:59cm ecl:zzz",
        "eyr:2038 hcl:74454a iyr:2023",
        "pid:3556412378 byr:2007 ",
        "",
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
        "hcl:#623a2f",
        "",
        "eyr:2029 ecl:blu cid:129 byr:1989",
        "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "",
        "hcl:#888785",
        "hgt:164cm byr:2001 iyr:2015 cid:88",
        "pid:545766238 ecl:hzl",
        "eyr:2022",
        "",
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        "",
    ];

    #[test]
    fn part1_test() {
        let count = part1(EXAMPLES);
        assert_eq!(count, 10);
    }

    #[test]
    fn part2_test() {
        let count = part2(EXAMPLES);
        assert_eq!(count, 6);
    }
}
