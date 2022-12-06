use std::collections::HashSet;

fn solve(input: &str, len: usize) -> Option<u32> {
    input.as_bytes().windows(len)
        .enumerate()
        .find(|(i, bytes)| {
            let set: HashSet<u8> = HashSet::from_iter(bytes.iter().copied());
            set.len() == len
        })
        .map(|(i, _)| (i + len) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
