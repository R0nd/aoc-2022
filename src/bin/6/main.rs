use std::collections::HashSet;

fn is_distinct(input: &[char]) -> bool {
    HashSet::<_>::from_iter(input).len() == input.len()
}

fn process(input: &str, size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(size)
        .position(is_distinct)
        .unwrap()
        + size
}

fn process_part1(input: &str) -> usize {
    process(input, 4)
}

fn process_part2(input: &str) -> usize {
    process(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(process_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

fn main() {
    let input = include_str!("input.txt").lines().next().unwrap();

    let result_part1 = process_part1(input);
    println!("{}", result_part1);

    let result_part2 = process_part2(input);
    println!("{}", result_part2);
}
