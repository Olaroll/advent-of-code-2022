extern crate core;

use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;

fn char_score(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - b'a' + 1,
        'A'..='Z' => c as u8 - b'A' + 27,
        _ => panic!("can't give a score to {}", c),
    }
}

fn intersect<T>(mut sets: Vec<HashSet<T>>) -> HashSet<T>
where T: Hash + Eq
{
    let i = sets.iter().position_min_by_key(|set| set.len()).expect("empty vec of iterators given");
    let mut set = sets.swap_remove(i);
    set.retain(|item| !sets.iter().any(|set2| !set2.contains(item)));
    set
}

fn setify(s: &str) -> HashSet<char> {
    HashSet::from_iter(s.chars())
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines()
        .map(|line| line.split_at(line.len()/2))
        .map(|(a, b)| vec![setify(a), setify(b)])
        .map(|sets| intersect(sets).into_iter().exactly_one().expect("only one!"))
        .map(|c| char_score(c) as u32)
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines()
        .batching(|lines| lines.next_tuple::<(_, _, _)>())
        .map(|(a, b, c)| vec![setify(a), setify(b), setify(c)])
        .map(|sets| intersect(sets).into_iter().exactly_one().expect("only one!"))
        .map(|ch| char_score(ch) as u32)
        .sum1()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
