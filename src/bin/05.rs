use itertools::Itertools;

fn parse_state(state: &str) -> Vec<Vec<char>> {
    let rows: Vec<_> = state.lines().rev().skip(1)
        .map(|line| parse_line(line))
        .collect();

    // transpose
    let cols: Vec<Vec<_>> = (0..rows[0].len()).map(|x| rows
        .iter()
        .filter_map(|row| row[x])
        .collect()
    ).collect();

    cols
}

fn parse_line(line: &str) -> Vec<Option<char>> {
    assert!(line.is_ascii());
    let line = line.as_bytes();
    let mut out = Vec::with_capacity(line.len()/4);

    let mut i = 1;
    while i < line.len() {
        out.push(line[i].is_ascii_alphabetic().then(|| line[i] as char));
        i += 4;
    }

    out
}

fn print_state(state: &[Vec<char>]) {
    let s = state.iter()
        .map(|col| col.iter().join(""))
        .join("\n");

    println!("{s}\n");
}

fn solve(
    input: &str,
    mut f: impl FnMut(&mut [Vec<char>], usize, usize, usize)
) -> Option<String>
{
    let (state, instructions) = input.split_once("\n\n")
        .or_else(|| input.split_once("\r\n\r\n"))?;

    let mut state = parse_state(state);

    instructions.lines()
        // .inspect(|&line| println!("{line}"))
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|segments| (segments[1], segments[3], segments[5]))
        .filter_map(|(a, b, c)| Some((
            a.parse::<usize>().ok()?,
            b.parse::<usize>().ok()?,
            c.parse::<usize>().ok()?,
        )))
        .map(|(a, b, c)| (a, b-1, c-1))
        .for_each(|(n, from, to)| f(&mut state, n, from, to));
    // print_state(&state);

    let res = state.iter()
        .map(|col| col.last().unwrap())
        .collect();

    Some(res)
}

pub fn part_one(input: &str) -> Option<String> {
   solve(input, |state, n, from, to| {
       (0..n).for_each(|_| {
           let temp = state[from].pop().unwrap();
           state[to].push(temp)
       })
   })
}

pub fn part_two(input: &str) -> Option<String> {
    solve(input, |state, n, from, to| {
        // print_state(state);
        let len = state[from].len();
        let mut temp: Vec<_> = state[from].drain(len-n..).collect();
        state[to].append(&mut temp);
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
