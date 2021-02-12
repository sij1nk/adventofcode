use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_bags<'a, I, S>(lines: I) -> HashMap<String, HashMap<String, usize>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let re = Regex::new(r"(?:[0-9] )?[a-z]* [a-z]* bag").unwrap();
    let mut bags = HashMap::new();
    for line in lines
        .into_iter()
        .map(|l| l.as_ref())
        .filter(|l| !l.contains("no other"))
    {
        let mut finds = re.find_iter(line).map(|m| m.as_str().to_owned());
        if let Some(mut bag) = finds.next() {
            bag.truncate(bag.len() - 4);
            bags.insert(
                bag,
                finds
                    .map(|l| {
                        let mut chars = l.chars();
                        let next = chars.next().unwrap().to_string();
                        let amount = next.parse::<usize>().unwrap();
                        let mut name = chars
                            .skip(1)
                            .collect::<Vec<_>>()
                            .into_iter()
                            .collect::<String>();
                        name.truncate(name.len() - 4);
                        (name, amount)
                    })
                    .collect::<HashMap<String, usize>>(),
            );
        }
    }

    bags
}

pub fn part1<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let bags = parse_bags(lines);
    let mut found: HashSet<&str> = HashSet::new();
    let mut temp: HashSet<&str> = HashSet::new();
    let mut goal: HashSet<&str> = HashSet::new();
    goal.insert("shiny gold");

    loop {
        for g in goal.iter() {
            let parents = bags
                .iter()
                .filter(|&(_, v)| v.keys().any(|k| k == g))
                .map(|(k, _)| k.as_str());
            temp.extend(parents);
        }

        if temp.len() == 0 {
            break;
        }

        goal.clear();
        goal.extend(temp.iter());
        found.extend(temp.iter());
        temp.clear();
    }

    found.len()
}

pub fn part2<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let bags = parse_bags(lines);
    let mut stack: Vec<(&str, usize)> = vec![("shiny gold", 1)];
    let mut count = 0;

    while let Some((s, n)) = stack.pop() {
        count += n;
        if let Some(children) = bags.get(s) {
            for (c, nc) in children.iter() {
                stack.push((c, nc * n));
            }
        }
    }

    // The shiny gold bag should not be counted
    count - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    #[test]
    fn part1_test() {
        let count = part1(EXAMPLE);
        assert_eq!(count, 4);
    }

    #[test]
    fn part2_test() {
        let count = part2(EXAMPLE);
        assert_eq!(count, 32);
    }
}
