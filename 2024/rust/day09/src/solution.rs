#[derive(Debug, PartialEq, Clone, Copy)]
enum Node {
    Space,
    File(u64),
}

#[derive(Debug, Clone)]
struct Disk {
    inner: Vec<Node>,
}

impl Disk {
    fn new(nodes: Vec<Node>) -> Self {
        Self { inner: nodes }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = &Node> {
        self.inner.iter()
    }

    fn space_block_iter(&self) -> SpaceBlockIterator {
        SpaceBlockIterator {
            nodes: &self.inner,
            current: None,
        }
    }

    fn file_block_iter(&self) -> FileBlockIterator {
        FileBlockIterator {
            nodes: &self.inner,
            current: None,
        }
    }
}

fn parse<'a, I, S>(lines: I) -> Disk
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut id = 0;
    let mut nodes = Vec::new();
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
                nodes.push(Node::File(id));
            }
            id += 1;
        } else {
            for _ in 0..n {
                nodes.push(Node::Space);
            }
        }
    }

    Disk::new(nodes)
}

fn find_next_space(disk: &Disk, index: usize) -> usize {
    disk.iter()
        .skip(index + 1)
        .position(|n| *n == Node::Space)
        .unwrap()
        + index
        + 1
}

struct SpaceBlockIterator<'a> {
    nodes: &'a Vec<Node>,
    current: Option<(usize, usize)>,
}

impl<'a> Iterator for SpaceBlockIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (_, c2) = self.current.unwrap_or_default();

        let zip = self
            .nodes
            .iter()
            .zip(self.nodes.iter().skip(1))
            .enumerate()
            .skip(c2);

        for (i, (n1, n2)) in zip {
            if !(matches!(n1, Node::File(_)) && *n2 == Node::Space) {
                continue;
            }

            let c1 = i + 1;

            let mut c2 = c1;
            let iter = self.nodes.iter().skip(c1);
            for next in iter {
                if *next == Node::Space {
                    c2 += 1;
                } else {
                    break;
                }
            }

            self.current = Some((c1, c2));
            return self.current;
        }

        None
    }
}

struct FileBlockIterator<'a> {
    nodes: &'a Vec<Node>,
    current: Option<(usize, usize)>,
}

impl<'a> Iterator for FileBlockIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.nodes.len();
        let (mut c1, _) = self.current.unwrap_or((len, len));

        let rev = self.nodes.iter().rev().enumerate().skip(len - c1);

        let mut current_id: Option<u64> = None;

        for (i, n) in rev {
            let Node::File(id) = n else {
                match current_id {
                    Some(_) => {
                        self.current = Some((len - i, len - c1));
                        return self.current;
                    }
                    None => continue,
                }
            };

            match current_id {
                Some(c_id) if c_id == *id => continue,
                Some(_) => {
                    self.current = Some((len - i, len - c1));
                    return self.current;
                }
                None => {
                    c1 = i;
                    current_id = Some(*id);
                }
            }
        }

        // does not return first (last) block but don't care
        None
    }
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
        disk.inner[space_p] = disk.inner[file_p];
        disk.inner[file_p] = Node::Space;

        space_p = find_next_space(&disk, space_p);
        file_p = find_next_file(&disk, file_p);
    }

    Ok(disk
        .iter()
        .enumerate()
        .filter_map(|(i, n)| match n {
            Node::Space => None,
            Node::File(id) => Some((i, id)),
        })
        .fold(0, |acc, (i, id)| acc + (i as u64) * id))
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut disk = parse(lines);
    let spare_disk = disk.clone(); // not sure if needed

    let mut skip_file = 0;

    loop {
        let Some(file_block) = spare_disk.file_block_iter().nth(skip_file) else {
            break;
        };

        skip_file += 1;

        let file_len = file_block.1 - file_block.0;

        let Some(space_block) = disk.space_block_iter().find(|b| {
            let len = b.1 - b.0;
            len >= file_len && b.0 < file_block.0
        }) else {
            continue;
        };

        for i in (space_block.0..space_block.1).take(file_len) {
            disk.inner[i] = disk.inner[file_block.0];
        }

        for i in file_block.0..file_block.1 {
            disk.inner[i] = Node::Space;
        }
    }

    Ok(disk
        .iter()
        .enumerate()
        .filter_map(|(i, n)| match n {
            Node::Space => None,
            Node::File(id) => Some((i, id)),
        })
        .fold(0, |acc, (i, id)| acc + (i as u64) * id))
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

        assert_eq!(result, 2858);
    }
}
