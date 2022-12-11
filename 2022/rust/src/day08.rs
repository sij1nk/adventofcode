use std::collections::{HashSet, HashMap};

struct Map {
    trees: Vec<Vec<u32>>,
}

impl Map {
    pub fn new() -> Self {
        Self { trees: vec![] }
    }

    pub fn get_visible_trees(&self) -> usize {
        let mut visible_tree_coords = HashSet::<(usize, usize)>::new();

        let mut up_maps = HashMap::<usize, HashMap<u32, (usize, usize)>>::new();
        let mut down_maps = HashMap::<usize, HashMap<u32, (usize, usize)>>::new();

        for (rowi, row) in self.trees.iter().enumerate() {
            let mut left = HashMap::<u32, (usize, usize)>::new();
            let mut right = HashMap::<u32, (usize, usize)>::new();

            for (i, &tree) in row.iter().enumerate() {
                let up = up_maps.entry(i).or_insert_with(HashMap::new);
                let down = down_maps.entry(i).or_insert_with(HashMap::new);

                let c = (i, rowi);

                if let Some(&max) = left.keys().max() {
                    if tree > max {
                        left.insert(tree, c);
                    }
                } else {
                    left.insert(tree, c);
                }

                if let Some(&max) = up.keys().max() {
                    if tree > max {
                        up.insert(tree, c);
                    }
                } else {
                    up.insert(tree, c);
                }
                
                for i in 0..=tree {
                    right.remove(&i);
                }
                right.insert(tree, c);

                for i in 0..=tree {
                    down.remove(&i);
                }
                down.insert(tree, c);
            }

            visible_tree_coords.extend(left.values());
            visible_tree_coords.extend(right.values());
        }

        visible_tree_coords.extend(up_maps.values().flat_map(|m| m.values()));
        visible_tree_coords.extend(down_maps.values().flat_map(|m| m.values()));

        visible_tree_coords.len()
    }
 
    fn calculate_scenic_score_before(&self, before: &[u32], tree: u32) -> usize {
        before.iter()
            .rev()
            .enumerate()
            .find(|(_, &other)| other >= tree)
            .map(|(i, _)| i + 1)
            .unwrap_or(before.len())
    }

    fn calculate_scenic_score_after(&self, after: &[u32], tree: u32) -> usize {
        after.iter()
            .skip(1)
            .enumerate()
            .find(|(_, &other)| other >= tree)
            .map(|(i, _)| i + 1)
            .unwrap_or(after.len() - 1)
    }

    fn calculate_scenic_score(&self, (x, y): (usize, usize), tree: u32) -> usize {
        let row = &self.trees[y];
        let (left, right) = row.split_at(x);

        let column = self.trees.iter().map(|row| row[x]).collect::<Vec<_>>();
        let (up, down) = column.split_at(y);

        let up_score = self.calculate_scenic_score_before(up, tree);
        let down_score = self.calculate_scenic_score_after(down, tree);
        let left_score = self.calculate_scenic_score_before(left, tree);
        let right_score = self.calculate_scenic_score_after(right, tree);

        up_score * down_score * left_score * right_score
    }

    pub fn get_highest_scenic_score(&self) -> usize {
        let mut max = 0;

        // We're skipping the trees on the edges, because if the score is 0 in at least one
        // direction, the product will be 0
        let mut row_iter = self.trees.iter().enumerate().skip(1);
        let _ = row_iter.next_back();

        for (rowi, row) in row_iter {
            let mut iter = row.iter().enumerate().skip(1);
            let _ = iter.next_back();

            for (i, tree) in iter {
                let c = (i, rowi);

                let score = self.calculate_scenic_score(c, *tree);
                if max < score {
                    max = score;
                }
            }
        }

        max
    }
}

pub fn part1<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut map = Map::new();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        map.trees.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
    }

    Some(map.get_visible_trees())
}

pub fn part2<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut map = Map::new();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        map.trees.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>());
    }

    Some(map.get_highest_scenic_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 21);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 8);
    }
}
