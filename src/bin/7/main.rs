use std::iter::{once, Iterator};
use std::str::Lines;

struct Dir {
    name: String,
    own_size: usize,
    subdirs: Vec<Dir>,
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            own_size: 0,
            subdirs: vec![],
        }
    }

    fn total_size(&self) -> usize {
        self.own_size + self.subdirs.iter().map(|d| d.total_size()).sum::<usize>()
    }

    fn collect(&self) -> Vec<&Self> {
        once(self)
            .chain(self.subdirs.iter().flat_map(|d| d.collect()))
            .collect()
    }

    fn find(&mut self, path: &[&str]) -> &mut Dir {
        match path.split_first() {
            Some((&head, tail)) => self
                .subdirs
                .iter_mut()
                .find(|d| d.name == head)
                .unwrap()
                .find(tail),
            None => self,
        }
    }

    fn ensure(&mut self, subdir_name: String) {
        if self.subdirs.iter().any(|d| d.name == subdir_name) {
            return;
        }

        self.subdirs.push(Dir::new(subdir_name));
    }
}

fn populate(dir: &mut Dir, input: &str) {
    match input.split_whitespace().collect::<Vec<&str>>()[..] {
        ["dir", subdir_name] => {
            dir.ensure(subdir_name.to_string());
        }
        [file_size, _] => dir.own_size += file_size.parse::<usize>().unwrap(),
        _ => panic!(),
    };
}

fn concat_vec<T: std::clone::Clone>(a: Vec<T>, b: T) -> Vec<T> {
    let mut rslt = a.to_vec();
    rslt.push(b);
    rslt
}

fn process_internal(mut root: Dir, path: Vec<&str>, mut input: Lines) -> Dir {
    let next_path = match input.next() {
        Some(line) => match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => path[..0].to_vec(),
            ["$", "cd", ".."] => path[..(path.len() - 1)].to_vec(),
            ["$", "cd", cd_path] => concat_vec(path, cd_path),
            ["$", "ls"] => path,
            _ => {
                populate(root.find(&path), line);
                path
            }
        },
        None => return root,
    };

    process_internal(root, next_path, input)
}

fn process(input: Lines) -> Dir {
    const ROOT_NAME: &str = "/";

    let root = Dir {
        name: ROOT_NAME.to_string(),
        own_size: 0,
        subdirs: vec![],
    };

    process_internal(root, vec![ROOT_NAME], input)
}

fn process_part1(input: Lines) -> usize {
    process(input)
        .collect()
        .iter()
        .map(|d| d.total_size())
        .filter(|s| *s <= 100000)
        .sum()
}

fn process_part2(input: Lines) -> usize {
    const TOTAL_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;

    let root = process(input);
    let used_space = root.total_size();

    root.collect()
        .iter()
        .map(|d| d.total_size())
        .filter(|s| *s >= REQUIRED_SPACE - (TOTAL_SPACE - used_space))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(process_part1(INPUT.lines()), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process_part2(INPUT.lines()), 24933642);
    }
}

fn main() {
    let input = include_str!("input.txt");

    let result_part1 = process_part1(input.lines());
    println!("{}", result_part1);

    let result_part2 = process_part2(input.lines());
    println!("{}", result_part2);
}
