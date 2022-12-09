use std::collections::HashSet;
use std::iter;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Cmd {
    Up,
    Down,
    Left,
    Right
}

impl TryFrom<char> for Cmd {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("invalid char")
        }
    }
}

struct Segment {
    x: i32,
    y: i32,
}

impl Segment {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    fn step(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn step_cmd(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Up => self.step(0, 1),
            Cmd::Down => self.step(0, -1),
            Cmd::Left => self.step(-1, 0),
            Cmd::Right => self.step(1, 0),
        }
    }

    fn follow(&mut self, other: &Self) {
        if (other.x - self.x).abs() > 1 || (other.y - self.y).abs() > 1 {
            self.x += (other.x - self.x).clamp(-1, 1);
            self.y += (other.y - self.y).clamp(-1, 1);
        }
    }
}

fn get_cmds(input: &str) -> impl Iterator<Item=Cmd> + '_ {
    input.lines()
        .filter_map(|line| line.split_once(' '))
        .filter_map(|(left, right)| left.chars().exactly_one().ok().zip(right.parse::<usize>().ok()))
        .filter_map(|(left, n)| Cmd::try_from(left).ok().map(|cmd| (cmd, n)))
        .flat_map(|(cmd, n)| iter::repeat(cmd).take(n))
}

pub fn part_one(input: &str) -> Option<u32> {
    let cmds = get_cmds(input);

    let mut set = HashSet::new();

    let mut head = Segment::new();
    let mut tail = Segment::new();

    for cmd in cmds {
        head.step_cmd(cmd);
        tail.follow(&head);

        set.insert((tail.x, tail.y));
    }

    Some(set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cmds = get_cmds(input);

    let mut set = HashSet::new();

    let mut segments: Vec<_> = iter::repeat_with(|| Segment::new()).take(10).collect();

    for cmd in cmds {
        segments[0].step_cmd(cmd);

        let mut i = 1;
        while i < segments.len() {
            let (left, right) = segments.split_at_mut(i);
            let head = left.last().unwrap();
            let tail = right.first_mut().unwrap();

            tail.follow(head);

            i += 1;
        }

        let tail = segments.last().unwrap();
        set.insert((tail.x, tail.y));
    }

    Some(set.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
