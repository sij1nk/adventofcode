use std::collections::{HashMap, VecDeque};

fn find_paths(routes: &HashMap<&str, Vec<&str>>, src: &str, dst: &str) -> u32 {
    let mut sum = 0;
    let mut queue = VecDeque::from([src]);

    while let Some(current) = queue.pop_front() {
        let Some(to) = routes.get(current) else {
            continue;
        };

        for &t in to.iter() {
            if t == dst {
                sum += 1;
            } else {
                queue.push_back(t);
            }
        }
    }

    sum
}

fn find_paths_short_circuit(
    routes: &HashMap<&str, Vec<&str>>,
    src: &str,
    dst: &str,
) -> Option<u32> {
    let mut sum = 0;
    let mut queue = VecDeque::from([src]);

    while let Some(current) = queue.pop_front() {
        let Some(to) = routes.get(current) else {
            continue;
        };

        for &t in to.iter() {
            if t == dst {
                sum += 1;
            } else {
                queue.push_back(t);
            }
        }

        if queue.len() > 100000000 {
            return None;
        }
    }

    Some(sum)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut routes = HashMap::new();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (from, to) = line.split_once(":").expect("line to contain ':'");
        let to = to.split(" ").skip(1).collect::<Vec<_>>();

        routes.insert(from, to);
    }

    Ok(find_paths(&routes, "you", "out"))
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut routes = HashMap::new();
    let mut routes_backwards: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (from, to) = line.split_once(":").expect("line to contain ':'");
        let to = to.split(" ").skip(1).collect::<Vec<_>>();

        for &t in to.iter() {
            routes_backwards.entry(t).or_default().push(from);
        }

        routes.insert(from, to);
    }

    // let fft_from_svr = find_paths_short_circuit(&routes_backwards, "fft", "svr");
    // let svr_to_fft = find_paths_short_circuit(&routes, "svr", "fft");
    // println!("fft_from_svr: {fft_from_svr:?}, svr_to_fft: {svr_to_fft:?}");
    //
    // let dac_from_svr = find_paths_short_circuit(&routes_backwards, "dac", "svr");
    // let svr_to_dac = find_paths_short_circuit(&routes, "svr", "dac");
    // println!("dac_from_svr: {dac_from_svr:?}, svr_to_dac: {svr_to_dac:?}");

    let dac_from_fft = find_paths_short_circuit(&routes_backwards, "dac", "fft");
    let fft_to_dac = find_paths_short_circuit(&routes, "fft", "dac");
    println!("dac_from_fft: {dac_from_fft:?}, fft_to_dac: {fft_to_dac:?}");

    // let fft_from_dac = find_paths_short_circuit(&routes_backwards, "fft", "dac");
    // let dac_to_fft = find_paths_short_circuit(&routes, "dac", "fft");
    // println!("fft_from_dac: {fft_from_dac:?}, dac_to_fft: {dac_to_fft:?}");
    //
    // let out_from_dac = find_paths_short_circuit(&routes_backwards, "out", "dac");
    // let dac_to_out = find_paths_short_circuit(&routes, "dac", "out");
    // println!("out_from_dac: {out_from_dac:?}, dac_to_out: {dac_to_out:?}");
    //
    // let out_from_fft = find_paths_short_circuit(&routes_backwards, "out", "fft");
    // let fft_to_out = find_paths_short_circuit(&routes, "fft", "out");
    // println!("out_from_fft: {out_from_fft:?}, fft_to_out: {fft_to_out:?}");

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "aaa: you hhh",
        "you: bbb ccc",
        "bbb: ddd eee",
        "ccc: ddd eee fff",
        "ddd: ggg",
        "eee: out",
        "fff: out",
        "ggg: out",
        "hhh: ccc fff iii",
        "iii: out",
    ];

    static EXAMPLE_2: &[&str] = &[
        "svr: aaa bbb",
        "aaa: fft",
        "fft: ccc",
        "bbb: tty",
        "tty: ccc",
        "ccc: ddd eee",
        "ddd: hub",
        "hub: fff",
        "eee: dac",
        "dac: fff",
        "fff: ggg hhh",
        "ggg: out",
        "hhh: out",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 5);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE_2).unwrap();

        assert_eq!(result, 2);
    }
}
