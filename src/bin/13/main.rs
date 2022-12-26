use std::{
    cmp::Ordering,
    iter::from_fn,
    str::{Chars, Lines},
};

#[derive(Clone, Eq)]
enum Value {
    Single(usize),
    List(Vec<Value>),
}

impl Value {
    fn split(&self) -> (&Self, Self) {
        match self {
            Value::Single(_) => panic!(),
            Value::List(l) => {
                let (head, tail) = l.split_first().unwrap();
                (head, Value::List(tail.to_vec()))
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(l0), Self::Single(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Single(a), Value::Single(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) if a.is_empty() && b.is_empty() => Ordering::Equal,
            (Value::List(a), _) if a.is_empty() => Ordering::Less,
            (_, Value::List(b)) if b.is_empty() => Ordering::Greater,
            (a @ Value::List(_), b @ Value::List(_)) => {
                let (a_head, a_tail) = a.split();
                let (b_head, b_tail) = b.split();
                match a_head.cmp(b_head) {
                    Ordering::Equal => a_tail.cmp(&b_tail),
                    o => o,
                }
            }
            (a @ Value::Single(_), b @ Value::List(_)) => {
                let (b_head, b_tail) = b.split();
                match a.cmp(b_head) {
                    Ordering::Equal => Value::List(vec![]).cmp(&b_tail),
                    o => o,
                }
            }
            (a @ Value::List(_), b @ Value::Single(_)) => {
                let (a_head, a_tail) = a.split();
                match a_head.cmp(b) {
                    Ordering::Equal => a_tail.cmp(&Value::List(vec![])),
                    o => o,
                }
            }
        }
    }
}

fn parse_list(input: &mut Chars) -> Value {
    Value::List(from_fn(|| parse_value(input)).collect())
}

fn parse_value(input: &mut Chars) -> Option<Value> {
    let mut digit_buf = vec![];

    for c in input.by_ref() {
        match c {
            '[' => return Some(parse_list(input)),
            ']' | ',' => break,
            c if c.is_numeric() => {
                digit_buf.push(c);
            }
            _ => panic!(),
        };
    }

    if !digit_buf.is_empty() {
        Some(Value::Single(
            digit_buf.iter().collect::<String>().parse().unwrap(),
        ))
    } else {
        None
    }
}

fn process_part1(input: Lines) -> usize {
    let mut pairs = vec![];

    let mut pair = vec![];
    for line in input {
        match line {
            "" => {
                pairs.push(pair);
                pair = vec![];
            }
            _ => {
                pair.push(parse_value(&mut line.chars()).unwrap());
            }
        }
    }

    pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair[0] <= pair[1])
        .map(|(i, _)| i + 1)
        .sum()
}

fn divider(value: usize) -> Value {
    Value::List(vec![Value::List(vec![Value::Single(value)])])
}

fn process_part2(input: Lines) -> usize {
    let mut values = input
        .filter(|line| !line.is_empty())
        .map(|line| parse_value(&mut line.chars()).unwrap())
        .collect::<Vec<Value>>();

    values.push(divider(2));
    values.push(divider(6));

    values.sort();

    values
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == divider(2) || **v == divider(6))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 140);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
