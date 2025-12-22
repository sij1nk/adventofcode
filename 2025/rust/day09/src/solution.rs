#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct V2 {
    x: i64,
    y: i64,
}

impl V2 {
    fn area(&self, other: &V2) -> u64 {
        (self.y.abs_diff(other.y) + 1) * (self.x.abs_diff(other.x) + 1)
    }

    fn rect_polyedges(&self, other: &V2) -> [PolyEdge; 4] {
        let mut p1 = *self;
        let mut p2 = *other;

        if p1.x > p2.x {
            std::mem::swap(&mut p1, &mut p2);
        }

        let p3 = V2 { x: p1.x, y: p2.y };
        let p4 = V2 { x: p2.x, y: p1.y };

        let p1_on_top = p1.y > p2.y;

        [
            PolyEdge::new(Edge::new(p1, p3), Dir::Right),
            PolyEdge::new(
                Edge::new(p3, p2),
                // if p1_on_top { Dir::Down } else { Dir::Up },
                if p1_on_top { Dir::Up } else { Dir::Down },
            ),
            PolyEdge::new(Edge::new(p2, p4), Dir::Left),
            PolyEdge::new(
                Edge::new(p4, p1),
                // if p1_on_top { Dir::Up } else { Dir::Down },
                if p1_on_top { Dir::Down } else { Dir::Up },
            ),
        ]
    }

    fn sub(&self, other: &V2) -> V2 {
        V2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut dots: Vec<V2> = vec![];

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (x_str, y_str) = line.split_once(",").expect("line to contain a comma");
        let x = x_str.parse::<i64>()?;
        let y = y_str.parse::<i64>()?;
        dots.push(V2 { x, y })
    }

    let mut max = 0;

    for (i, d) in dots.iter().enumerate() {
        for dd in dots.iter().skip(i + 1) {
            let area = d.area(dd);
            if area > max {
                max = area;
            }
        }
    }

    Ok(max)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    p1: V2,
    p2: V2,
}

impl Edge {
    fn new(p1: V2, p2: V2) -> Self {
        Self { p1, p2 }
    }

    fn contains(&self, p: &V2) -> bool {
        let (p1, p2) = (self.p1, self.p2);
        let is_x_aligned = p1.x == p2.x;

        if is_x_aligned {
            p.x == p1.x && p.y >= p1.y.min(p2.y) && p.y <= p1.y.max(p2.y)
        } else {
            p.y == p1.y && p.x >= p1.x.min(p2.x) && p.x <= p1.x.max(p2.x)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PolyEdge {
    inner: Edge,
    inside_dir: Dir,
}

impl PolyEdge {
    fn new(inner: Edge, inside_dir: Dir) -> Self {
        Self { inner, inside_dir }
    }

    fn intersects_inside(&self, other: &Edge) -> bool {
        let Some(isect) = self.intersection(other) else {
            return false;
        };

        if isect == self.inner.p1 || isect == self.inner.p2 {
            return false;
        }

        match self.inside_dir {
            Dir::Left => other.p1.x < isect.x || other.p2.x < isect.x,
            Dir::Up => other.p1.y > isect.y || other.p2.y > isect.y,
            Dir::Right => other.p1.x > isect.x || other.p2.x > isect.x,
            Dir::Down => other.p1.y < isect.y || other.p2.y < isect.y,
        }
    }

    // TODO: check if this can be simplified since our lines are axis-aligned
    fn intersection(&self, other: &Edge) -> Option<V2> {
        let (p1, p2) = (self.inner.p1, self.inner.p2);
        let (p3, p4) = (other.p1, other.p2);

        let pxd = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
        let pyd = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);

        if pxd == 0 || pyd == 0 {
            return None;
        }

        let px = ((p1.x * p2.y - p1.y * p2.x) * (p3.x - p4.x)
            - (p1.x - p2.x) * (p3.x * p4.y - p3.y * p4.x))
            / pxd;
        let py = ((p1.x * p2.y - p1.y * p2.x) * (p3.y - p4.y)
            - (p1.y - p2.y) * (p3.x * p4.y - p3.y * p4.x))
            / pyd;

        let p = V2 { x: px, y: py };

        if self.inner.contains(&p) && other.contains(&p) {
            Some(p)
        } else {
            None
        }
    }

    fn overlaps(&self, other: &PolyEdge) -> bool {
        let p1 = self.inner.p1;
        let p2 = self.inner.p2;

        let p3 = other.inner.p1;
        let p4 = other.inner.p2;

        let is_self_x_aligned = p1.y == p2.y;
        let is_other_x_aligned = p3.y == p4.y;

        if is_self_x_aligned != is_other_x_aligned {
            return false;
        }

        let (mut a, mut b, mut c, mut d) = if is_self_x_aligned {
            if p1.y != p3.y {
                return false;
            }
            (p1.x, p2.x, p3.x, p4.x)
        } else {
            if p1.x != p3.x {
                return false;
            }
            (p1.y, p2.y, p3.y, p4.y)
        };

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if c > d {
            std::mem::swap(&mut c, &mut d);
        }
        if a > c {
            std::mem::swap(&mut a, &mut c);
            std::mem::swap(&mut b, &mut d);
        }

        let edges_share_point = a == c || b == d;
        let non_trivial_overlap = b > c;

        edges_share_point || non_trivial_overlap
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

impl Dir {
    fn turn_left(&self) -> Dir {
        match self {
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
        }
    }

    fn turn_right(&self) -> Dir {
        match self {
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
        }
    }
}

fn assign_edge_inside_directions(edges: Vec<Edge>) -> Vec<PolyEdge> {
    let mut inside_dirs: Vec<Option<Dir>> = vec![None; edges.len()];

    // leftmost edge must be vertical, with the inside direction to the right
    let (leftmost_edge_i, leftmost_edge) = edges
        .iter()
        .enumerate()
        .filter(|(_, e)| e.p1.x == e.p2.x)
        .min_by_key(|(_, e)| e.p1.x)
        .expect("at least one edge");

    let mut current_edge = leftmost_edge;
    let mut current_dir = Dir::Right;

    inside_dirs[leftmost_edge_i] = Some(current_dir);

    for (i, edge) in edges
        .iter()
        .enumerate()
        .cycle()
        .skip(leftmost_edge_i + 1)
        .take(edges.len() - 1)
    {
        let (p1, p2, p3) = (current_edge.p1, current_edge.p2, edge.p2); // current_edge.p2 == edge.p1

        let d1 = p2.sub(&p1);
        let d2 = p3.sub(&p2);

        let is_right_turn = d1.x * d2.y - d1.y * d2.x < 0;

        if is_right_turn {
            current_dir = current_dir.turn_right();
        } else {
            current_dir = current_dir.turn_left();
        }

        inside_dirs[i] = Some(current_dir);
        current_edge = edge;
    }

    edges
        .iter()
        .zip(inside_dirs)
        .map(|(e, dir)| PolyEdge {
            inner: *e,
            inside_dir: dir.expect("all edges to have assigned directions"),
        })
        .collect()
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut points: Vec<V2> = vec![];

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (x_str, y_str) = line.split_once(",").expect("line to contain a comma");
        let x = x_str.parse::<i64>()?;
        let y = y_str.parse::<i64>()?;
        points.push(V2 { x, y })
    }

    let mut edges: Vec<Edge> = vec![];

    // collect edges
    for (p1, p2) in points
        .iter()
        .zip(points.iter().skip(1).chain(std::iter::once(&points[0])))
    {
        edges.push(Edge { p1: *p1, p2: *p2 })
    }
    let edges = assign_edge_inside_directions(edges);

    let mut max_area = 0;

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            let is_line = p1.x == p2.x || p1.y == p2.y;
            if is_line {
                // puzzle doesn't consider this to be a valid rectangle
                continue;
            }

            let rect_polyedges = p1.rect_polyedges(p2);

            // first, let's check if there are any input edges which poke a hole in our rectangle
            let is_cut = rect_polyedges
                .iter()
                .any(|rpe| edges.iter().any(|e| rpe.intersects_inside(&e.inner)));

            if is_cut {
                continue;
            }

            // then, check if our rectangle is outside of the polygon that's formed by the input edges
            let is_outside = rect_polyedges.iter().any(|rpe| {
                edges
                    .iter()
                    .any(|e| rpe.overlaps(e) && rpe.inside_dir != e.inside_dir)
            });

            if is_outside {
                continue;
            }

            let area = p1.area(p2);
            if area > max_area {
                max_area = area;
            }
        }
    }

    Ok(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 50);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 24);
    }
}
