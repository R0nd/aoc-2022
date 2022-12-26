use std::str::Lines;

fn parse_range(value: &str) -> Result<(u8, u8), ()> {
    match value.split('-').collect::<Vec<&str>>()[..] {
        [a, b] => match (a.parse::<u8>(), b.parse::<u8>()) {
            (Ok(a_val), Ok(b_val)) => Ok((a_val, b_val)),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn parse_line(line: &str) -> ((u8, u8), (u8, u8)) {
    match line.split(',').collect::<Vec<&str>>()[..] {
        [a, b] => (parse_range(a).unwrap(), parse_range(b).unwrap()),
        _ => panic!(),
    }
}

fn contains<T: PartialOrd>(a: &(T, T), b: &(T, T)) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn overlap<T: PartialOrd>(a: &(T, T), b: &(T, T)) -> bool {
    a.0 <= b.1 && a.1 >= b.0
}

fn either<T: PartialOrd, F: Fn(&(T, T), &(T, T)) -> bool>(f: F, a: &(T, T), b: &(T, T)) -> bool {
    f(a, b) || f(b, a)
}

fn process_part1(iter: Lines) -> usize {
    iter.map(parse_line)
        .filter(|(a, b)| either(contains, a, b))
        .count()
}

fn process_part2(iter: Lines) -> usize {
    iter.map(parse_line)
        .filter(|(a, b)| either(overlap, a, b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 4);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
