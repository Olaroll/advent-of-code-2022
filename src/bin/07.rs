use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use itertools::Itertools;

struct Dir {
    name: String,
    parent: Weak<Dir>,
    files: Vec<File>,
    dirs: RefCell<Vec<Rc<Dir>>>,
}

struct File {
    name: String,
    size: u32,
}

impl Dir {
    fn new<'a, I>(mut input: &mut I, name: &str, parent: Weak<Dir>) -> Rc<Self>
    where I: Iterator<Item=&'a str>
    {
        let mut dir = Dir {
            name: name.to_string(),
            parent,
            files: vec![],
            dirs: RefCell::new(vec![]),
        };

        let mut last = None;

        for line in &mut input {
            let cmd = parse_command(&line);
            match cmd {
                Some(Command::LS) => continue,
                Some(Command::CD(_)) => {
                    last = Some(line);
                    break
                }
                None => {
                    if line.starts_with("dir") {
                        // We're gonna add these when we CD into them
                        continue
                    }

                    let mut iter = line.split(' ');
                    let size = iter.next().unwrap().parse::<u32>().unwrap();
                    let name = iter.next().unwrap();
                    dir.files.push(File {
                        name: name.to_string(),
                        size,
                    })
                }
            }
        }

        let dir = Rc::new(dir);

        loop {
            let line = last.take().or_else(|| input.next());
            let line = match line {
                None => break,
                Some(v) => v,
            };

            let cmd = parse_command(&line);
            match cmd {
                None => panic!("unexpected non-command"),
                Some(Command::LS) => panic!("unexpected ls command"),
                Some(Command::CD(next_name)) => {
                    if next_name == ".." {
                        break
                    }

                    let next_dir = Dir::new(&mut *input, next_name, Rc::downgrade(&dir));
                    dir.dirs.borrow_mut().push(next_dir);
                }
            }
        }

        dir
    }

    fn size(&self) -> u32 {
        let files: u32 = self.files.iter().map(|f| f.size()).sum();
        let dirs: u32 = self.dirs.borrow().iter()
            .map(|d| d.size())
            .sum();
        files + dirs
    }

    fn walk_dirs(start: Rc<Dir>) -> impl Iterator<Item=Rc<Dir>> {
        let mut vec = vec![start];

        let mut i = 0;
        while i < vec.len() {
            let mut dirs = vec[i].dirs.borrow().clone();
            vec.append(&mut dirs);

            i += 1;
        }

        vec.into_iter()
    }
}

impl File {
    fn size(&self) -> u32 {
        self.size
    }
}

fn parse_command(cmd: &str) -> Option<Command> {
    if !cmd.starts_with('$') {
        return None
    }

    let mut iter = cmd.split(' ').skip(1);
    match iter.next()? {
        "cd" => Some(Command::CD(iter.next()?)),
        "ls" => Some(Command::LS),
        _ => panic!("unrecognised command"),
    }
}



enum Command<'a> {
    LS,
    CD(&'a str),
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines().skip(1);
    let root = Dir::new(&mut lines, "/", Weak::new());

    let sum = Dir::walk_dirs(root)
        .map(|dir| dir.size())
        .filter(|size| *size <= 100000)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().skip(1);
    let root = Dir::new(&mut lines, "/", Weak::new());

    // Update size - Empty space
    let size_needed = 30000000 - (70000000 - root.size());

    Dir::walk_dirs(root)
        .map(|dir| dir.size())
        .filter(|size| *size >= size_needed)
        .sorted()
        .next()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
