use std::collections::HashSet;

fn union(set1: &HashSet<char>, set2: &HashSet<char>) -> HashSet<char> {
    set1.union(set2).cloned().collect()
}

fn intersect(set1: &HashSet<char>, set2: &HashSet<char>) -> HashSet<char> {
    set1.intersection(set2).cloned().collect()
}

fn count_answers<'a, I, S, F>(lines: I, reset: String, op: F) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
    F: Fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>,
{
    let mut iter = lines.into_iter();
    let mut sum = 0;
    let mut answers = iter
        .next()
        .unwrap()
        .as_ref()
        .chars()
        .collect::<HashSet<_>>();
    loop {
        match iter.next() {
            None => {
                break sum + answers.len();
            }
            Some(ans) => {
                let ans = ans.as_ref();
                if ans == "" {
                    sum += answers.len();
                    answers = reset.chars().collect();
                } else {
                    let chars = ans.chars().collect::<HashSet<_>>();
                    answers = op(&answers, &chars);
                }
            }
        }
    }
}

pub fn part1<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    count_answers(lines, "".into(), union)
}

pub fn part2<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    count_answers(lines, "abcdefghijklmnopqrstuvwxyz".into(), intersect)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
    ];

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
