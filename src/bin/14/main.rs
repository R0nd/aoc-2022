use std::{
    cmp::{max, min},
    iter::from_fn,
    ops::RangeInclusive,
    str::Lines,
};

#[derive(Clone, Copy, Debug)]
enum Cell {
    Air,
    Rock,
    Sand,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    base_col: usize,
}

type Point = (usize, usize);

const INIT_POINT: Point = (500, 0);

impl Grid {
    fn new(rock_ranges: Vec<RangeInclusive<Point>>) -> Self {
        let col_range = rock_ranges.iter().fold(INIT_POINT..=INIT_POINT, |a, b| {
            min(*a.start(), *b.start())..=max(*a.end(), *b.end())
        });

        let base_col = col_range.start().0;
        let width = col_range.end().0 - col_range.start().0 + 1;
        let height = rock_ranges
            .iter()
            .map(|x| max(x.start().1, x.end().1))
            .max()
            .unwrap()
            + 1;

        let mut cells = vec![vec![Cell::Air; width]; height];
        for rock_range in rock_ranges {
            for col in rock_range.start().0..=rock_range.end().0 {
                (rock_range.start().1..=rock_range.end().1).for_each(|row| {
                    cells[row][col - base_col] = Cell::Rock;
                });
            }
        }

        Self { base_col, cells }
    }

    fn norm_col(&self, col_idx: usize) -> usize {
        col_idx - self.base_col
    }

    fn subslice(&self, (col_idx, row_idx): Point) -> [Option<Cell>; 3] {
        let row = self.cells.get(row_idx + 1);
        match row {
            None => [None; 3],
            Some(row) => (self.norm_col(col_idx) as isize - 1
                ..=self.norm_col(col_idx) as isize + 1)
                .map(|i| i.try_into().map_or(None, |i: usize| row.get(i).copied()))
                .collect::<Vec<Option<Cell>>>()
                .try_into()
                .unwrap(),
        }
    }

    fn pour(&mut self, point: Point) -> Option<()> {
        match self.subslice(point) {
            [_, Some(Cell::Air), _] => self.pour((point.0, point.1 + 1)),
            [_, None, _] => None,
            [Some(Cell::Air), _, _] => self.pour((point.0 - 1, point.1 + 1)),
            [None, _, _] => None,
            [_, _, Some(Cell::Air)] => self.pour((point.0 + 1, point.1 + 1)),
            [_, _, None] => None,
            _ if point == INIT_POINT => None,
            _ => {
                let col_idx = self.norm_col(point.0);
                self.cells[point.1][col_idx] = Cell::Sand;
                Some(())
            }
        }
    }
}

fn parse_segment(input: &str) -> Point {
    match input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()[..]
    {
        [a, b] => (a, b),
        _ => panic!(),
    }
}

fn parse_line(input: &str) -> Vec<RangeInclusive<Point>> {
    input
        .split("->")
        .map(parse_segment)
        .collect::<Vec<Point>>()
        .windows(2)
        .map(|pair| match pair {
            [a, b] => {
                if a <= b {
                    *a..=*b
                } else {
                    *b..=*a
                }
            }
            _ => panic!(),
        })
        .collect()
}

fn parse_input(input: Lines) -> Vec<RangeInclusive<Point>> {
    input.flat_map(parse_line).collect()
}

fn process_part1(input: Lines) -> usize {
    let mut grid = Grid::new(parse_input(input));
    from_fn(|| grid.pour(INIT_POINT)).count()
}

fn process_part2(input: Lines) -> usize {
    let mut ranges = parse_input(input);

    let floor_y = ranges
        .iter()
        .map(|x| max(x.start().1, x.end().1))
        .max()
        .unwrap()
        + 2;

    ranges.push((0, floor_y)..=(1000, floor_y));

    let mut grid = Grid::new(ranges);
    from_fn(|| grid.pour(INIT_POINT)).count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 93);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
