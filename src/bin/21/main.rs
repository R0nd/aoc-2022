use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
    str::Lines,
};

type MonkeyKey = [char; 4];

#[derive(Clone, Copy)]
enum Monkey {
    Const(usize),
    Op(MonkeyKey, MonkeyKey, Op),
    Var,
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn op(a: usize, b: usize, f: &Op) -> usize {
    (match f {
        Op::Add => Add::add,
        Op::Sub => Sub::sub,
        Op::Mul => Mul::mul,
        Op::Div => Div::div,
    })(a, b)
}

fn parse_op(input: &str) -> Op {
    match input {
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        _ => panic!(),
    }
}

fn parse_key(input: &str) -> MonkeyKey {
    input.chars().collect::<Vec<_>>().try_into().unwrap()
}

fn parse_value(input: &str) -> Monkey {
    match input.split_whitespace().collect::<Vec<_>>()[..] {
        [v] => Monkey::Const(v.parse().unwrap()),
        [a, op, b] => Monkey::Op(parse_key(a), parse_key(b), parse_op(op)),
        _ => panic!(),
    }
}

fn parse_line(input: &str) -> (MonkeyKey, Monkey) {
    match input.split(':').collect::<Vec<_>>()[..] {
        [key, value] => (parse_key(key), parse_value(value.trim_start())),
        _ => panic!(),
    }
}

fn parse_input(input: Lines) -> HashMap<MonkeyKey, Monkey> {
    input.map(parse_line).collect()
}

fn yell(key: &MonkeyKey, monkeys: &HashMap<MonkeyKey, Monkey>) -> usize {
    let monkey = monkeys.get(key).unwrap();

    match monkey {
        Monkey::Const(v) => *v,
        Monkey::Op(a, b, f) => op(yell(a, monkeys), yell(b, monkeys), f),
        _ => panic!(),
    }
}

fn contains_var(key: &MonkeyKey, monkeys: &HashMap<MonkeyKey, Monkey>) -> bool {
    match monkeys.get(key).unwrap() {
        Monkey::Const(_) => false,
        Monkey::Var => true,
        Monkey::Op(a, b, _) => contains_var(a, monkeys) || contains_var(b, monkeys),
    }
}

fn flip_op(source: &Op) -> Op {
    match source {
        Op::Add => Op::Sub,
        Op::Sub => Op::Add,
        Op::Mul => Op::Div,
        Op::Div => Op::Mul,
    }
}

fn flip(
    source: &Monkey,
    new_subkey: &MonkeyKey,
    monkeys: &HashMap<MonkeyKey, Monkey>,
) -> (Monkey, MonkeyKey) {
    match source {
        Monkey::Op(a, b, f) if contains_var(a, monkeys) => {
            (Monkey::Op(*new_subkey, *b, flip_op(f)), *a)
        }
        Monkey::Op(a, b, f) if contains_var(b, monkeys) => {
            if matches!(f, Op::Div | Op::Sub) {
                (Monkey::Op(*b, *new_subkey, flip_op(f)), *a)
            } else {
                (Monkey::Op(*new_subkey, *a, flip_op(f)), *b)
            }
        }
        _ => panic!(),
    }
}

fn balance(left: &MonkeyKey, right: &MonkeyKey, monkeys: &HashMap<MonkeyKey, Monkey>) -> usize {
    match [left, right].map(|k| monkeys.get(k).unwrap()) {
        [Monkey::Var, _] => yell(right, monkeys),
        [_, Monkey::Var] => yell(left, monkeys),
        [l, _] if contains_var(left, monkeys) => {
            let (flipped, out) = flip(l, right, monkeys);
            balance(
                &out,
                left,
                &monkeys
                    .iter()
                    .map(|(k, v)| if k == left { (*k, flipped) } else { (*k, *v) })
                    .collect(),
            )
        }
        [_, r] if contains_var(right, monkeys) => {
            let (flipped, out) = flip(r, left, monkeys);
            balance(
                right,
                &out,
                &monkeys
                    .iter()
                    .map(|(k, v)| if k == right { (*k, flipped) } else { (*k, *v) })
                    .collect(),
            )
        }
        _ => panic!(),
    }
}

fn process_part1(input: Lines) -> usize {
    let monkeys = parse_input(input);

    yell(&['r', 'o', 'o', 't'], &monkeys)
}

fn process_part2(input: Lines) -> usize {
    let mut monkeys = parse_input(input);

    let humn = ['h', 'u', 'm', 'n'];
    monkeys.remove(&humn);
    monkeys.insert(humn, Monkey::Var);

    if let Monkey::Op(left, right, _) = monkeys.get(&['r', 'o', 'o', 't']).unwrap() {
        balance(left, right, &monkeys)
    } else {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 152);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 301);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
