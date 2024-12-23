use std::collections::BTreeMap;

type Int = u8;

type Rules = BTreeMap<Int, Vec<Int>>;
type Update = Vec<Int>;

fn parse<'a, I, S>(lines: I) -> (Rules, Vec<Update>)
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut rules: Rules = BTreeMap::new();
    let mut updates: Vec<Update> = Vec::new();

    let mut should_parse_as_update = false;

    for line in lines.into_iter().map(|s| s.as_ref()) {
        if line.is_empty() {
            should_parse_as_update = true;
            continue;
        }

        if should_parse_as_update {
            let update = line
                .split(',')
                .map(|w| w.parse::<Int>().unwrap())
                .collect::<Vec<_>>();
            updates.push(update);
        } else {
            let (rule_from, rule_to) = line.split_once('|').unwrap();
            let rule_from = rule_from.parse::<Int>().unwrap();
            let rule_to = rule_to.parse::<Int>().unwrap();

            if let Some(tos) = rules.get_mut(&rule_from) {
                tos.push(rule_to);
            } else {
                rules.insert(rule_from, vec![rule_to]);
            }
        }
    }

    (rules, updates)
}

fn verify_update(rules: &Rules, update: &Update) -> Option<u8> {
    let zip = update.iter().zip(update.iter().skip(1));

    for (first, second) in zip {
        let result = rules.get(first).map(|tos| tos.contains(second));
        if result.is_none() || result.is_some_and(|v| !v) {
            return None;
        }
    }

    // assuming length is an odd number > 1, which seems to be the case in the input
    let middle = update[(update.len() - 1) / 2];

    Some(middle)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (rules, updates) = parse(lines);

    Ok(updates
        .iter()
        .filter_map(|u| verify_update(&rules, u))
        .map(|n| n as u32)
        .sum())
}

fn fix_incorrect_update(rules: &Rules, update: &Update) -> Option<u8> {
    if verify_update(rules, update).is_some() {
        return None;
    }

    // In a correctly ordered update, the rule for the page at update[i] points to all other pages
    // that come after update[i]. So *only* the rule for the page in the middle points to exactly
    // this amount of page numbers included in the update:
    let tos_count_from_middle = (update.len() - 1) / 2;

    for int in update.iter() {
        let rest = update.iter().filter(|&n| n != int);
        let Some(tos) = rules.get(int) else {
            continue;
        };

        let tos_count = rest.filter(|&n| tos.contains(n)).count();
        if tos_count == tos_count_from_middle {
            return Some(*int);
        }
    }

    panic!("Could not find middle page number");
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (rules, updates) = parse(lines);

    Ok(updates
        .iter()
        .filter_map(|u| fix_incorrect_update(&rules, u))
        .map(|n| n as u32)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "47|53",
        "97|13",
        "97|61",
        "97|47",
        "75|29",
        "61|13",
        "75|53",
        "29|13",
        "97|29",
        "53|29",
        "61|53",
        "97|53",
        "61|29",
        "47|13",
        "75|47",
        "97|75",
        "47|61",
        "75|61",
        "47|29",
        "75|13",
        "53|13",
        "",
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 143);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 123);
    }
}
