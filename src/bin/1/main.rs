use std::str::Lines;

fn max<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

fn process_part1(agg: usize, cur: usize, mut iter: Lines) -> usize {
    let next_line = match iter.next() {
        Some(line) => line,
        _ => return max(agg, cur),
    };

    let (next_max, next_cur) = match next_line.parse::<usize>() {
        Ok(number) => (agg, cur + number),
        _ => (max(agg, cur), 0),
    };

    process_part1(next_max, next_cur, iter)
}

fn process_part2(mut agg: Vec<usize>, cur: usize, mut iter: Lines) -> usize {
    let (next_max, next_cur) = match iter.next().map(|line| line.parse::<usize>()) {
        Some(Ok(number)) => (agg, cur + number),
        break_or_none => {
            agg.push(cur);
            agg.sort();
            agg.reverse();
            agg.pop();

            if break_or_none.is_none() {
                return agg.iter().sum();
            }

            (agg, 0)
        }
    };

    process_part2(next_max, next_cur, iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(0, 0, INPUT.lines());
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(vec![0; 3], 0, INPUT.lines());
        assert_eq!(result, 45000);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(0, 0, input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(vec![0; 3], 0, input.lines());
    println!("{}", result_part2);
}
