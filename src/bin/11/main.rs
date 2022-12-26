use std::str::Lines;

const PRIME_PRODUCT: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

struct Monkey {
    items: Vec<usize>,
    operator: fn(usize, usize) -> usize,
    operand: String,
    divisor: usize,
    target_true: usize,
    target_false: usize,
    inspected: usize,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: vec![],
            operator: |_, _| panic!(),
            operand: "".to_string(),
            divisor: 0,
            target_true: usize::MAX,
            target_false: usize::MAX,
            inspected: 0,
        }
    }

    fn inspect_items(&mut self, relief: usize) -> Vec<(usize, usize)> {
        self.items
            .iter()
            .map(|item| {
                let operand = match self.operand.as_str() {
                    "old" => *item,
                    n => n.parse().unwrap(),
                };
                let next_item = ((self.operator)(*item, operand) / relief) % PRIME_PRODUCT;
                let target = if next_item % self.divisor == 0 {
                    self.target_true
                } else {
                    self.target_false
                };
                self.inspected += 1;
                (next_item, target)
            })
            .collect()
    }
}

fn get_operator(input: &str) -> fn(usize, usize) -> usize {
    match input {
        "+" => core::ops::Add::add,
        "*" => core::ops::Mul::mul,
        _ => panic!(),
    }
}

fn parse_op(input: &str) -> (fn(usize, usize) -> usize, String) {
    match input.split_whitespace().collect::<Vec<&str>>()[..] {
        ["new", "=", "old", op, b] => (get_operator(op), b.to_string()),
        _ => panic!(),
    }
}

fn parse_monkey(mut monkey: Monkey, input: &mut Lines) -> Option<Monkey> {
    match input
        .next()
        .map(|line| line.split(':').map(|s| s.trim()).collect::<Vec<&str>>())
        .as_deref()
    {
        Some([line, _]) if line.starts_with("Monkey ") => (),
        Some(["Starting items", items]) => items
            .split(',')
            .map(|n| n.trim().parse::<usize>().unwrap())
            .for_each(|n| monkey.items.push(n)),
        Some(["Operation", op]) => {
            (monkey.operator, monkey.operand) = parse_op(op);
        }
        Some(["Test", line]) => {
            monkey.divisor = line.trim_start_matches("divisible by ").parse().unwrap()
        }
        Some(["If true", line]) => {
            monkey.target_true = line.trim_start_matches("throw to monkey ").parse().unwrap()
        }
        Some(["If false", line]) => {
            monkey.target_false = line.trim_start_matches("throw to monkey ").parse().unwrap()
        }
        Some([""]) => return Some(monkey),
        None => return None,
        _ => panic!(),
    };

    parse_monkey(monkey, input)
}

fn parse_input(mut monkeys: Vec<Monkey>, mut input: Lines) -> Vec<Monkey> {
    match parse_monkey(Monkey::new(), &mut input) {
        Some(monkey) => monkeys.push(monkey),
        _ => return monkeys,
    }

    parse_input(monkeys, input)
}

fn round(relief: usize, monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let throws = monkeys[i].inspect_items(relief);
        monkeys[i].items.clear();
        for (item, target) in throws {
            monkeys[target].items.push(item);
        }
    }
}

fn process(relief: usize, rounds: usize, input: Lines) -> usize {
    let mut monkeys = parse_input(vec![], input);

    for _ in 0..rounds {
        round(relief, &mut monkeys);
    }

    let mut inspecteds = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspecteds.sort();
    inspecteds.reverse();
    inspecteds.iter().take(2).product()
}

fn process_part1(input: Lines) -> usize {
    process(3, 20, input)
}

fn process_part2(input: Lines) -> usize {
    process(1, 10000, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 2713310158);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
