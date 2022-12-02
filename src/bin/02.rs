use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn from(s: &str) -> Option<Self> {
        match s {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None
        }
    }

    fn play_against(self, other: Self) -> u32 {
        match self {
            RPS::Rock => {
                let x = 1;
                match other {
                    RPS::Rock => x + 3,
                    RPS::Paper => x + 0,
                    RPS::Scissors => x + 6,
                }
            }
            RPS::Paper => {
                let x = 2;
                match other {
                    RPS::Rock => x + 6,
                    RPS::Paper => x + 3,
                    RPS::Scissors => x + 0,
                }
            }
            RPS::Scissors => {
                let x = 3;
                match other {
                    RPS::Rock => x + 0,
                    RPS::Paper => x + 6,
                    RPS::Scissors => x + 3,
                }
            }
        }
    }

    fn play_against2(self, other: Self) -> u32 {
        match self {
            RPS::Rock => {
                let x = 0;
                match other {
                    RPS::Rock => x + 3,
                    RPS::Paper => x + 1,
                    RPS::Scissors => x + 2,
                }
            }
            RPS::Paper => {
                let x = 3;
                match other {
                    RPS::Rock => x + 1,
                    RPS::Paper => x + 2,
                    RPS::Scissors => x + 3,
                }
            }
            RPS::Scissors => {
                let x = 6;
                match other {
                    RPS::Rock => x + 2,
                    RPS::Paper => x + 3,
                    RPS::Scissors => x + 1,
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines()
        .filter_map(|line| line.split(' ').collect_tuple::<(_, _)>())
        .filter_map(|(enemy, me)| RPS::from(enemy).zip(RPS::from(me)))
        .map(|(enemy, me)| me.play_against(enemy))
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines()
        .filter_map(|line| line.split(' ').collect_tuple::<(_, _)>())
        .filter_map(|(enemy, me)| RPS::from(enemy).zip(RPS::from(me)))
        .map(|(enemy, me)| me.play_against2(enemy))
        .sum1()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
