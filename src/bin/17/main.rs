use std::{collections::HashMap, iter::repeat, ops::Range, str::Chars};

#[derive(Clone)]
enum Dir {
    Left,
    Right,
}

const WIDTH: usize = 7;

const BOUNDS: Range<i32> = 0..WIDTH as i32;

const MAGIC: usize = 16;

type Slice = [bool; WIDTH];

type Stack = Vec<Slice>;

type Point = (usize, usize);

type State = (usize, usize, Stack);

const START_X: usize = 2;

struct Shape {
    points: Vec<Point>,
}

impl Shape {
    fn new(pattern: Vec<Point>, bottom_y: usize) -> Self {
        let points = pattern
            .iter()
            .map(|(x, y)| (x + START_X, y + bottom_y))
            .collect();
        Self { points }
    }

    fn flat(bottom_y: usize) -> Self {
        Self::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)], bottom_y)
    }

    fn plus(bottom_y: usize) -> Self {
        Self::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], bottom_y)
    }

    fn j(bottom_y: usize) -> Self {
        Self::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], bottom_y)
    }

    fn i(bottom_y: usize) -> Self {
        Self::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)], bottom_y)
    }

    fn square(bottom_y: usize) -> Self {
        Self::new(vec![(0, 0), (0, 1), (1, 0), (1, 1)], bottom_y)
    }

    fn shift(&mut self, dir: &Dir, stack: &Stack) {
        let offset = match dir {
            Dir::Left => -1,
            Dir::Right => 1,
        };

        let next_points = self
            .points
            .iter()
            .map(|(x, y)| {
                let next_x = *x as i32 + offset;
                if BOUNDS.contains(&next_x) && !stack[*y][next_x as usize] {
                    Some((next_x.try_into().unwrap(), *y))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if next_points.iter().all(|np| np.is_some()) {
            self.points = next_points.iter().filter_map(|np| *np).collect();
        }
    }

    fn fall(&mut self, stack: &Stack) -> Option<()> {
        let next_points = self
            .points
            .iter()
            .map(|(x, y)| {
                if *y > 0 && !stack[y - 1][*x] {
                    Some((*x, y - 1))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if next_points.iter().all(|np| np.is_some()) {
            self.points = next_points.iter().filter_map(|np| *np).collect();
            Some(())
        } else {
            None
        }
    }
}

fn pad(source: &mut Stack) {
    repeat([false; 7])
        .take(7)
        .for_each(|slice| source.push(slice));
}

fn trim(source: &mut Stack) {
    if !source.last().unwrap().iter().any(|x| *x) {
        source.pop();
        trim(source);
    }
}

fn parse_input(input: Chars) -> impl Iterator<Item = Dir> + '_ {
    input.map(|c| match c {
        '<' => Dir::Left,
        '>' => Dir::Right,
        _ => panic!(),
    })
}

fn generate_shapes() -> impl Iterator<Item = (usize, &'static fn(usize) -> Shape)> {
    repeat(
        [Shape::flat, Shape::plus, Shape::j, Shape::i, Shape::square]
            .iter()
            .enumerate(),
    )
    .flatten()
}

fn process(n: usize, input: Chars) -> usize {
    let mut directions = repeat(parse_input(input).enumerate().collect::<Vec<_>>()).flatten();
    let mut shapes = generate_shapes();

    let mut stack = vec![];

    let mut state_set = HashMap::<State, (usize, usize)>::new();
    let mut loop_adjustment = None;

    let mut i = 0;
    while i < n {
        let bottom_y = stack.len() + 3;

        pad(&mut stack);

        let (shape_idx, shape_fn) = shapes.next().unwrap();
        let mut shape = shape_fn(bottom_y);

        let mut direction_idx = 0;

        directions
            .by_ref()
            .map(|(dir_idx, dir)| {
                direction_idx = dir_idx;
                shape.shift(&dir, &stack);
                shape.fall(&stack)
            })
            .take_while(|x| x.is_some())
            .for_each(drop);

        for (x, y) in shape.points {
            stack[y][x] = true;
        }

        trim(&mut stack);

        if loop_adjustment.is_none() && stack.len() >= MAGIC {
            let state: State = (
                shape_idx,
                direction_idx,
                stack[(stack.len() - MAGIC)..].to_vec(),
            );

            if let Some((prev_i, prev_len)) = state_set.get(&state) {
                let jump_i = n - ((n - prev_i) % (i - prev_i));
                loop_adjustment = Some((stack.len() - prev_len) * ((jump_i - i) / (i - prev_i)));
                i = jump_i;
            }
            state_set.insert(state, (i, stack.len()));
        }

        i += 1;
    }

    stack.len() + loop_adjustment.unwrap_or_default()
}

fn process_part1(input: Chars) -> usize {
    process(2022, input)
}

fn process_part2(input: Chars) -> usize {
    process(1000000000000, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT.chars());
        assert_eq!(result, 3068);
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT.chars());
        assert_eq!(result, 1514285714288);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.chars());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.chars());
    println!("{}", result_part2);
}
