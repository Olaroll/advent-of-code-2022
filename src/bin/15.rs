use std::collections::{HashMap, HashSet};
use glam::IVec2;
use itertools::Itertools;
use crate::parsers::parse_input;
use rayon::prelude::*;
use rust_lapper::{Lapper, Interval};

#[derive(Debug, Clone)]
struct Sensor {
    pos: IVec2,
    closest_beacon: IVec2,
    range: u32,
}

impl Sensor {
    fn from_pair(sensor_pos: IVec2, beacon_pos: IVec2) -> Self {
        let range = (beacon_pos - sensor_pos).abs().as_uvec2()
            .to_array()
            .into_iter()
            .sum();

        Self {
            pos: sensor_pos,
            closest_beacon: beacon_pos,
            range,
        }
    }

    fn is_in_range(&self, pos: IVec2) -> bool {
        (self.pos - pos).abs().to_array().into_iter().sum::<i32>() <= self.range as i32
    }
}

#[derive(Debug, Clone, Default)]
struct Board {
    sensors: Vec<Sensor>,
    beacon_rows: HashMap<i32, HashSet<i32>>,
}

impl Board {
    fn from_input(input: &str) -> Self {
        let mut board = Board::default();

        parse_input(input).expect("couldn't parse input").1
            .into_iter()
            .for_each(|(sensor, beacon)| {
                board.add_sensor(Sensor::from_pair(sensor, beacon))
            });

        board
    }

    fn add_sensor(&mut self, sensor: Sensor) {
        let entry = self.beacon_rows.entry(sensor.closest_beacon.y).or_default();
        entry.insert(sensor.closest_beacon.x);

        self.sensors.push(sensor)
    }

    fn count_blocked(&self, row: i32) -> u64 {
        let mut intervals: Vec<_> = self.sensors.iter()
            .map(|sens| {
                let dist = (sens.pos.y - row).abs();
                let left_over = sens.range as i32 - dist;
                (sens.pos.x-left_over, sens.pos.x + left_over)
            })
            .filter(|(start, end)| start <= end)
            .collect();

        intervals.sort_unstable_by_key(|(start, _)| *start);

        let union: u64 = intervals.into_iter()
            .coalesce(|(start_a, end_a), (start_b, end_b)| {
                if end_a >= start_b {
                    Ok((start_a, end_a.max(end_b)))
                } else {
                    Err(((start_a, end_a), (start_b, end_b)))
                }
            })
            .map(|(start, end)| (end - start + 1) as u64)
            .sum();


        let beacons_in_row = self.beacon_rows.get(&row)
            .map(|row| row.len())
            .unwrap_or(0);

        union - beacons_in_row as u64
    }

    fn find_distress_beacon(&self, min: IVec2, max: IVec2) -> Option<IVec2> {
        (min.y..max.y).into_par_iter().find_map_any(|y| {
            let mut x = min.x;
            loop {
                if let Some(x_offset) = self.get_next_offset(IVec2::new(x, y)) {
                    x += x_offset;

                    if x > max.x {
                        break None
                    }
                } else {
                    break Some(IVec2::new(x, y))
                }
            }
        })
    }

    fn get_next_offset(&self, pos: IVec2) -> Option<i32> {
        self.sensors.iter()
            .find(|&sens| sens.is_in_range(pos))
            .map(|sens| {
                sens.range as i32 - (sens.pos.y - pos.y).abs() + (sens.pos.x - pos.x) + 1
            })
    }
}

mod parsers {
    use glam::IVec2;
    use nom::bytes::complete::{is_a, tag};
    use nom::character::complete::{char, i32 as i32fn, line_ending};
    use nom::multi::separated_list1;
    use nom::sequence::{pair, preceded, separated_pair};

    pub fn parse_input(input: &str) -> nom::IResult<&str, Vec<(IVec2, IVec2)>> {
        separated_list1(line_ending, parse_line)(input)
    }

    fn parse_line(input: &str) -> nom::IResult<&str, (IVec2, IVec2)> {
        pair(
            preceded(tag("Sensor at "), parse_ivec),
            preceded(tag(": closest beacon is at "), parse_ivec),
        )(input)
    }

    fn parse_ivec(input: &str) -> nom::IResult<&str, IVec2> {
        let (remainder, (x, y)) = separated_pair(parse_xy, tag(", "), parse_xy)(input)?;
        let ivec = IVec2::new(x, y);
        Ok((remainder, ivec))
    }

    fn parse_xy(input: &str) -> nom::IResult<&str, i32> {
        preceded(pair(is_a("xy"), char('=')), i32fn)(input)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let board = Board::from_input(input);

    let test_row = if cfg!(test) {
        10
    } else {
        2000000
    };


    Some(board.count_blocked(test_row))
}

pub fn part_two(input: &str) -> Option<i64> {
    let board = Board::from_input(input);

    let search_min = IVec2::ZERO;
    let search_max = if cfg!(test) {
        IVec2::new(20, 20)
    } else {
        IVec2::new(4000000, 4000000)
    };

    board.find_distress_beacon(search_min, search_max)
        .map(|pos| 4000000 * pos.x as i64 + pos.y as i64)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
