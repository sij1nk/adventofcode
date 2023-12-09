use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Step {
    Left,
    Right,
}

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_graph_and_steps<'a, I, S>(lines: I) -> anyhow::Result<(Graph<'a>, Vec<Step>)>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut graph: Graph = HashMap::new();

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
                None
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

    Ok((graph, steps))
}

fn measure_path(
    graph: &Graph,
    steps: &Vec<Step>,
    starting_node: &str,
    end_condition: impl Fn(&str) -> bool,
) -> u64 {
    let mut current_node = starting_node;
    let mut step_count = 0;

    while !end_condition(current_node) {
        let &(left_neighbor, right_neighbor) = &graph[current_node];
        let step_index = step_count % steps.len();
        let step = steps[step_index];

        current_node = match step {
            Step::Left => left_neighbor,
            Step::Right => right_neighbor,
        };

        step_count += 1;
    }

    step_count as u64
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (graph, steps) = parse_graph_and_steps(lines)?;

    let step_count = measure_path(&graph, &steps, "AAA", |s| s == "ZZZ");
    Ok(step_count)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (graph, steps) = parse_graph_and_steps(lines)?;

    let least_common_multiple = graph
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| measure_path(&graph, &steps, s, |s| s.ends_with('Z')))
        .fold(1, lcm);

    Ok(least_common_multiple)
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

    static EXAMPLE_PART2: &[&str] = &[
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
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
        let result = part2(EXAMPLE_PART2).unwrap();

        assert_eq!(result, 6);
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(21, 6), 42);
    }
}
