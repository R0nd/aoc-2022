use std::str::Lines;

type Instruction = (usize, usize, usize);

fn parse_input(mut iter: Lines) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let field_lines = iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let mut field_lines_iter = field_lines.iter().rev();

    let stack_count = (field_lines_iter.next().expect("").len() + 1) / 4;
    let mut field = vec![vec![]; stack_count];
    field_lines_iter.for_each(|line| {
        line.char_indices()
            .filter(|(_, c)| c.is_alphabetic())
            .for_each(|(idx, value)| field[(idx - 1) / 4].push(value))
    });

    let instructions = iter
        .map(|line| {
            if let [x, y, z] = line
                .chars()
                .filter(|c| c.is_numeric() || c.is_whitespace())
                .collect::<String>()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()[..]
            {
                (x, y, z)
            } else {
                panic!();
            }
        })
        .collect();

    (field, instructions)
}

fn move_sequential(mut field: Vec<Vec<char>>, params: Instruction) -> Vec<Vec<char>> {
    let (count, from, to) = params;
    for _ in 0..count {
        field[from - 1]
            .pop()
            .iter()
            .copied()
            .for_each(|c| field[to - 1].push(c));
    }
    field
}

fn move_batch(mut field: Vec<Vec<char>>, params: (usize, usize, usize)) -> Vec<Vec<char>> {
    let (count, from, to) = params;
    let split_index = field[from - 1].len() - count;
    let mut moved = field[from - 1].split_off(split_index);
    field[to - 1].append(&mut moved);
    field
}

fn process<F: Fn(Vec<Vec<char>>, (usize, usize, usize)) -> Vec<Vec<char>>>(
    iter: Lines,
    move_fn: F,
) -> String {
    let (mut field, instructions) = parse_input(iter);

    for params in instructions {
        field = move_fn(field, params);
    }

    field
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn process_part1(iter: Lines) -> String {
    process(iter, move_sequential)
}

fn process_part2(iter: Lines) -> String {
    process(iter, move_batch)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, "MCD");
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
