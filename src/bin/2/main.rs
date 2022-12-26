use std::str::Lines;

fn shape_score(shape: char) -> i64 {
    match shape {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}

fn outcome_score(a: char, b: char) -> i64 {
    match (a, b) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        _ => 0,
    }
}

fn round_score(a: char, b: char) -> i64 {
    shape_score(b) + outcome_score(a, b)
}

fn round_score2(a: char, b: char) -> i64 {
    let mapped_b = match (a, b) {
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 'X',
        ('B', 'Y') | ('C', 'X') | ('A', 'Z') => 'Y',
        ('C', 'Y') | ('A', 'X') | ('B', 'Z') => 'Z',
        _ => panic!(),
    };

    let outcome_score = match b {
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };

    shape_score(mapped_b) + outcome_score
}

fn parse_round(input: &str) -> Result<(char, char), ()> {
    if let [[a], [b]] = input.chars().collect::<Vec<char>>()[..]
        .split(|c| *c == ' ')
        .collect::<Vec<&[char]>>()[..]
    {
        Ok((*a, *b))
    } else {
        Err(())
    }
}

fn process_part1(agg: i64, mut iter: Lines) -> i64 {
    let next_agg = match iter.next().map(parse_round) {
        Some(Ok((a, b))) => agg + round_score(a, b),
        _ => return agg,
    };

    process_part1(next_agg, iter)
}

fn process_part2(agg: i64, mut iter: Lines) -> i64 {
    let next_agg = match iter.next().map(parse_round) {
        Some(Ok((a, b))) => agg + round_score2(a, b),
        _ => return agg,
    };

    process_part2(next_agg, iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(0, INPUT.lines());
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(0, INPUT.lines());
        assert_eq!(result, 12);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(0, input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(0, input.lines());
    println!("{}", result_part2);
}
