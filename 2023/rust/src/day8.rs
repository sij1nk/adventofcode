use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Step {
    Left,
    Right,
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut lines = lines
        .into_iter()
        .map(|l| l.as_ref())
        .filter(|l| !l.is_empty());

    let steps = lines
        .next()
        .ok_or(anyhow!("Input should not be empty"))?
        .chars()
        .filter_map(|c| match c {
            'L' => Some(Step::Left),
            'R' => Some(Step::Right),
            unknown => {
                eprintln!("Encountered unknown step type '{}'", unknown);
                return None;
            }
        })
        .collect::<Vec<_>>();

    for line in lines {
        let (node, neighbors) = line
            .split_once('=')
            .ok_or(anyhow!("Couldn't split node description string by '='"))?;
        let (left_neighbor, right_neighbor) = neighbors
            .split_once(',')
            .ok_or(anyhow!("Couldn't split neighbors by ','"))?;

        graph.insert(
            node.trim(),
            (
                &left_neighbor[2..],
                &right_neighbor[1..right_neighbor.len() - 1],
            ),
        );
    }

    let mut current = "AAA";
    let mut step_count = 0;

    while current != "ZZZ" {
        let node = &graph[current];
        let step_index = step_count % steps.len();
        let step = steps[step_index];

        current = match step {
            Step::Left => &node.0,
            Step::Right => &node.1,
        };

        step_count += 1;
    }

    Ok(step_count as u32)
}

pub fn part2<'a, I, S>(_lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    static EXAMPLE2: &[&str] = &[
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();
        let result2 = part1(EXAMPLE2).unwrap();

        assert_eq!(result, 2);
        assert_eq!(result2, 6);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
