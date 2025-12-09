use std::{cell::RefCell, collections::BTreeMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct V3 {
    x: u64,
    y: u64,
    z: u64,
}

impl V3 {
    fn dist(&self, other: &V3) -> u64 {
        let xd = self.x.abs_diff(other.x);
        let yd = self.y.abs_diff(other.y);
        let zd = self.z.abs_diff(other.z);

        let d = ((xd * xd + yd * yd + zd * zd) as f64).sqrt();

        // Hack to turn float into something that can be used as a key in a map
        (d * 1000000f64).floor() as u64
    }
}

pub fn part1<'a, I, S>(lines: I, mut connections: u32) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut points: Vec<V3> = vec![];

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let mut words = line.split(",");
        let x = words.next().expect("x to exist").parse::<u64>()?;
        let y = words.next().expect("y to exist").parse::<u64>()?;
        let z = words.next().expect("z to exist").parse::<u64>()?;

        points.push(V3 { x, y, z });
    }

    let mut ds: BTreeMap<u64, Vec<(&V3, &V3)>> = BTreeMap::new();

    // TODO:
    // This is VERY expensive (0.17s) but I can't come up with anything better...
    // Can I discover the shortest path without having to walk all paths?
    for (i, p) in points.iter().enumerate() {
        for pp in points.iter().skip(i + 1) {
            let d = p.dist(pp);
            ds.entry(d).or_default().push((p, pp));
        }
    }

    // This barely makes a difference in the bench as opposed to Vec<Vec<&V3>>
    let mut graphs: Vec<RefCell<Vec<&V3>>> = vec![];

    for vs in ds.values() {
        for &(v1, v2) in vs.iter() {
            if connections == 0 {
                break;
            }

            connections -= 1;

            let vec1found = graphs.iter().find(|g| g.borrow().contains(&v1));
            let vec2found = graphs
                .iter()
                .enumerate()
                .find(|(_, g)| g.borrow().contains(&v2));

            if let Some(vec1found) = vec1found {
                if let Some((v2i, vec2found)) = vec2found {
                    if vec1found == vec2found {
                        continue;
                    }

                    vec1found
                        .borrow_mut()
                        .extend_from_slice(&vec2found.borrow());

                    graphs.swap_remove(v2i);
                } else {
                    vec1found.borrow_mut().push(v2);
                }
            } else if let Some((_, vec2found)) = vec2found {
                vec2found.borrow_mut().push(v1);
            } else {
                graphs.push(RefCell::new(vec![v1, v2]));
            }
        }
    }

    let mut gl: Vec<_> = graphs.iter().map(|v| v.borrow().len() as u64).collect();
    gl.sort();

    Ok(gl.iter().rev().take(3).product())
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut points: Vec<V3> = vec![];

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let mut words = line.split(",");
        let x = words.next().expect("x to exist").parse::<u64>()?;
        let y = words.next().expect("y to exist").parse::<u64>()?;
        let z = words.next().expect("z to exist").parse::<u64>()?;

        points.push(V3 { x, y, z });
    }

    let mut ds: BTreeMap<u64, Vec<(&V3, &V3)>> = BTreeMap::new();

    for (i, p) in points.iter().enumerate() {
        for pp in points.iter().skip(i + 1) {
            let d = p.dist(pp);
            ds.entry(d).or_default().push((p, pp));
        }
    }

    let mut graphs: Vec<Vec<&V3>> = vec![];

    for vs in ds.values() {
        for &(v1, v2) in vs.iter() {
            let vec1found = graphs.iter().enumerate().find(|(_, g)| g.contains(&v1));
            let vec2found = graphs.iter().enumerate().find(|(_, g)| g.contains(&v2));

            if let Some((v1i, vec1found)) = vec1found {
                if let Some((v2i, vec2found)) = vec2found {
                    if vec1found == vec2found {
                        continue;
                    }

                    let mut new_vec = vec1found.clone();
                    new_vec.extend_from_slice(vec2found);

                    let (mini, maxi) = if v1i < v2i { (v1i, v2i) } else { (v2i, v1i) };
                    graphs.swap_remove(maxi);
                    graphs.swap_remove(mini);
                    graphs.push(new_vec);
                } else {
                    let mut new_vec = vec1found.clone();
                    new_vec.push(v2);

                    graphs.swap_remove(v1i);
                    graphs.push(new_vec);
                }
            } else if let Some((v2i, vec2found)) = vec2found {
                let mut new_vec = vec2found.clone();
                new_vec.push(v1);

                graphs.swap_remove(v2i);
                graphs.push(new_vec);
            } else {
                let new_vec = vec![v1, v2];
                graphs.push(new_vec);
            }

            if graphs.len() == 1 && graphs[0].len() == points.len() {
                return Ok(v1.x * v2.x);
            }
        }
    }

    panic!("No result")
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE, 10).unwrap();

        assert_eq!(result, 40);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 25272);
    }
}
