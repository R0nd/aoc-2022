use std::{
    cmp::max,
    collections::HashMap,
    ops::{Add, Sub},
    str::Lines,
};

type Resources = (usize, usize, usize);

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Bot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Bots = [(Bot, usize); 4];
type Blueprint = [(Option<Bot>, Resources); 5];

fn zip<T>(
    &(aa, ab, ac): &Resources,
    &(ba, bb, bc): &Resources,
    f: fn(usize, usize) -> T,
) -> (T, T, T) {
    (f(aa, ba), f(ab, bb), f(ac, bc))
}

fn le((aa, ab, ac): &Resources, (ba, bb, bc): &Resources) -> bool {
    aa <= ba && ab <= bb && ac <= bc
}

fn sub(a: &Resources, b: &Resources) -> Resources {
    zip(a, b, Sub::sub)
}

fn add(a: &Resources, b: &Resources) -> Resources {
    zip(a, b, Add::add)
}

fn to_tuple<T: Copy>(&source: &[T; 3]) -> (T, T, T) {
    (source[0], source[1], source[2])
}

fn triangle(value: usize) -> usize {
    if value == 0 {
        return 0;
    }
    value + triangle(value - 1)
}

fn process_internal(
    blueprint: &Blueprint,
    bots: &Bots,
    resources: &Resources,
    time: usize,
    memo: &mut HashMap<(Bots, Resources, usize), usize>,
    max_geodes: usize,
    magic: usize,
) -> usize {
    if bots[3].1 * time + triangle(time) <= max_geodes {
        return 0;
    }
    if time == 0 {
        0
    } else if time == 1 {
        bots[3].1
    } else if let Some(result) = memo.get(&(*bots, *resources, time)) {
        *result
    } else {
        let next_resources = add(
            resources,
            &to_tuple(&bots.map(|(_, n)| n)[..3].try_into().unwrap()),
        );

        let mut moves = blueprint
            .iter()
            .rev()
            .filter(|(_, cost)| le(cost, resources))
            .collect::<Vec<_>>();

        if let Some(geode_move) = moves.iter().find(|(b, _)| b == &Some(Bot::Geode)) {
            moves = vec![geode_move];
        } else if moves
            .iter()
            .filter_map(|(b, _)| *b)
            .filter(|b| [Bot::Ore, Bot::Clay].contains(b))
            .count()
            == 2
        {
            moves.retain(|(b, _)| b.is_some());
        };

        let result = bots[3].1
            + moves.iter().fold(0, |agg, (bot, cost)| {
                let next_bots =
                    bots.map(|(b, n)| if bot == &Some(b) { (b, n + 1) } else { (b, n) });
                max(
                    agg,
                    process_internal(
                        blueprint,
                        &next_bots,
                        &sub(&next_resources, cost),
                        time - 1,
                        memo,
                        max_geodes,
                        magic,
                    ),
                )
            });

        if time >= magic {
            memo.insert((*bots, *resources, time), result);
        }

        result
    }
}

fn process(blueprint: &Blueprint, time: usize, magic: usize) -> usize {
    process_internal(
        blueprint,
        &[
            (Bot::Ore, 1),
            (Bot::Clay, 0),
            (Bot::Obsidian, 0),
            (Bot::Geode, 0),
        ],
        &(0, 0, 0),
        time,
        &mut HashMap::<_, _>::new(),
        0,
        magic,
    )
}

fn parse_line(input: &str) -> (usize, Blueprint) {
    let values = input
        .split([' ', ':'])
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_numeric()))
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    match values[..] {
        [i, oo, co, obo, obc, go, gob] => (
            i,
            [
                (None, (0, 0, 0)),
                (Some(Bot::Ore), (oo, 0, 0)),
                (Some(Bot::Clay), (co, 0, 0)),
                (Some(Bot::Obsidian), (obo, obc, 0)),
                (Some(Bot::Geode), (go, 0, gob)),
            ],
        ),
        _ => panic!(),
    }
}

fn parse_input(input: Lines) -> impl Iterator<Item = (usize, Blueprint)> + '_ {
    input.map(parse_line)
}

fn process_part1(input: Lines) -> usize {
    parse_input(input)
        .map(|(i, b)| i * process(&b, 24, 3))
        .sum()
}

fn process_part2(input: Lines) -> usize {
    parse_input(input)
        .take(3)
        .map(|(_, b)| process(&b, 32, 5))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 33);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
