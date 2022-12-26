use std::{iter::from_fn, str::Lines};

fn cycles(mut input: Lines) -> impl Iterator<Item = i64> + '_ {
    let mut add_value = None;
    let mut x = 1;
    from_fn(move || {
        if let Some(value) = add_value {
            let snapshot = x;
            x += value;
            add_value = None;
            return Some(snapshot);
        }
        match input
            .next()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .as_deref()
        {
            None => None,
            Some(["addx", value]) => {
                add_value = Some(value.parse::<i64>().unwrap());
                Some(x)
            }
            Some(["noop"]) => Some(x),
            _ => panic!(),
        }
    })
}

fn process_part1(input: Lines) -> i64 {
    cycles(input)
        .zip(1..)
        .skip(20 - 1)
        .step_by(40)
        .take(6)
        .map(|(a, b)| a * b)
        .sum()
}

fn process_part2(input: Lines) -> String {
    cycles(input)
        .zip(0..)
        .map(|(x, i)| if x.abs_diff(i % 40) <= 1 { '#' } else { '.' })
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(process_part1(INPUT.lines()), 13140);
    }

    #[test]
    fn test_part2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(process_part2(INPUT.lines()), expected);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
