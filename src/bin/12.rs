use std::cmp::Reverse;
use std::collections::HashSet;
use std::hash::Hash;
use glam::UVec2;
use priority_queue::PriorityQueue;

struct Board {
    heights: Vec<u8>,
    size: UVec2,
    start: UVec2,
    end: UVec2,
}

impl Board {
    fn from_input(input: &str) -> Self {
        let mut heights = Vec::with_capacity(input.len());

        let mut start = None;
        let mut end = None;

        let mut width = None;

        for (y, line) in input.lines().enumerate() {
            let width = width.get_or_insert(line.len());
            assert_eq!(line.len(), *width, "Uneven row length at line {}", y + 1);

            for (x, byte) in line.as_bytes().iter().enumerate() {
                let height = match byte {
                    b'a'..=b'z' => byte_to_height(*byte),
                    b'S' => {
                        start = Some(UVec2::new(x as u32, y as u32));
                        byte_to_height(b'a')
                    }
                    b'E' => {
                        end = Some(UVec2::new(x as u32, y as u32));
                        byte_to_height(b'z')
                    }
                    _ => panic!("unsupported char {}", *byte as char)
                };

                heights.push(height);
            }
        }

        let start = start.expect("Didn't find a start cell");
        let end = end.expect("Didn't find an end cell");
        let width = width.expect("No rows in the input");
        let size = UVec2::new(width as u32, (heights.len() / width) as u32);

        Self {
            heights,
            size,
            start,
            end,
        }
    }

    fn get_height(&self, pos: UVec2) -> u8 {
        self.heights[(self.size.x * pos.y + pos.x) as usize]
    }

    fn dijkstra(&self, part2: bool) -> Option<u32> {
        let mut queue = PriorityQueue::new();
        let mut seen = HashSet::new();

        queue.push(self.end, Reverse(0));
        seen.insert(self.end);

        while !queue.is_empty() {
            let (pos, depth) = queue.pop().unwrap();
            if (!part2 && pos == self.start) || (part2 && self.get_height(pos) == 0) {
                // self.print_visited(&seen);
                return Some(depth.0);
            }

            let next_depth = Reverse(depth.0 + 1);

            if pos.x > 0 {
                let next_pos = pos - UVec2::X;
                if self.step_allowed(next_pos, pos) && seen.insert(next_pos) {
                    queue.push(next_pos, next_depth);
                }
            }

            if pos.y > 0 {
                let next_pos = pos - UVec2::Y;
                if self.step_allowed(next_pos, pos) && seen.insert(next_pos) {
                    queue.push(next_pos, next_depth);
                }
            }

            if pos.x < self.size.x - 1 {
                let next_pos = pos + UVec2::X;
                if self.step_allowed(next_pos, pos) && seen.insert(next_pos) {
                    queue.push(next_pos, next_depth);
                }
            }

            if pos.y < self.size.y - 1 {
                let next_pos = pos + UVec2::Y;
                if self.step_allowed(next_pos, pos) && seen.insert(next_pos) {
                    queue.push(next_pos, next_depth);
                }
            }
        }

        // self.print_visited(&seen);
        None
    }

    fn step_allowed(&self, from: UVec2, to: UVec2) -> bool {
        let from = self.get_height(from);
        let to = self.get_height(to);

        from + 1 >= to
    }

    fn print_visited(&self, seen: &HashSet<UVec2>) {
        (0..self.size.y).for_each(|y| {
            (0..self.size.x).for_each(|x| {
                let ansi = if seen.contains(&UVec2::new(x, y)) {
                    "\x1b[32m"
                } else {
                    "\x1b[0m"
                };
                print!("{ansi}{:>3}", self.get_height(UVec2::new(x, y)));
            });
            println!();
        });
    }
}


fn byte_to_height(byte: u8) -> u8 {
    byte - b'a'
}




pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::from_input(input);

    board.dijkstra(false)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = Board::from_input(input);

    board.dijkstra(true)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
