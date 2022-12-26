use std::str::Lines;

fn parse_input(input: Lines) -> impl Iterator<Item = (usize, isize)> + '_ {
    input.map(|s| s.parse::<isize>().unwrap()).enumerate()
}

fn mix(input: Vec<(usize, isize)>) -> Vec<(usize, isize)> {
    let mut ns = input;

    let len = ns.len();

    for i in 0..len {
        let index = ns
            .iter()
            .enumerate()
            .find(|(_, (id, _))| id == &i)
            .unwrap()
            .0;

        let item = ns.remove(index);
        let next_index = (index as isize + item.1)
            .rem_euclid(len as isize - 1)
            .try_into()
            .unwrap();
        ns.insert(next_index, item);
    }

    ns
}

fn score(ns: &Vec<isize>) -> isize {
    let start = ns.iter().enumerate().find(|(_, v)| **v == 0).unwrap().0;

    [1000, 2000, 3000]
        .iter()
        .map(|i| (i + start) % ns.len())
        .map(|i| ns[i])
        .sum()
}

fn process_part1(input: Lines) -> isize {
    let ns = parse_input(input).collect();
    let mixed = mix(ns);

    score(&mixed.iter().map(|(_, v)| *v).collect())
}

fn process_part2(input: Lines) -> isize {
    let ns = parse_input(input)
        .map(|(i, v)| (i, v * 811589153))
        .collect();
    let mixed = (0..10).fold(ns, |agg, _| mix(agg));

    score(&mixed.iter().map(|(_, v)| *v).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 1623178306);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
