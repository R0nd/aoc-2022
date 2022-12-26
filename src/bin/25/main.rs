use std::str::Lines;

fn snafu_digit(snafu: char) -> isize {
    match snafu {
        '-' => -1,
        '=' => -2,
        _ => snafu.to_digit(3).and_then(|d| d.try_into().ok()).unwrap(),
    }
}

fn snafu_to_int(snafu: &str) -> isize {
    snafu.chars().rev().enumerate().fold(0, |agg, (i, d)| {
        agg + 5_isize.pow(i.try_into().unwrap()) * snafu_digit(d)
    })
}

fn int_to_base5(int: isize) -> String {
    if int == 0 {
        "".to_owned()
    } else {
        int_to_base5(int / 5) + &(int % 5).to_string()
    }
}

fn digit_inc(digit: char) -> (char, bool) {
    match digit {
        '0' => ('1', false),
        '1' => ('2', false),
        '2' => ('3', false),
        '3' => ('4', false),
        '4' => ('0', true),
        _ => panic!(),
    }
}

fn int_to_snafu(int: isize) -> String {
    let base5 = int_to_base5(int);
    base5
        .chars()
        .rev()
        .fold((false, "".to_owned()), |(carry, agg), c| {
            let (nc, ncarry) = if carry { digit_inc(c) } else { (c, false) };
            match nc {
                '0' | '1' | '2' => (ncarry, agg + &nc.to_string()),
                '3' => (true, agg + "="),
                '4' => (true, agg + "-"),
                _ => panic!(),
            }
        })
        .1
        .chars()
        .rev()
        .collect()
}

fn process_part1(input: Lines) -> String {
    int_to_snafu(input.map(snafu_to_int).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, "2=-1=0");
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);
}
