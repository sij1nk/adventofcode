#[derive(Debug, PartialEq, Clone, Copy)]
enum Node {
    Space,
    File(u64),
}

type Disk = Vec<Node>;

fn parse<'a, I, S>(lines: I) -> Disk
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut id = 0;
    let mut disk = Vec::new();
    let line = lines.into_iter().map(|s| s.as_ref()).next().unwrap();
    for (i, n) in line
        .bytes()
        .map(|b| {
            let c = char::from(b);
            c.to_digit(10).unwrap()
        })
        .enumerate()
    {
        if i % 2 == 0 {
            for _ in 0..n {
                disk.push(Node::File(id));
            }
            id += 1;
        } else {
            for _ in 0..n {
                disk.push(Node::Space);
            }
        }
    }

    disk
}

fn find_next_space(disk: &Disk, index: usize) -> usize {
    disk.iter()
        .skip(index + 1)
        .position(|n| *n == Node::Space)
        .unwrap()
        + index
        + 1
}

fn find_next_file(disk: &Disk, index: usize) -> usize {
    let index_from_end = disk.len() - index;
    index
        - disk
            .iter()
            .rev()
            .skip(index_from_end)
            .position(|n| matches!(*n, Node::File(_)))
            .unwrap()
        - 1
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut disk = parse(lines);

    let mut space_p = find_next_space(&disk, 0);
    let mut file_p = disk.len() - 1;

    while space_p < file_p {
        disk[space_p] = disk[file_p];
        disk[file_p] = Node::Space;

        space_p = find_next_space(&disk, space_p);
        file_p = find_next_file(&disk, file_p);
    }

    Ok(disk
        .iter()
        .filter_map(|n| match n {
            Node::Space => None,
            Node::File(id) => Some(id),
        })
        .enumerate()
        .fold(0, |acc, (i, id)| acc + (i as u64) * id))
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["2333133121414131402"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 1928);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
