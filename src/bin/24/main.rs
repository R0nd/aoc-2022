use std::str::Lines;

type Point = (usize, usize);
type WrapPoint = (isize, isize);
type Dir = fn(&Point) -> WrapPoint;
type Blizzard = (Point, Dir);

fn up((x, y): &Point) -> WrapPoint {
    (*x as isize, *y as isize - 1)
}
fn down((x, y): &Point) -> WrapPoint {
    (*x as isize, *y as isize + 1)
}
fn left((x, y): &Point) -> WrapPoint {
    (*x as isize - 1, *y as isize)
}
fn right((x, y): &Point) -> WrapPoint {
    (*x as isize + 1, *y as isize)
}
fn id((x, y): &Point) -> WrapPoint {
    (*x as isize, *y as isize)
}

fn parse_blizzard(c: char) -> Option<Dir> {
    match c {
        '<' => Some(left),
        '>' => Some(right),
        '^' => Some(up),
        'v' => Some(down),
        _ => None,
    }
}

fn parse_input(input: Lines) -> Vec<Blizzard> {
    input
        .skip(1)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .skip(1)
                .enumerate()
                .filter_map(move |(x, c)| parse_blizzard(c).map(|b| ((x, y), b)))
        })
        .collect()
}

fn next_blizzards(blizzards: &[Blizzard], (width, height): Point) -> Vec<Blizzard> {
    blizzards
        .iter()
        .map(|(p, f)| {
            let (nx, ny) = f(p);
            (
                (
                    nx.rem_euclid(width as isize) as usize,
                    ny.rem_euclid(height as isize) as usize,
                ),
                *f,
            )
        })
        .collect()
}

fn process(
    blizzards: &mut Vec<Blizzard>,
    start: Point,
    end: Point,
    (width, height): Point,
) -> usize {
    let mut positions = vec![start];

    let dirs = [up, down, left, right, id];

    for i in 1.. {
        let mut nb = next_blizzards(blizzards, (width, height));
        blizzards.clear();
        blizzards.append(&mut nb);
        positions = positions
            .iter()
            .flat_map(|p| {
                dirs.iter()
                    .map(|f| f(p))
                    .filter_map(|(px, py)| px.try_into().ok().zip(py.try_into().ok()))
                    .filter(|&(nx, ny): &Point| {
                        (nx < width
                            && ny < height
                            && !blizzards.iter().any(|(b, _)| b == &(nx, ny)))
                            || (nx, ny) == end
                            || (nx, ny) == start
                    })
            })
            .collect();

        if positions.contains(&end) {
            return i;
        }

        positions.sort();
        positions.dedup();
    }

    panic!()
}

fn process_part1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len() - 2;
    let height = input.lines().count() - 2;

    let start = (0, 0);
    let end = (width - 1, height);

    let mut blizzards = parse_input(input.lines());

    process(&mut blizzards, start, end, (width, height))
}

fn process_part2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len() - 2;
    let height = input.lines().count() - 2;

    let start = (0, 0);
    let end = (width - 1, height);

    let mut blizzards = parse_input(input.lines());

    process(&mut blizzards, start, end, (width, height))
        + process(&mut blizzards, end, start, (width, height))
        + process(&mut blizzards, start, end, (width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, 54);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input);
    println!("{}", result_part1);

    let result_part2 = process_part2(input);
    println!("{}", result_part2);
}
