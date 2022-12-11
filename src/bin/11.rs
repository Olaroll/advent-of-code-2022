use lazy_static::lazy_static;
use regex::Regex;
use anyhow::{anyhow, Context, Result};
use derivative::Derivative;
use itertools::Itertools;

#[derive(Derivative)]
#[derivative(Debug)]
struct Monkey {
    inspection_count: u32,

    items: Vec<Item>,
    #[derivative(Debug="ignore")]
    operation: Box<dyn Fn(Item) -> Item>,

    test_divisor: u32,
    target_true: MonkeyIndex,
    target_false: MonkeyIndex,
}

lazy_static! {
    static ref ITEMS_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref OP_RE: Regex = Regex::new(r"= ?(.+)").unwrap();
    static ref TEST_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref TRUE_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref FALSE_RE: Regex = Regex::new(r"\d+").unwrap();
}

impl Monkey {
    fn throw(&mut self) -> Vec<(Item, MonkeyIndex)> {
        self.inspection_count += self.items.len() as u32;

        let operation = &*self.operation;
        self.items.drain(..)
            .map(|item| operation(item) as Item)
            .map(|item| item / 3)
            .map(|item| {
                let target = if item % self.test_divisor as Item == 0 {
                    self.target_true
                } else {
                    self.target_false
                };
                (item, target)
            })
            .collect()
    }

    fn throw_unchained(&mut self, common_multiple: u64) -> Vec<(Item, MonkeyIndex)> {
        self.inspection_count += self.items.len() as u32;

        let operation = &*self.operation;
        self.items.drain(..)
            .map(|item| operation(item) as Item)
            .map(|mut item| {
                item %= common_multiple;
                let target = if item % self.test_divisor as Item == 0 {
                    self.target_true
                } else {
                    self.target_false
                };
                (item, target)
            })
            .collect()
    }

    fn catch(&mut self, item: Item) {
        self.items.push(item)
    }

    fn from_iter<'a, I>(lines: &mut I) -> Option<Result<Self>>
    where I: Iterator<Item=&'a str>
    {
        let _monkey = lines.find(|&line| !line.is_empty())?;
        let items_str = lines.next()?;
        let op_str = lines.next()?;
        let test_str = lines.next()?;
        let true_str = lines.next()?;
        let false_str = lines.next()?;

        let res: Result<Self> = (|| {
            let items: Vec<_> = ITEMS_RE.find_iter(items_str)
                .map(|num| num.as_str().parse::<Item>().map_err(|e| anyhow!(e)))
                .collect::<Result<_>>()?;

            let operation = &OP_RE.captures(op_str).context("couldn't match operation")?[1];
            let operation = operation.parse::<meval::Expr>()?.bind("old")?;
            let operation: Box<dyn Fn(Item) -> Item> = Box::new(move |old| operation(old as f64).trunc() as Item);

            let test_divisor: u32 = TEST_RE.find(test_str).context("couldn't match divisor")?.as_str().parse()?;
            let target_true: MonkeyIndex = TRUE_RE.find(true_str).context("couldn't match true target")?.as_str().parse()?;
            let target_false: MonkeyIndex = FALSE_RE.find(false_str).context("couldn't match false target")?.as_str().parse()?;

            Ok(Self {
                inspection_count: 0,
                items,
                operation,
                test_divisor,
                target_true,
                target_false,
            })
        })();

        Some(res)
    }
}

type Item = u64;
type MonkeyIndex = usize;

fn get_monkeys(input: &str) -> Vec<Monkey> {
    input.lines().into_iter()
        .batching(|lines| Monkey::from_iter(lines))
        .collect::<Result<_>>()
        .expect("couldn't parse a monkey")
}

fn get_answer(mut monkeys: Vec<Monkey>) -> Option<u64> {
    monkeys.sort_unstable_by_key(|monkey| monkey.inspection_count);
    monkeys.into_iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspection_count as u64)
        .product1()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys: Vec<_> = get_monkeys(input);

    for _round in 1..=20 {
        for i in 0..monkeys.len() {
            let res = monkeys[i].throw();
            for (item, j) in res {
                monkeys[j].catch(item)
            }
        }
    }

    get_answer(monkeys)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys: Vec<_> = get_monkeys(input);

    let common_multiple = monkeys.iter()
        .map(|monkey| monkey.test_divisor as u64)
        .product();

    for _round in 1..=10000 {
        for i in 0..monkeys.len() {
            let res = monkeys[i].throw_unchained(common_multiple);
            for (item, j) in res {
                monkeys[j].catch(item)
            }
        }
    }

    get_answer(monkeys)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
