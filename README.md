# Advent of Code 2023

This is my repository for AoC 2023. I'm in the process of learning Rust, so I'm using this as an opportunity to practice my skills!

The Table of Contents has been (lovingly) ripped from @savbell's [great AoC repository](https://github.com/savbell/advent-of-code-one-liners/tree/master).


## Progress Tracking
| Status | Description |
| ------ | ----------- |
| ❌     | Problem not attempted yet |
| ✅     | Completed |

## 2023 Solutions
| Day              | Part 1 | Part 2 | Commentary |
|------------------|--------|--------|------------|
| [01](src/day01.rs) | ✅ | ✅ |  |
| [02](src/day02.rs) | ✅ | ✅ |  |
| [03](src/day03.rs) | ✅ | ✅ |  |
| [04](src/day04.rs) | ✅ | ✅ |  |
| [05](src/day05.rs) | ✅ | ✅ |  |
| [06](src/day06.rs) | ✅ | ✅ |  |
| [07](src/day07.rs) | ✅ | ✅ |  |
| [08](src/day08.rs) | ✅ | ✅ |  |
| [09](src/day09.rs) | ✅ | ✅ |  |
| [10](src/day10.rs) | ✅ | ✅ | Some good memes came out of this one |
| [11](src/day11.rs) | ✅ | ✅ |  |
| [12](src/day12.rs) | ✅ | ✅ | This is insane, at least part 2 |
| [13](src/day13.rs) | ✅ | ✅ |  |
| [14](src/day14.rs) | ✅ | ✅ |  |
| [15](src/day15.rs) | ✅ | ✅ |  |
| [16](src/day16.rs) | ✅ | ✅ |  |
| [17](src/day17.rs) | ❌ | ❌ | Damn this one is hard |
| [18](src/day18.rs) | ✅ | ✅ | I was basically able to get this myself; I just needed some time off to see how to fix the area for part 2. Happy to be able to use Day 10 for part 1! |
| [19](src/day19.rs) | ✅ | ✅ | Part 1 was a fun trip down parsing lane, and I got to practice borrowing instead of cloning. Part 2 was insane but I'm so proud to say I got it right the first try! Of note, I never had to worry about "paring down" two ranges that were equal. That's what I was scared of, but thankfully I didn't have to! |
| [20](src/day20.rs) | ✅ | ✅ | The first part was really fun, figuring out how best to please the borrow checker, and in the process learning more about my previous (invalid!) assumptions. I love building a simulator! The second part was hard until it wasn't. It's a trick that I think may have been used in a similar fashion earlier, but basically you see when the inputs of the input to rx is satisfied individually; then multiply the cycles that each is satisfied to get the cycle overall the input will be satisfied. Really interesting! |