use std::{collections::HashSet, str::Lines};

type Vector = [i16; 2];

type Rope<const N: usize> = [Vector; N];

#[cfg(debug_assertions)]
fn format<const N: usize>(value: Rope<N>) -> String {
    let xs = value.map(|[x, _]| x);
    let x_min = xs.iter().min().unwrap();
    let x_max = xs.iter().max().unwrap();
    let width: usize = (x_max - x_min + 1).try_into().unwrap();

    let ys = value.map(|[_, y]| y);
    let y_min = ys.iter().min().unwrap();
    let y_max = ys.iter().max().unwrap();
    let height: usize = (y_max - y_min + 1).try_into().unwrap();

    let mut grid = vec![vec!['.'; width]; height];

    for (index, [x, y]) in value.iter().enumerate() {
        let nx: usize = (x - x_min).try_into().unwrap();
        let ny: usize = (y - y_min).try_into().unwrap();
        grid[ny][nx] = match grid[ny][nx] {
            '.' => char::from_digit(index.try_into().unwrap(), 10).unwrap(),
            c => c,
        };
    }

    grid.iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn basis_vector(value: &str) -> Vector {
    match value {
        "U" => [0, -1],
        "D" => [0, 1],
        "L" => [-1, 0],
        "R" => [1, 0],
        _ => panic!(),
    }
}

fn move_vector(value: &str) -> (Vector, u8) {
    match value.split_whitespace().collect::<Vec<&str>>()[..] {
        [dir, dist] => (basis_vector(dir), dist.parse::<u8>().unwrap()),
        _ => panic!(),
    }
}

fn zip_vector([hx, hy]: Vector, [tx, ty]: Vector) -> [(i16, i16); 2] {
    [(hx, tx), (hy, ty)]
}

fn catch_up(prev: &Vector, cur: &Vector) -> Vector {
    let pairs = zip_vector(*prev, *cur);
    if pairs.iter().all(|(h, t)| h.abs_diff(*t) <= 1) {
        *cur
    } else {
        pairs.map(|(h, t)| t + (h - t).signum())
    }
}

fn simulate_move<const N: usize>(
    rope: Rope<N>,
    move_vec: (Vector, u8),
    tail_positions: &mut HashSet<Vector>,
) -> [Vector; N] {
    match move_vec {
        (_, 0) => rope,
        (dir, dist) => {
            let next_head = zip_vector(rope[0], dir).map(|(a, b)| a + b);
            let next_rope: Rope<N> = rope
                .iter()
                .skip(1)
                .fold(vec![next_head], |mut next_rope, v| {
                    next_rope.push(catch_up(next_rope.last().unwrap(), v));
                    next_rope
                })
                .try_into()
                .unwrap();

            tail_positions.extend(next_rope.last());
            simulate_move(next_rope, (dir, dist - 1), tail_positions)
        }
    }
}

fn process_internal<const N: usize>(
    rope: Rope<N>,
    agg: &mut HashSet<Vector>,
    mut input: Lines,
) -> usize {
    #[cfg(debug_assertions)]
    println!("{}\n", format(rope));

    let next_rope = match input.next().map(move_vector) {
        Some(move_vec) => simulate_move(rope, move_vec, agg),
        _ => return agg.len(),
    };

    process_internal(next_rope, agg, input)
}

fn process<const N: usize>(input: Lines) -> usize {
    let zero = [0; 2];
    let mut agg = HashSet::<_>::from([zero]);
    process_internal([zero; N], &mut agg, input)
}

fn process_part1(input: Lines) -> usize {
    process::<2>(input)
}

fn process_part2(input: Lines) -> usize {
    process::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(process_part1(INPUT.lines()), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process_part2(INPUT.lines()), 1);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
