use itertools::Itertools;

fn inside(a: &[u32; 2], b: &[u32; 2]) -> bool {
    (a[0] <= b[0] && a[1] >= b[1]) || (b[0] <= a[0] && b[1] >= a[1])
}

fn overlaps(a: &[u32; 2], b: &[u32; 2]) -> bool {
    a[0] <= b[1] && a[1] >= b[0]
}

fn solve(input: &str, predicate: impl Fn(&[u32; 2], &[u32; 2]) -> bool) -> Option<u32> {
    let res = input.lines()
        .flat_map(|line| line.split(','))
        .flat_map(|half| half.split('-'))
        .filter_map(|quarter| quarter.parse::<u32>().ok())
        .batching(|quarters| {
            Some([
                [quarters.next()?, quarters.next()?],
                [quarters.next()?, quarters.next()?],
            ])
        })
        .filter(|[a, b]| predicate(a, b))
        .count();

    Some(res as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, inside)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, overlaps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
