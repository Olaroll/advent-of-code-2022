use std::cmp::Ordering;
use itertools::{EitherOrBoth, Itertools};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Item {
    List(Vec<Item>),
    Int(i32),
}

impl Item {
    fn from_str(string: &str) -> Option<Self> {
        serde_json::from_str(string).ok()
    }

    fn from_iter<'a, I>(lines: &mut I) -> Option<Self>
    where I: Iterator<Item=&'a str>
    {
        let line = lines.find(|line| !line.is_empty())?;

        Self::from_str(line)
    }
}


impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, right: &Self) -> Ordering {
        let left = self;

        match (left, right) {
            (Self::Int(l), Self::Int(r)) => {
                l.cmp(r)
            }
            (Self::List(l), Self::List(r)) => {
                l.iter().zip_longest(r.iter())
                    .map(|lr| {
                        match lr {
                            EitherOrBoth::Both(l, r) => l.cmp(r),
                            EitherOrBoth::Left(_) => Ordering::Greater,
                            EitherOrBoth::Right(_) => Ordering::Less,
                        }
                    })
                    .find(|res| res != &Ordering::Equal)
                    .unwrap_or(Ordering::Equal)
            }
            (Self::Int(l), Self::List(_)) => {
                let left = Self::List(vec![Self::Int(*l)]);
                left.cmp(right)
            }
            (Self::List(_), Self::Int(r)) => {
                let right = &Self::List(vec![Self::Int(*r)]);
                left.cmp(right)
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines()
        .batching(|lines| {
            Some((
                Item::from_iter(lines)?,
                Item::from_iter(lines)?,
            ))
        })
        .enumerate()
        .filter_map(|(i, (left, right))| {
            let res = left < right;
            res.then_some(i as u32 + 1)
        })
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut items: Vec<_> = input.lines()
        .batching(|lines| Item::from_iter(lines))
        .collect();

    let divider_a = Item::from_str("[[2]]").unwrap();
    let divider_b = Item::from_str("[[6]]").unwrap();

    items.push(divider_a.clone());
    items.push(divider_b.clone());

    items.sort_unstable();

    items.iter()
        .positions(|item| item == &divider_a || item == &divider_b)
        .map(|i| i as u32 + 1)
        .product1()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
