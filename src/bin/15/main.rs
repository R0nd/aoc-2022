use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::RangeInclusive,
    str::Lines,
    vec,
};

type Point = (i32, i32);

type Entry = (Point, Point);

struct Ranges {
    ranges: Vec<RangeInclusive<i32>>,
}

fn overlap<T: Ord>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn union<T: Ord + Copy>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> RangeInclusive<T> {
    *min(a.start(), b.start())..=*max(a.end(), b.end())
}

fn intersection<T: Ord + Copy>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> RangeInclusive<T> {
    *max(a.start(), b.start())..=*min(a.end(), b.end())
}

fn difference<T: Ord + Copy + std::ops::Add<i32, Output = T> + std::ops::Sub<i32, Output = T>>(
    a: &RangeInclusive<T>,
    b: &RangeInclusive<T>,
) -> Vec<RangeInclusive<T>> {
    let mut result = vec![];

    if b.start() > a.start() && b.start() <= a.end() {
        result.push(*a.start()..=(*b.start() - 1));
    }

    if b.end() < a.end() && b.end() >= a.start() {
        result.push((*b.end() + 1)..=*a.end());
    }

    result
}

impl Ranges {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn push(&mut self, value: RangeInclusive<i32>) {
        let mut next_ranges = vec![];

        let mut agg = value.clone();
        for r in self.ranges.iter().cloned() {
            if overlap(&r, &value) {
                agg = union(&agg, &r);
            } else {
                next_ranges.push(r);
            }
        }
        next_ranges.push(agg);

        next_ranges.sort_by_key(|r| *r.start());

        self.ranges = next_ranges;
    }

    fn clamp(&mut self, bounds: RangeInclusive<i32>) {
        self.ranges = self
            .ranges
            .iter()
            .filter(|r| overlap(r, &bounds))
            .map(|r| intersection(r, &bounds))
            .collect()
    }

    fn total_len(&self) -> usize {
        self.ranges
            .iter()
            .map(|r| (r.end() - r.start()) as usize + 1)
            .sum()
    }
}

impl FromIterator<RangeInclusive<i32>> for Ranges {
    fn from_iter<T: IntoIterator<Item = RangeInclusive<i32>>>(iter: T) -> Self {
        let mut ranges = Self::new();
        iter.into_iter().for_each(|r| ranges.push(r));
        ranges
    }
}

fn parse_coord(input: &str) -> i32 {
    match input.split('=').collect::<Vec<&str>>()[..] {
        [_, v] => v.trim_end_matches([',', ':']).parse().unwrap(),
        _ => panic!(),
    }
}

fn parse_entry(input: &str) -> Entry {
    match input.split_whitespace().collect::<Vec<&str>>()[..] {
        ["Sensor", "at", s_x, s_y, "closest", "beacon", "is", "at", b_x, b_y] => (
            (parse_coord(s_x), parse_coord(s_y)),
            (parse_coord(b_x), parse_coord(b_y)),
        ),
        _ => panic!(),
    }
}

fn parse_input(input: Lines) -> impl Iterator<Item = Entry> + '_ {
    input.map(parse_entry)
}

fn area_slice(&(sensor, beacon): &Entry, row: i32) -> Option<RangeInclusive<i32>> {
    let distance = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
    let offset = distance as i32 - sensor.1.abs_diff(row) as i32;

    match offset {
        0.. => Some((sensor.0 - offset)..=(sensor.0 + offset)),
        _ => None,
    }
}

fn process_part1(y: i32, input: Lines) -> usize {
    let entries = parse_input(input).collect::<Vec<Entry>>();

    let beacon_xs = entries
        .iter()
        .filter(|(_, (_, b_y))| *b_y == y)
        .map(|(_, (b_x, _))| b_x)
        .copied()
        .collect::<HashSet<_>>();

    let slices = entries
        .iter()
        .filter_map(|e| area_slice(e, y))
        .collect::<Ranges>();

    slices.total_len()
        - beacon_xs
            .iter()
            .filter(|x| slices.ranges.iter().any(|r| r.contains(x)))
            .count()
}

fn process_part2(bound: i32, input: Lines) -> i64 {
    let entries = parse_input(input).collect::<Vec<Entry>>();

    for y in 0..=bound {
        let beacon_xs = entries
            .iter()
            .filter(|(_, (_, b_y))| *b_y == y)
            .map(|(_, (b_x, _))| b_x)
            .copied()
            .collect::<HashSet<_>>();

        let mut slices = entries
            .iter()
            .filter_map(|e| area_slice(e, y))
            .chain(beacon_xs.iter().copied().map(|x| x..=x))
            .collect::<Ranges>();

        slices.clamp(0..=bound);

        if slices.total_len() == bound.try_into().unwrap() {
            let positions = slices.ranges.iter().fold(vec![0..=bound], |agg, next| {
                agg.iter().flat_map(|r| difference(r, next)).collect()
            });

            match &positions[..] {
                [p] if p.start() == p.end() => return *p.start() as i64 * 4000000 + y as i64,
                _ => panic!(),
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(10, INPUT.lines());
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(20, INPUT.lines());
        assert_eq!(result, 56000011);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(2000000, input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(4000000, input.lines());
    println!("{}", result_part2);
}
