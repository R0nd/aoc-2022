use std::{collections::HashMap, str::Lines};

type Grid = Vec<Vec<Option<bool>>>;

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
enum Path {
    Move(usize),
    Turn(Dir),
}

type Point = (usize, usize);

type Basis = (isize, isize);

fn parse_line(input: &str) -> Vec<Option<bool>> {
    input
        .chars()
        .map(|c| match c {
            '.' => Some(true),
            '#' => Some(false),
            _ => None,
        })
        .collect()
}

fn parse_path(input: &str) -> Vec<Path> {
    let mut digit_buf = "".to_owned();
    let mut result = vec![];
    for c in input.chars() {
        if c.is_numeric() {
            digit_buf.push(c);
        } else {
            if !digit_buf.is_empty() {
                result.push(Path::Move(digit_buf.parse().unwrap()));
            }
            digit_buf.clear();
            match c {
                'L' => result.push(Path::Turn(Dir::Left)),
                'R' => result.push(Path::Turn(Dir::Right)),
                _ => panic!(),
            }
        }
    }
    if !digit_buf.is_empty() {
        result.push(Path::Move(digit_buf.parse().unwrap()));
    }

    result
}

fn parse_input(input: &mut Lines) -> (Grid, Vec<Path>) {
    (
        input
            .take_while(|l| !l.is_empty())
            .map(parse_line)
            .collect(),
        parse_path(input.next().unwrap()),
    )
}

fn mv(grid: &Grid, (x, y): Point, facing: Basis, n: usize) -> Point {
    let mut slice = match facing {
        (-1 | 1, 0) => grid[y]
            .iter()
            .enumerate()
            .map(|(x, v)| ((x, y), v))
            .collect::<Vec<_>>(),
        (0, -1 | 1) => grid
            .iter()
            .map(|row| row.get(x).unwrap_or(&None))
            .enumerate()
            .map(|(y, v)| ((x, y), v))
            .collect(),
        _ => panic!(),
    };

    if matches!(facing, (-1 | 0, -1 | 0)) {
        slice.reverse();
    }

    slice.retain(|(_, c)| c.is_some());

    let result = slice
        .iter()
        .cycle()
        .skip_while(|(p, _)| p != &(x, y))
        .take_while(|(_, c)| c.unwrap_or_default())
        .take(n + 1)
        .last()
        .unwrap()
        .0;

    result
}

fn turn((x, y): Basis, dir: &Dir) -> Basis {
    match dir {
        Dir::Left => (y, -x),
        Dir::Right => (-y, x),
    }
}

fn walk(grid: &Grid, coord: Point, facing: Basis, path: &Path) -> (Point, Basis) {
    match path {
        Path::Move(n) => (mv(grid, coord, facing, *n), facing),
        Path::Turn(dir) => (coord, turn(facing, dir)),
    }
}

fn score((col, row): &Point, facing: &Basis) -> usize {
    (row + 1) * 1000
        + (col + 1) * 4
        + match facing {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            _ => panic!(),
        }
}

fn process_part1(input: &mut Lines) -> usize {
    let (grid, path) = parse_input(input);

    let start = (
        grid[0]
            .iter()
            .enumerate()
            .find(|(_, &c)| c == Some(true))
            .unwrap()
            .0,
        0,
    );

    let (pos, facing) = path.iter().fold((start, (1, 0)), |(coord, facing), p| {
        walk(&grid, coord, facing, p)
    });

    score(&pos, &facing)
}

type Node = HashMap<Basis, (Point, Basis)>;

fn add((px, py): &Point, (bx, by): &Basis) -> Option<Point> {
    (*px as isize + bx)
        .try_into()
        .ok()
        .zip((*py as isize + by).try_into().ok())
}

fn opp((x, y): &Basis) -> Basis {
    (-x, -y)
}

struct GraphIterator<'a> {
    graph: &'a HashMap<Point, Node>,
    pos: Point,
    facing: Basis,
}

impl GraphIterator<'_> {
    fn new<'a>(
        graph: &'a HashMap<Point, Node>,
        &pos: &Point,
        &facing: &Basis,
    ) -> GraphIterator<'a> {
        GraphIterator { graph, pos, facing }
    }
}

impl Iterator for GraphIterator<'_> {
    type Item = (Point, Basis);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.graph.get(&self.pos).unwrap().get(&self.facing);

        if let Some((pos, facing)) = result {
            self.pos = *pos;
            self.facing = *facing;
        }

        result.copied()
    }
}

struct BiRange {
    to: usize,
    cur: Option<usize>,
    asc: bool,
}

impl BiRange {
    fn new(from: usize, to: usize) -> Self {
        Self {
            cur: Some(from),
            to,
            asc: from <= to,
        }
    }
}

impl Iterator for BiRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.cur {
            if self.asc {
                let next = cur + 1;
                self.cur = if next > self.to { None } else { Some(next) };
                Some(cur)
            } else {
                let next = if cur == 0 { None } else { Some(cur - 1) };
                self.cur = next.filter(|n| n >= &self.to);
                Some(cur)
            }
        } else {
            None
        }
    }
}

fn point_range(&(ax, ay): &Point, &(bx, by): &Point) -> impl Iterator<Item = Point> {
    BiRange::new(ax, bx).flat_map(move |x| BiRange::new(ay, by).map(move |y| (x, y)))
}

fn process_part2(
    input: &mut Lines,
    stitches: &[(Point, Point, Basis, Point, Point, Basis)],
) -> usize {
    let (grid, path) = parse_input(input);

    let mut nodes = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, cell)| cell.unwrap_or_default())
                .map(move |(x, _)| ((x, y), HashMap::<_, _>::new()))
        })
        .collect::<HashMap<_, _>>();

    let bases = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    for (p, edges) in nodes.iter_mut() {
        for b in bases {
            if let Some((nx, ny)) = add(p, &b) {
                if grid
                    .get(ny)
                    .and_then(|row| row.get(nx).copied())
                    .flatten()
                    .unwrap_or_default()
                {
                    edges.insert(b, ((nx, ny), b));
                }
            }
        }
    }

    for (from_a, from_b, from_edge, to_a, to_b, to_edge) in stitches {
        let from = point_range(from_a, from_b);
        let to = point_range(to_a, to_b);

        for (f, t) in from.zip(to) {
            if nodes.contains_key(&f) && nodes.contains_key(&t) {
                let mut from_edges = nodes.remove(&f).unwrap();
                let mut to_edges = nodes.remove(&t).unwrap();
                from_edges.insert(*from_edge, (t, opp(to_edge)));
                to_edges.insert(*to_edge, (f, opp(from_edge)));
                nodes.insert(f, from_edges);
                nodes.insert(t, to_edges);
            }
        }
    }

    let start = (
        grid[0]
            .iter()
            .enumerate()
            .find(|(_, &c)| c == Some(true))
            .unwrap()
            .0,
        0,
    );
    let start_facing = (1, 0);

    let (pos, facing) = path
        .iter()
        .fold((start, start_facing), |(pos, facing), p| match p {
            Path::Turn(d) => (pos, turn(facing, d)),
            Path::Move(n) => GraphIterator::new(&nodes, &pos, &facing)
                .take(*n)
                .last()
                .unwrap_or((pos, facing)),
        });

    score(&pos, &facing)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(&mut INPUT.lines());
        assert_eq!(result, 6032);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(
            &mut INPUT.lines(),
            &[
                ((8, 0), (11, 0), (0, -1), (3, 4), (0, 4), (0, -1)),
                ((8, 0), (8, 3), (-1, 0), (4, 4), (7, 4), (0, -1)),
                ((11, 0), (11, 3), (1, 0), (11, 11), (11, 8), (1, 0)),
                ((11, 4), (11, 7), (1, 0), (15, 8), (12, 8), (0, -1)),
                ((0, 4), (0, 7), (-1, 0), (15, 11), (12, 11), (0, 1)),
                ((0, 7), (3, 7), (0, 1), (11, 11), (8, 11), (0, 1)),
                ((4, 7), (7, 7), (0, 1), (8, 11), (8, 8), (-1, 0)),
            ],
        );
        assert_eq!(result, 5031);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(&mut input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(
        &mut input.lines(),
        &[
            ((50, 0), (99, 0), (0, -1), (0, 150), (0, 199), (-1, 0)),
            ((100, 0), (149, 0), (0, -1), (0, 199), (49, 199), (0, 1)),
            ((149, 0), (149, 49), (1, 0), (99, 149), (99, 100), (1, 0)),
            ((50, 0), (50, 49), (-1, 0), (0, 149), (0, 100), (-1, 0)),
            ((100, 49), (149, 49), (0, 1), (99, 50), (99, 99), (1, 0)),
            ((50, 50), (50, 99), (-1, 0), (0, 100), (49, 100), (0, -1)),
            ((50, 149), (99, 149), (0, 1), (49, 150), (49, 199), (1, 0)),
        ],
    );
    println!("{}", result_part2);
}
