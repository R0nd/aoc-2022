use std::str::Lines;

fn parse_input(input: Lines) -> Vec<Vec<char>> {
    input.map(|line| line.chars().collect()).collect()
}

fn rays(field: &[Vec<char>], i: usize, j: usize) -> Vec<Vec<char>> {
    let mut result = vec![];

    result.push(field[i][..j].to_vec());
    result.push({
        let mut ray = field[i][j + 1..].to_vec();
        ray.reverse();
        ray
    });

    let column: Vec<char> = field.iter().map(|line| line[j]).collect();
    result.push(column[..i].to_vec());
    result.push({
        let mut ray = column[i + 1..].to_vec();
        ray.reverse();
        ray
    });

    result
}

fn process_part1(input: Lines) -> usize {
    let field = parse_input(input);

    (0..field.len())
        .map(|i| {
            let field = &field;
            (0..field[i].len())
                .filter(move |&j| {
                    rays(field, i, j)
                        .iter()
                        .any(|ray| ray.iter().all(|other| *other < field[i][j]))
                })
                .count()
        })
        .sum()
}

fn score(field: &[Vec<char>], i: usize, j: usize) -> usize {
    let this = field[i][j];
    rays(field, i, j)
        .iter()
        .map(|ray| {
            let count = ray.iter().rev().take_while(|other| **other < this).count();
            if count == ray.len() {
                count
            } else {
                count + 1
            }
        })
        .product()
}

fn process_part2(input: Lines) -> usize {
    let field = parse_input(input);

    (0..field.len())
        .flat_map(|i| {
            let field = &field;
            (0..field[i].len()).map(move |j| score(field, i, j))
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(process_part1(INPUT.lines()), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process_part2(INPUT.lines()), 8);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
