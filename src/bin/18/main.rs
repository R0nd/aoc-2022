use std::{collections::HashMap, str::Lines};

type Cube = (isize, isize, isize);

#[derive(PartialEq)]
enum Cell {
    Lava,
    Water,
}

fn parse_input(input: Lines) -> impl Iterator<Item = Cube> + '_ {
    input.map(
        |line| match line.split(',').flat_map(|v| v.parse()).collect::<Vec<_>>()[..] {
            [x, y, z] => (x, y, z),
            _ => panic!(),
        },
    )
}

fn neighbors((x, y, z): &Cube) -> [Cube; 6] {
    [
        (x - 1, *y, *z),
        (x + 1, *y, *z),
        (*x, y - 1, *z),
        (*x, y + 1, *z),
        (*x, *y, z - 1),
        (*x, *y, z + 1),
    ]
}

fn process_part1(input: Lines) -> usize {
    let cubes = parse_input(input)
        .map(|cube| (cube, Some(())))
        .collect::<HashMap<Cube, Option<()>>>();

    cubes
        .iter()
        .map(|(cube, _)| 6 - neighbors(cube).iter().filter_map(|c| cubes.get(c)).count())
        .sum()
}

fn flood_fill(cubes: &mut HashMap<Cube, Cell>, point: &Cube, min: &Cube, max: &Cube) {
    if point.0 < min.0 || point.1 < min.1 || point.2 < min.2 {
        return;
    }
    if point.0 > max.0 || point.1 > max.1 || point.2 > max.2 {
        return;
    }
    if !cubes.contains_key(point) {
        cubes.insert(*point, Cell::Water);
        neighbors(point)
            .iter()
            .for_each(|p| flood_fill(cubes, p, min, max))
    }
}

fn process_part2(input: Lines) -> usize {
    let source = parse_input(input).collect::<Vec<_>>();

    let mut cubes = source
        .iter()
        .map(|cube| (*cube, Cell::Lava))
        .collect::<HashMap<Cube, Cell>>();

    let min_x = source.iter().map(|&(x, _, _)| x).min().unwrap();
    let max_x = source.iter().map(|&(x, _, _)| x).max().unwrap();
    let min_y = source.iter().map(|&(_, y, _)| y).min().unwrap();
    let max_y = source.iter().map(|&(_, y, _)| y).max().unwrap();
    let min_z = source.iter().map(|&(_, _, z)| z).min().unwrap();
    let max_z = source.iter().map(|&(_, _, z)| z).max().unwrap();

    let min = &(min_x - 1, min_y - 1, min_z - 1);
    let max = &(max_x + 1, max_y + 1, max_z + 1);

    flood_fill(&mut cubes, &(min_x - 1, min_y, min_z), min, max);
    flood_fill(&mut cubes, &(max_x + 1, min_y, min_z), min, max);
    flood_fill(&mut cubes, &(min_x, min_y - 1, min_z), min, max);
    flood_fill(&mut cubes, &(min_x, max_y + 1, min_z), min, max);
    flood_fill(&mut cubes, &(min_x, min_y, min_z - 1), min, max);
    flood_fill(&mut cubes, &(min_x, min_y, max_z + 1), min, max);

    cubes
        .iter()
        .filter(|(_, v)| **v == Cell::Lava)
        .map(|(p, _)| {
            neighbors(p)
                .iter()
                .filter(|c| cubes.get(c) == Some(&Cell::Water))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 64);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 58);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
