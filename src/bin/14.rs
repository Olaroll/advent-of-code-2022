use std::collections::HashMap;
use std::iter;
use glam::IVec2;

#[derive(Debug, Copy, Clone)]
enum Cell {
    Wall,
    Sand,
}

struct Sandbox {
    cells: HashMap<IVec2, Cell>,
    floor: i32,
    void_floor: bool,
}

impl Sandbox {
    fn from_input(input: &str) -> Self {
        let mut sandbox = Self {
            cells: HashMap::new(),
            floor: i32::MIN,
            void_floor: true,
        };

        let (_, res) = parsers::parse_input(input).expect("couldn't parse input");

        for wall in res {
            wall.windows(2)
                .flat_map(|pair| iter_line(pair[0], pair[1]))
                .for_each(|point| sandbox.insert(point, Cell::Wall))
        }

        sandbox
    }

    fn insert(&mut self, pos: IVec2, cell: Cell) {
        if let Cell::Wall = cell {
            self.floor = self.floor.max(pos.y)
        }

        self.cells.insert(pos, cell);
    }

    fn contains(&self, pos: IVec2) -> bool {
        self.cells.contains_key(&pos)
            || (!self.void_floor && pos.y >= self.floor)
    }

    fn with_fake_floor(mut self) -> Self {
        self.floor += 2;
        self.void_floor = false;
        self
    }

    const SAND_OFFSETS: [IVec2; 3] = [
        IVec2::new(0, 1),
        IVec2::new(-1, 1),
        IVec2::new(1, 1),
    ];

    fn drop_sand(&mut self, mut pos: IVec2) -> Option<IVec2> {
        while pos.y < self.floor {
            let next_offset = Self::SAND_OFFSETS.into_iter()
                .find(|offset| !self.contains(pos + *offset));

            if let Some(offset) = next_offset {
                pos += offset
            } else {
                self.insert(pos, Cell::Sand);
                return Some(pos);
            }
        }

        // Sand fell past the last floor
        None
    }
}

fn iter_line(from: IVec2, to: IVec2) -> impl Iterator<Item=IVec2> {
    let range_x = if from.x <= to.x {
        from.x..=to.x
    } else {
        to.x..=from.x
    };

    let range_y = if from.y <= to.y {
        from.y..=to.y
    } else {
        to.y..=from.y
    };

    let iter_x = range_x.map(move |x| IVec2::new(x, to.y));
    let iter_y = range_y.map(move |y| IVec2::new(to.x, y));

    iter_x.chain(iter_y)
}

mod parsers {
    use glam::IVec2;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, i32, line_ending};
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;

    pub fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<IVec2>>> {
        separated_list1(line_ending, parse_line)(input)
    }

    fn parse_line(input: &str) -> nom::IResult<&str, Vec<IVec2>> {
        separated_list1(tag(" -> "), parse_ivec)(input)
    }

    fn parse_ivec(input: &str) -> nom::IResult<&str, IVec2> {
        let (remainder, (x, y)) = separated_pair(i32, char(','), i32)(input)?;
        let ivec = IVec2::new(x, y);
        Ok((remainder, ivec))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sandbox = Sandbox::from_input(input);

    let count = iter::repeat_with(|| sandbox.drop_sand(IVec2::new(500, 0)))
        .take_while(|settled| settled.is_some())
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sandbox = Sandbox::from_input(input)
        .with_fake_floor();

    let sand_spawn = IVec2::new(500, 0);
    let count = iter::repeat_with(|| sandbox.drop_sand(sand_spawn))
        .take_while(|settled| {
            if let Some(pos) = settled {
                *pos != sand_spawn
            } else {
                false
            }
        })
        .count();

    Some(count as u32 + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
