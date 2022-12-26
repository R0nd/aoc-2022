use std::{
    cmp::{max, min},
    str::Lines,
};

type Point = (isize, isize);

fn parse_input(input: Lines) -> Vec<Point> {
    input
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn compl(p: Point) -> Point {
    match p {
        (_, 0) => (0, 1),
        (0, _) => (1, 0),
        _ => panic!(),
    }
}

fn mul((x, y): &Point, coef: isize) -> Point {
    (x * coef, y * coef)
}

fn add((ax, ay): &Point, (bx, by): &Point) -> Point {
    (ax + bx, ay + by)
}

fn is_neighbor((ax, ay): &Point, (bx, by): &Point) -> bool {
    (ax, ay) != (bx, by) && ax.abs_diff(*bx) <= 1 && ay.abs_diff(*by) <= 1
}

fn turn<T: Iterator<Item = Point>>(elves: &[Point], vectors: &mut T) -> Vec<Point> {
    let proposal_templates = vectors
        .take(4)
        .map(|v| {
            (
                v,
                (-1..=1)
                    .map(move |coef| add(&v, &mul(&compl(v), coef)))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    vectors.next();

    let proposals = elves
        .iter()
        .map(|e| {
            if !elves.iter().any(|o| is_neighbor(o, e)) {
                *e
            } else {
                proposal_templates
                    .iter()
                    .find(|(_, pv)| pv.iter().all(|p| !elves.contains(&add(e, p))))
                    .map_or(*e, |(p, _)| add(e, p))
            }
        })
        .collect::<Vec<_>>();

    let result = elves
        .iter()
        .zip(proposals.iter())
        .map(|(e, p)| {
            if proposals.iter().filter(|pp| pp == &p).count() > 1 {
                e
            } else {
                p
            }
        })
        .copied()
        .collect::<Vec<_>>();

    result
}

fn process_part1(input: Lines) -> usize {
    let init_elves = parse_input(input);

    let mut vectors = [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().copied().cycle();

    let elves = (0..10).fold(init_elves, |elves, _| turn(&elves, &mut vectors));

    let (left, top) = elves
        .iter()
        .cloned()
        .reduce(|(minx, miny), (x, y)| (min(minx, x), min(miny, y)))
        .unwrap();
    let (right, bottom) = elves
        .iter()
        .cloned()
        .reduce(|(maxx, maxy), (x, y)| (max(maxx, x), max(maxy, y)))
        .unwrap();

    (right.abs_diff(left) + 1) * (bottom.abs_diff(top) + 1) - elves.len()
}

fn process_part2(input: Lines) -> usize {
    let mut elves = parse_input(input);

    let mut vectors = [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().copied().cycle();

    (1..)
        .take_while(|_| {
            let next_elves = turn(&elves, &mut vectors);
            let done = elves == next_elves;
            elves = next_elves;
            !done
        })
        .last()
        .unwrap()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 110);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 20);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
