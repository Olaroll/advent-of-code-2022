use itertools::Itertools;
use advent_of_code::helpers;


pub fn sum_elves(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines()
        .batching(|lines| {
            lines.take_while(|&line| !line.is_empty())
                .filter_map(|str| str.parse::<u32>().ok())
                .sum1::<u32>()
        })
}


pub fn part_one(input: &str) -> Option<u32> {
    sum_elves(input).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    sum_elves(input)
        .sorted_unstable()
        .rev()
        .take(3)
        .sum1()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
