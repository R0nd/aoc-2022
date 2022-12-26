use std::str::Lines;

type Point = (usize, usize);
type Grid = Vec<Vec<u32>>;
type Visits = Vec<Vec<bool>>;

fn parse_input(input: Lines) -> (Point, Point, Grid) {
    let mut start = None;
    let mut end = None;

    let mut grid = vec![];
    for (y, line) in input.enumerate() {
        let mut line_vec = vec![];
        for (x, char) in line.chars().enumerate() {
            let value = match char {
                'a'..='z' => char.to_digit(36).unwrap() - 10,
                'S' => {
                    start = Some((x, y));
                    0
                }
                'E' => {
                    end = Some((x, y));
                    25
                }
                _ => panic!(),
            };
            line_vec.push(value);
        }
        grid.push(line_vec);
    }

    (start.unwrap(), end.unwrap(), grid)
}

fn adjacent<T>(source: &[Vec<T>], row: usize, col: usize) -> [Option<T>; 4]
where
    T: Copy,
{
    [(-1, 0), (0, -1), (0, 1), (1, 0)].map(|(drow, dcol): (isize, isize)| {
        source
            .get((row as isize + drow) as usize)
            .and_then(|row| row.get((col as isize + dcol) as usize))
            .copied()
    })
}

fn find_path<F: Fn(&Grid, &Visits) -> bool>(
    iter: usize,
    done_predicate: F,
    grid: Grid,
    visits: Visits,
) -> usize {
    if done_predicate(&grid, &visits) {
        return iter;
    }

    let mut next_visits = vec![];

    for (row, line) in visits.iter().enumerate() {
        let mut next_line = vec![];
        for (col, v) in line.iter().enumerate() {
            let next_v = *v
                || adjacent(&grid, row, col)
                    .iter()
                    .map(|mx| mx.map(|x| x <= grid[row][col] + 1).unwrap_or_default())
                    .zip(adjacent(&visits, row, col))
                    .any(|(a, b)| a && b.unwrap_or_default());
            next_line.push(next_v);
        }
        next_visits.push(next_line);
    }

    find_path(iter + 1, done_predicate, grid, next_visits)
}

fn init_visits(height: usize, width: usize, start: Point) -> Visits {
    (0..height)
        .map(|row| (0..width).map(|col| (col, row) == start).collect())
        .collect()
}

fn process_part1(input: Lines) -> usize {
    let (start, end, grid) = parse_input(input);
    let init_visits = init_visits(grid.len(), grid[0].len(), end);
    find_path(0, |_, v| v[start.1][start.0], grid, init_visits)
}

fn process_part2(input: Lines) -> usize {
    let (_, end, grid) = parse_input(input);
    let init_visits = init_visits(grid.len(), grid[0].len(), end);
    find_path(
        0,
        |g, v| {
            g.iter()
                .flatten()
                .zip(v.iter().flatten())
                .any(|(g, v)| *g == 0 && *v)
        },
        grid,
        init_visits,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 31);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 29);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
