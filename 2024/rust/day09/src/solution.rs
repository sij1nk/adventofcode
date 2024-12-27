use std::collections::{BTreeMap, BTreeSet};

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
    current: Option<BlockPointer>,
}

impl<'a> Iterator for SpaceBlockIterator<'a> {
    type Item = BlockPointer;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.unwrap_or_default();

        let zip = self
            .nodes
            .iter()
            .zip(self.nodes.iter().skip(1))
            .enumerate()
            .skip(current.end);

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

            self.current = Some(BlockPointer { start: c1, end: c2 });
            return self.current;
        }

        None
    }
}

struct FileBlockIterator<'a> {
    nodes: &'a Vec<Node>,
    current: Option<BlockPointer>,
}

impl<'a> Iterator for FileBlockIterator<'a> {
    type Item = BlockPointer;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.nodes.len();
        let mut current = self.current.unwrap_or(BlockPointer {
            start: len,
            end: len,
        });

        let rev = self
            .nodes
            .iter()
            .rev()
            .enumerate()
            .skip(len - current.start);

        let mut current_id: Option<u64> = None;

        for (i, n) in rev {
            let Node::File(id) = n else {
                match current_id {
                    Some(_) => {
                        self.current = Some(BlockPointer {
                            start: len - i,
                            end: len - current.start,
                        });
                        return self.current;
                    }
                    None => continue,
                }
            };

            match current_id {
                Some(c_id) if c_id == *id => continue,
                Some(_) => {
                    self.current = Some(BlockPointer {
                        start: len - i,
                        end: len - current.start,
                    });
                    return self.current;
                }
                None => {
                    current.start = i;
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

type BlockSize = usize;
type SpaceBlocksMap = BTreeMap<BlockSize, BTreeSet<BlockPointer>>;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct BlockPointer {
    start: usize,
    end: usize,
}

impl BlockPointer {
    fn size(&self) -> usize {
        self.end - self.start
    }
}

impl Ord for BlockPointer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for BlockPointer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn build_space_blocks_map(disk: &Disk) -> SpaceBlocksMap {
    let mut map: SpaceBlocksMap = BTreeMap::new();

    for block in disk.space_block_iter() {
        map.entry(block.size()).or_default().insert(block);
    }

    map
}

fn get_first_viable_space_block(
    space_blocks_map: &SpaceBlocksMap,
    file_block: &BlockPointer,
) -> Option<BlockPointer> {
    let mut viable_space_blocks = space_blocks_map
        .iter()
        .filter(|&(size, _)| *size >= file_block.size())
        .filter_map(|(size, map)| {
            let space_block = map.first()?;

            Some((size, space_block))
        })
        .map(|(_, block)| block)
        .collect::<Vec<_>>();

    if viable_space_blocks.is_empty() {
        return None;
    }

    viable_space_blocks.sort();

    let space_block = viable_space_blocks[0];

    if space_block.start > file_block.start {
        None
    } else {
        Some(*space_block)
    }
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut disk = parse(lines);
    let cloned_disk = disk.clone();

    let mut space_blocks_map = build_space_blocks_map(&disk);

    for file_block in cloned_disk.file_block_iter() {
        let Some(space_block) = get_first_viable_space_block(&space_blocks_map, &file_block) else {
            continue;
        };

        let Node::File(file_id) = disk.inner[file_block.start] else {
            panic!(
                "Expected file block, found space block at {}",
                file_block.start
            );
        };

        space_blocks_map
            .get_mut(&space_block.size())
            .unwrap()
            .remove(&space_block);

        let remaining_space = space_block.size() - file_block.size();

        if remaining_space != 0 {
            space_blocks_map
                .entry(remaining_space)
                .or_default()
                .insert(BlockPointer {
                    start: space_block.end - remaining_space,
                    end: space_block.end,
                });
        };

        for i in (space_block.start..space_block.end).take(file_block.size()) {
            disk.inner[i] = Node::File(file_id);
        }

        for i in file_block.start..file_block.end {
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
