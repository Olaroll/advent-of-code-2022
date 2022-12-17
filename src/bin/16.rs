use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use crate::parsers::parse_input;

#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    flow: u32,
    connections: Vec<Name>,
    timed_connections: RefCell<HashMap<Name, u32>>,
}

type Name = [u8; 2];

type Valves = HashMap<Name, Valve>;

mod parsers {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use anyhow::{Result};
    use nom::bytes::complete::{tag};
    use nom::character::complete::{u32 as u32_fn, line_ending, anychar, multispace0};
    use nom::combinator::{all_consuming};
    use nom::multi::separated_list1;
    use nom::{AsChar, InputLength, InputTakeAtPosition, Parser};
    use nom::branch::alt;
    use nom::error::ParseError;
    use nom::sequence::{delimited, preceded, tuple};
    use crate::{Name, Valve, Valves};

    pub(super) fn parse_input(input: &str) -> Result<Valves> {
        let (_, res) = parse_all(separated_list1(line_ending, parse_line))(input)
            .map_err(|e| e.map(|e| nom::error::Error::new(e.input.to_owned(), e.code)))?;

        let result = res.into_iter()
            .map(|(name, flow, connections)| {
                Valve {
                    name,
                    flow,
                    connections,
                    timed_connections: RefCell::new(HashMap::new()),
                }
            })
            .map(|valve| (valve.name, valve))
            .collect();

        Ok(result)
    }

    fn parse_all<I, O, E>(parser: impl Parser<I, O, E>) -> impl FnMut(I) -> nom::IResult<I, O, E>
    where I: InputLength + InputTakeAtPosition,
          <I as InputTakeAtPosition>::Item: AsChar + Clone,
          E: ParseError<I>,
    {
        all_consuming(delimited(multispace0, parser, multispace0))
    }

    fn parse_line(input: &str) -> nom::IResult<&str, (Name, u32, Vec<Name>)> {
        tuple((
            preceded(tag("Valve "), parse_key),
            preceded(tag(" has flow rate="), u32_fn),
            preceded(alt((tag("; tunnels lead to valves "), tag("; tunnel leads to valve "))), separated_list1(
                tag(", "),
                parse_key,
            ))
        ))(input)
    }

    fn parse_key(input: &str) -> nom::IResult<&str, Name> {
        let (input, o1) = anychar(input)?;
        anychar(input).map(|(i, o2)| (i, [o1 as u8, o2 as u8]))
    }
}

fn build_connections(valves: &Valves) {
    for (&start_name, start_valve) in valves.iter() {
        let mut connections = start_valve.timed_connections.borrow_mut();

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        seen.insert(start_name);
        queue.push_back((0_u32, start_name));

        while !queue.is_empty() {
            let (distance, name) = queue.pop_front().unwrap();
            let valve = valves.get(&name).unwrap();

            if valve.flow > 0 && distance != 0 {
                connections.insert(name, distance);
            }

            valve.connections.iter()
                .filter(|&&conn| seen.insert(conn))
                .for_each(|&conn| queue.push_back((distance + 1, conn)))
        }
    }
}

fn solve(valves: &Valves, name: Name, mut open: Rc<HashSet<Name>>, minutes: u32, mut flow: u32, elephant: bool) -> u32 {
    if minutes == 0 {
        return 0;
    }

    let this = valves.get(&name).unwrap();

    if this.flow > 0 {
        let mut temp = (*open).clone();
        temp.insert(this.name);
        open = Rc::new(temp);
        flow += this.flow;
    }

    let mut max = flow * minutes;

    let res = this.timed_connections.borrow().iter()
        .filter(|(next_name, _)| !open.contains(*next_name))
        .map(|(n, dist)| (n, dist + 1)) // Time to turn on valve
        .filter(|(_, distance)| *distance < minutes)
        .map(|(&next_name, distance)| {
            distance * flow + solve(
                valves,
                next_name,
                open.clone(),
                minutes - distance,
                flow,
                elephant,
            )
        })
        .max()
        .unwrap_or(0);

    let res = res.max(max + if elephant {
        solve(
            valves,
            [b'A', b'A'],
            open,
            26,
            0,
            false,
        )
    } else { 0 });

    max = max.max(res);

    max
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = parse_input(input).expect("couldn't parse input");

    build_connections(&valves);

    let answer = solve(&valves, [b'A', b'A'], Rc::new(HashSet::new()), 30, 0, false);

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let valves = parse_input(input).expect("couldn't parse input");

    build_connections(&valves);

    // let a = valves.

    let answer = solve(&valves, [b'A', b'A'], Rc::new(HashSet::new()), 26, 0, true);

    Some(answer)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
