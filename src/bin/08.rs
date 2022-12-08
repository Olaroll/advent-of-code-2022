use itertools::Itertools;
use rayon::prelude::*;

fn count_steps(vec: &[Vec<u8>], x: i32, y: i32, x_step: i32, y_step: i32) -> u32 {
    assert!(x_step != 0 || y_step != 0);

    let max = vec[y as usize][x as usize];
    let mut x = x + x_step;
    let mut y = y + y_step;
    let mut count = 0;

    while y >= 0
        && (y as usize) < vec.len()
        && x >= 0
        && (x as usize) < vec[0].len()
    {
        let height = vec[y as usize][x as usize];

        count += 1;
        if height >= max {
            break;
        }

        x += x_step;
        y += y_step;
    }

    count
}

fn count_cardinals(vec: &[Vec<u8>], x: i32, y: i32) -> u32 {
    count_steps(vec, x, y, 1, 0)
        * count_steps(vec, x, y, -1, 0)
        * count_steps(vec, x, y, 0, 1)
        * count_steps(vec, x, y, 0, -1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let vec: Vec<Vec<u8>> = input.lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let mut cache = Vec::new();
    vec.iter()
        .for_each(|row| cache.push(vec![false; row.len()]));

    let height = vec.len();
    let width = vec[0].len();

    (0..height).for_each(|y| {
        let mut min = None;
        (0..width).for_each(|x| {
            if min.is_none() || vec[y][x] > min.unwrap() {
                min = Some(vec[y][x]);
                cache[y][x] = true
            }
        })
    });

    (0..height).for_each(|y| {
        let mut min = None;
        (0..width).rev().for_each(|x| {
            if min.is_none() || vec[y][x] > min.unwrap() {
                min = Some(vec[y][x]);
                cache[y][x] = true
            }
        })
    });

    (0..width).for_each(|x| {
        let mut min = None;
        (0..height).for_each(|y| {
            if min.is_none() || vec[y][x] > min.unwrap() {
                min = Some(vec[y][x]);
                cache[y][x] = true
            }
        })
    });

    (0..width).for_each(|x| {
        let mut min = None;
        (0..height).rev().for_each(|y| {
            if min.is_none() || vec[y][x] > min.unwrap() {
                min = Some(vec[y][x]);
                cache[y][x] = true
            }
        })
    });

    cache.into_iter().map(|row| {
        row.into_iter().filter(|cell| *cell).count() as u32
    }).sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec: Vec<Vec<u8>> = input.lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();

    let height = vec.len();
    let width = vec[0].len();

    (0..height)
        .flat_map(|y| {
            (0..width).map(move |x| (x, y))
        })
        .map(|(x, y)| count_cardinals(&vec, x as i32, y as i32))
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
