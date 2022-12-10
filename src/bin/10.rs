use itertools::Itertools;

#[derive(Debug)]
struct CPU<I> {
    x: i32,
    cycle: u32,

    cmds: I,
    processing: Option<(Cmd, u32)>,
}

#[derive(Debug, Copy, Clone)]
enum Cmd {
    NoOp,
    AddX(i32),
}

impl<I> CPU<I> {
    fn new<T>(cmds: T) -> Self
    where T: IntoIterator<Item=Cmd, IntoIter=I>
    {
        Self {
            x: 1,
            cycle: 0,
            cmds: cmds.into_iter(),
            processing: None,
        }
    }
}

impl<I> Iterator for CPU<I>
where I: Iterator<Item=Cmd>
{
    type Item = (u32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;

        if let Some((_, until)) = self.processing {
            if self.cycle >= until {
                self.processing.take().unwrap().0.execute(self)
            }
        }

        if let None = self.processing {
            let cmd = self.cmds.next()?;
            self.processing = Some((cmd, self.cycle + cmd.time()));
        }

        Some((self.cycle, self.x))
    }
}

impl Cmd {
    fn execute<I>(self, cpu: &mut CPU<I>) {
        match self {
            Cmd::NoOp => {}
            Cmd::AddX(n) => cpu.x += n,
        }
    }

    fn time(&self) -> u32 {
        match self {
            Cmd::NoOp => 1,
            Cmd::AddX(_) => 2,
        }
    }
}

impl TryFrom<&str> for Cmd {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split_whitespace();

        let err = "couldn't parse the command!";

        match iter.next().ok_or(err)? {
            "noop" => Ok(Cmd::NoOp),
            "addx" => Ok(Cmd::AddX(iter.next().ok_or(err)?.parse().map_err(|_| err)?)),
            _ => Err(err)
        }
    }
}

fn make_cmds(input: &str) -> impl Iterator<Item=Cmd> + '_ {
    input.lines()
        .filter_map(|line| Cmd::try_from(line).ok())
}

pub fn part_one(input: &str) -> Option<u32> {
    CPU::new(make_cmds(input))
        .filter(|(count, x)| *count >= 20 && (*count - 20) % 40 == 0)
        .map(|(count, x)| count * x as u32)
        .sum1()
}

pub fn part_two(input: &str) -> Option<String> {
    let res = CPU::new(make_cmds(input))
        .map(|(count, x)| if ((count as i32 % 40) - x - 1).abs() <= 1 { '#' } else { '.' })
        .chunks(40).into_iter()
        .map(|chunk| chunk.format(""))
        .join("\n");

    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some("##..##..##..##..##..##..##..##..##..##..\n\
                                           ###...###...###...###...###...###...###.\n\
                                           ####....####....####....####....####....\n\
                                           #####.....#####.....#####.....#####.....\n\
                                           ######......######......######......###.\n\
                                           #######.......#######.......#######.....".into()));
    }
}
