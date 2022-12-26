use std::{collections::HashMap, iter::once, str::Lines};

type ValveKey = [char; 2];

const START: ValveKey = ['A', 'A'];

#[derive(Clone)]
struct Valve {
    key: ValveKey,
    flow: usize,
    tunnels: Vec<ValveKey>,
}

impl Valve {
    fn new(key: ValveKey, flow: usize, tunnels: Vec<ValveKey>) -> Self {
        Self { key, flow, tunnels }
    }
}

fn parse_flow(input: &str) -> usize {
    match input
        .trim_end_matches(';')
        .split('=')
        .collect::<Vec<&str>>()[..]
    {
        ["rate", flow] => flow.parse().unwrap(),
        _ => panic!(),
    }
}

fn parse_key(input: &str) -> ValveKey {
    match input.trim().chars().collect::<Vec<char>>()[..] {
        [a, b] => [a, b],
        _ => panic!(),
    }
}

fn parse_line(input: &str) -> Valve {
    match input.split([' ', ',']).collect::<Vec<&str>>()[..] {
        ["Valve", key, "has", "flow", flow_str, ..] => Valve::new(
            parse_key(key),
            parse_flow(flow_str),
            input
                .chars()
                .rev()
                .take_while(|c| c.is_uppercase() || c.is_whitespace() || *c == ',')
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>()
                .split(',')
                .map(parse_key)
                .collect(),
        ),
        _ => panic!(),
    }
}

fn parse_input(input: Lines) -> impl Iterator<Item = Valve> + '_ {
    input.map(parse_line)
}

fn distance(
    from_key: &ValveKey,
    to_key: &ValveKey,
    visited: &[ValveKey],
    valves: &HashMap<ValveKey, Valve>,
) -> Option<usize> {
    let from = valves.get(from_key).unwrap();
    if from.tunnels.contains(to_key) {
        Some(1)
    } else {
        let next_visited = visited
            .iter()
            .chain(once(from_key))
            .copied()
            .collect::<Vec<ValveKey>>();
        from.tunnels
            .iter()
            .filter(|k| !visited.contains(k))
            .filter_map(|k| distance(k, to_key, &next_visited, valves))
            .min()
            .map(|d| d + 1)
    }
}

fn build_distances(valves: &HashMap<ValveKey, Valve>) -> HashMap<(ValveKey, ValveKey), usize> {
    let keys = valves
        .iter()
        .filter(|(k, v)| v.flow > 0 || **k == START)
        .map(|(k, _)| k)
        .cloned()
        .collect::<Vec<ValveKey>>();

    keys.iter()
        .flat_map(|a| keys.iter().copied().map(|b| (*a, b)))
        .filter(|(a, b)| a < b)
        .flat_map(|(a, b)| {
            let d = distance(&a, &b, &[], valves).unwrap();
            if a == START {
                vec![((a, b), d)]
            } else {
                vec![((a, b), d), ((b, a), d)]
            }
        })
        .collect()
}

fn walks(
    countdown: usize,
    last: ValveKey,
    distances: &HashMap<(ValveKey, ValveKey), usize>,
    include_partial: bool,
) -> Vec<Vec<ValveKey>> {
    if countdown <= 2 || distances.is_empty() {
        vec![vec![last]]
    } else {
        distances
            .iter()
            .filter(|((k, _), d)| k == &last && **d < countdown - 1)
            .flat_map(|((_, next), d)| {
                let next_countdown = countdown - *d - 1;
                let mut result = walks(
                    next_countdown,
                    *next,
                    &distances
                        .iter()
                        .filter(|((_, k), _)| k != next)
                        .map(|(k, v)| (*k, *v))
                        .collect(),
                    include_partial,
                )
                .iter()
                .map(|visits| once(&last).chain(visits.iter()).cloned().collect())
                .collect::<Vec<_>>();

                if include_partial {
                    result.push(vec![last]);
                };

                result
            })
            .collect()
    }
}

fn score(
    countdown: usize,
    walk: &[ValveKey],
    distances: &HashMap<(ValveKey, ValveKey), usize>,
    valves: &HashMap<ValveKey, Valve>,
) -> usize {
    if walk.len() <= 1 {
        0
    } else {
        let (head, rest) = walk.split_first().unwrap();
        let next = rest.first().unwrap();
        let next_countdown = countdown - distances.get(&(*head, *next)).unwrap() - 1;
        valves.get(next).unwrap().flow * next_countdown
            + score(next_countdown, rest, distances, valves)
    }
}

fn process_part1(input: Lines) -> usize {
    let valve_map = parse_input(input)
        .map(|v| (v.key, v))
        .collect::<HashMap<ValveKey, Valve>>();

    let distances = build_distances(&valve_map);

    walks(30, START, &distances, false)
        .iter()
        .map(|w| score(30, w, &distances, &valve_map))
        .max()
        .unwrap()
}

fn pairs<T: 'static + Clone>(input: &Vec<T>) -> impl Iterator<Item = (T, T)> + '_ {
    (0..input.len()).flat_map(move |i| {
        ((i + 1)..input.len())
            .map(move |j| (input.get(i).unwrap().clone(), input.get(j).unwrap().clone()))
    })
}

fn process_part2(input: Lines) -> usize {
    let valve_map = parse_input(input)
        .map(|v| (v.key, v))
        .collect::<HashMap<ValveKey, Valve>>();

    let distances = build_distances(&valve_map);

    let walks = walks(26, START, &distances, true)
        .iter()
        .filter(|w| w.len() > 1)
        .cloned()
        .map(|w| (w.clone(), score(26, &w, &distances, &valve_map)))
        .collect::<Vec<_>>();

    pairs(&walks)
        .filter(|((a, _), (b, _))| {
            !a.iter()
                .filter(|item_a| item_a != &&START)
                .any(|item_a| b.contains(item_a))
        })
        .map(|((_, score_a), (_, score_b))| score_a + score_b)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.lines());
        assert_eq!(result, 1651);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.lines());
        assert_eq!(result, 1707);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
