use std::{collections::HashSet, str::Lines};

fn type_priority(c: &char) -> i64 {
    match c {
        'a'..='z' => *c as i64 - 'a' as i64 + 1,
        'A'..='Z' => *c as i64 - 'A' as i64 + 27,
        _ => 0,
    }
}

fn rucksack_priority(items: Vec<char>) -> i64 {
    let mut compartments = items
        .chunks(items.len() / 2)
        .map(|c| HashSet::<_>::from_iter(c.to_vec()));
    compartments
        .next()
        .expect("Failed to get left compartment")
        .intersection(
            &compartments
                .next()
                .expect("Failed to get right compartment"),
        )
        .map(type_priority)
        .sum()
}

fn process_part1(iter: Lines) -> i64 {
    iter.map(|c| c.chars().collect::<Vec<char>>())
        .map(rucksack_priority)
        .sum()
}

fn process_part2(agg: i64, mut iter: Lines) -> i64 {
    let next_agg = match iter.next() {
        Some(first) => {
            agg + iter
                .by_ref()
                .take(2)
                .map(|a| HashSet::from_iter(a.chars()))
                .fold(first.chars().collect::<Vec<char>>(), |a, b| {
                    HashSet::<_>::from_iter(a)
                        .intersection(&b)
                        .copied()
                        .collect::<Vec<char>>()
                })
                .iter()
                .map(type_priority)
                .sum::<i64>()
        }
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
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(0, INPUT.lines());
        assert_eq!(result, 70);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(0, input.lines());
    println!("{}", result_part2);
}
