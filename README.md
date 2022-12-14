<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2022

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

Using the wonderful project template from here: https://github.com/fspoettel/advent-of-code-rust

## Journal
###### All times are relative to when I started, not when the task was opened.
### Day 01
#### Part 1: 00:40:46 --> Part 2: 00:09:12
- Spent quite a while figuring out how to iterate over each elf separately as a chunk.
- Later realized I could have split it into chunks with `\n\n` before parsing the input as lines.

### Day 02
#### Part 1: 00:24:57 --> Part 2: 00:07:12
- Went with simple match statements for the RPS logic.
- Quite happy with how the input iterating and parsing turned out.
- Had a bit of a brain fart at one point and lost like 10 mins >_<

### Day 03
#### Part 1: 00:24:07 --> Part 2: 00:21:09
- Spent most of the time figuring out HashSet intersections.
- This task took some fiddling to satisfy the borrow checker.

### Day 04
#### Part 1: 00:36:33 --> Part 2: 00:05:49
- Was fun to fiddle with iterators to find a clean solution.
- I'm on the fence about whether I should go full tryhard without caring about the quality of my solution.

### Day 05
#### Part 1: 01:11:42 --> Part 2: 00:23:33
- Today's inputs were super annoying to parse :c

### Day 06
#### Part 1: 00:10:09 --> Part 2: 00:00:28
- Simple as can be with the windows iterator :)

### Day 07
#### Part 1: 02:26:01 --> Part 2: 00:04:21
- I couldn't come up with any clever tricks, so I went ahead and parsed the whole file system into a proper tree.
- It was my first time making a tree in rust. It was hard, as expected :p

### Day 08
#### Part 1: 00:33:43 --> Part 2: 01:02:51
- Today I had to completely rewrite my solution for part 2

### Day 9
#### Part 1: 00:38:03 --> Part 2: 00:27:39
- Today's task was fun! Cool to see rope-like behaviour from such a simple system.
- Quite happy with the abstractions I came up with for today :)

### Day 10
#### Part 1: 01:12:11 --> Part 2: 00:29:58
- Another great task! Really enjoyed finding the output for part 2 :D
- Took some fiddling to get the generics right, but it works nicely in the end.

### Day 11
#### Part 1: 02:14:19 --> Part 2: 02:40:58
- Had to look for a hint for part 2 today. Turns out I was really close even before that, but I just didn't make the final connection.
- Did some really dumb mistakes in part 2, mostly related to integer overflow.

### Day 12
#### Part 1: 03:28:52 --> Part 2: 00:03:02
- Got distracted for like 2 hours in the middle of part 1.
- I guess this is the obligatory Dijkstra day :D

### Day 13
#### Part 1: 01:39:49 --> Part 2: 00:22:52
- I *really* like the solution I came up with today! Felt like I'm using Rust to it's fullest :D

### Day 14
#### Part 1: 02:54:11 --> Part 2: 00:15:27
- Got distracted a lot during part 1, so it's way longer than it should be.
- Used the `nom` parser crate today, and it worked out great! I'm definitely a fan now c:
