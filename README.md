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
| [17](src/day17.rs) | ✅ | ✅ | I am amazed I got the first one. I'm working on the second one, but it just feels like more of the same. Last year, the graph algorithms questions kicked my butt; this year, my first attempt was a success! The general thought is take the input and make it into a graph `G`, where each node is a block from the original input, and the weight of each edge is the destination block's weight. Then, crete a new graph where each node is duplicated with counts; either 1, 2, or 3. 1 and 2 are allowed to connect forward (in the same direction) or orthogonally; 3 is only allowed orthogonally. Then run dijkstra's for each start direction (down or right), and see all the ways we made it to the bottom right (because you have counts 1, 2, 3; and certain directions!). Get the minimum one and there's your answer. Part 2 was similar, but it also has a _minimum_ distance to go... so when setting up the edges you need to be careful. The nice thing is that the system itself didn't need any changes - once you get the edges set right, the rest just works. |
| [18](src/day18.rs) | ✅ | ✅ | I was basically able to get this myself; I just needed some time off to see how to fix the area for part 2. Happy to be able to use Day 10 for part 1! |
| [19](src/day19.rs) | ✅ | ✅ | Part 1 was a fun trip down parsing lane, and I got to practice borrowing instead of cloning. Part 2 was insane but I'm so proud to say I got it right the first try! Of note, I never had to worry about "paring down" two ranges that were equal. That's what I was scared of, but thankfully I didn't have to! |
| [20](src/day20.rs) | ✅ | ✅ | The first part was really fun, figuring out how best to please the borrow checker, and in the process learning more about my previous (invalid!) assumptions. I love building a simulator! The second part was hard until it wasn't. It's a trick that I think may have been used in a similar fashion earlier, but basically you see when the inputs of the input to rx is satisfied individually; then multiply the cycles that each is satisfied to get the cycle overall the input will be satisfied. Really interesting! |
| [21](src/day21.rs) | ❌ | ❌ | Still working on it! |
| [22](src/day22.rs) | ✅ | ✅ | This was fun! Took me a few tries on the second part to finally understand that I need to be working my way up the bricks! One of the harder ones to wrap my mind around, but a really good puzzle! |