use std::collections::HashMap;
use std::fmt;

use crate::util;

#[derive(Debug)]
struct Dir {
    name: String,
    absolute_name: String,
    nodes: Vec<Node>,
}

impl Dir {
    pub fn new(name: &str, absolute_name: String) -> Self {
        Self {
            name: name.into(),
            absolute_name,
            nodes: vec![],
        }
    }

    pub fn get_size(&self, cache: &mut HashMap<String, usize>) -> usize {
        let mut size = 0;

        for node in self.nodes.iter() {
            match node {
                Node::Dir(dir) => {
                    // I'm not actually using the cache as a cache. I couldn't get it to work in a
                    // small amount of time, so I gave up
                    let dir_size = dir.get_size(cache);
                    size += dir_size;
                }
                Node::File(file) => {
                    size += file.size;
                }
            }
        }

        cache.insert(self.absolute_name.clone(), size);

        size
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }
}

enum Node {
    Dir(Dir),
    File(File),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Dir(dir) => write!(f, "{}", dir.name),
            Node::File(file) => write!(f, "{}: {}", file.name, file.size),
        }
    }
}

struct Filesystem {
    root: Dir,
    size_cache: Option<HashMap<String, usize>>,
    cwd: Vec<String>,
}

impl Filesystem {
    pub fn new() -> Self {
        Self {
            root: Dir::new("/", "/".into()),
            size_cache: None,
            cwd: vec!["/".into()],
        }
    }

    fn get_cwd_string(&self) -> String {
        let (root, rest) = self.cwd.split_at(1);

        format!("{}{}", root[0], rest.join("/"))
    }

    fn calculate_size_cache(&mut self) {
        let mut cache = HashMap::new();

        let _ = self.root.get_size(&mut cache);

        self.size_cache = Some(cache);
    }

    pub fn sum_of_dir_sizes_below_size(&mut self, dir_size_limit: usize) -> usize {
        if self.size_cache.is_none() {
            self.calculate_size_cache();
        }

        self.size_cache
            .as_ref()
            .unwrap()
            .values()
            .filter(|&&v| v <= dir_size_limit)
            .sum()
    }

    pub fn smallest_dir_size_above_size(&mut self, min_size: usize) -> usize {
        if self.size_cache.is_none() {
            self.calculate_size_cache();
        }

        self.size_cache
            .as_ref()
            .unwrap()
            .values()
            .fold(usize::MAX, |acc, elem| {
                if acc > *elem && *elem >= min_size {
                    *elem
                } else {
                    acc
                }
            })
    }

    pub fn get_total_size(&mut self) -> usize {
        if self.size_cache.is_none() {
            self.calculate_size_cache();
        }

        *self.size_cache.as_ref().unwrap().get("/").unwrap()
    }

    pub fn build<'a, I, S>(lines: I) -> Result<Self, util::Error>
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
    {
        let mut fs = Self::new();
        let mut cwd_string = String::new();
        let mut current_nodes = vec![];

        for line in lines.into_iter().map(|l| l.as_ref()) {
            let mut split = line.split(' ');

            match split.next().unwrap() {
                "$" => {
                    if !current_nodes.is_empty() {
                        fs.add_nodes(&mut current_nodes)?;
                        current_nodes.clear();
                    }

                    if let "cd" = split.next().unwrap() {
                        fs.cd(split.next().unwrap());
                        cwd_string = fs.get_cwd_string();
                    }
                }
                "dir" => {
                    let name = split.next().unwrap();
                    let mut absolute_name = cwd_string.clone();
                    absolute_name.push('/');
                    absolute_name.push_str(name);
                    current_nodes.push(Node::Dir(Dir::new(name, absolute_name)));
                }
                size => {
                    let size = size
                        .parse::<usize>()
                        .map_err(|_| util::Error::new("Could not parse file size"))?;
                    let name = split.next().unwrap();
                    current_nodes.push(Node::File(File::new(name, size)));
                }
            }
        }

        if !current_nodes.is_empty() {
            fs.add_nodes(&mut current_nodes)?;
        }

        Ok(fs)
    }

    pub fn cd(&mut self, dir: &str) {
        match dir {
            "/" => {
                self.cwd.truncate(1);
            }
            ".." => {
                self.cwd.pop();
            }
            s => {
                self.cwd.push(s.into());
            }
        }
    }

    pub fn add_nodes(&mut self, nodes: &mut Vec<Node>) -> Result<(), util::Error> {
        let mut wd = &mut self.root;

        for part in self.cwd.iter().skip(1) {
            wd = wd
                .nodes
                .iter_mut()
                .filter_map(|node| {
                    if let Node::Dir(dir) = node {
                        Some(dir)
                    } else {
                        None
                    }
                })
                .find(|dir| dir.name == *part)
                .unwrap();
        }

        wd.nodes.append(nodes);

        Ok(())
    }
}

pub fn part1<'a, I, S>(lines: I) -> Result<usize, util::Error>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut fs = Filesystem::build(lines)?;

    Ok(fs.sum_of_dir_sizes_below_size(100000))
}

pub fn part2<'a, I, S>(lines: I) -> Result<usize, util::Error>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let disk_space = 70000000;
    let needed_free_space = 30000000;

    let mut fs = Filesystem::build(lines)?;

    let total_size = fs.get_total_size();

    let space_to_free_up = needed_free_space - (disk_space - total_size);

    Ok(fs.smallest_dir_size_above_size(space_to_free_up))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 95437);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 24933642);
    }
}
