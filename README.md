# Advent of Code 2023

This is my repository for AoC 2023. I'm in the process of learning Rust, so I'm using this as an opportunity to practice my skills!

The Table of Contents has been (lovingly) ripped from @savbell's [great AoC repository](https://github.com/savbell/advent-of-code-one-liners/tree/master).

## Progress Tracking

| Status |        Description        |
|--------|---------------------------|
|   ❌   | Problem not attempted yet |
|   ✅   | Completed                 |

## 2023 Solutions

|        Day         | Part 1 | Part 2 |            Commentary            |
|--------------------|--------|--------|----------------------------------|
| [1](src/day1.rs)   |   ✅   |   ✅   |                                  |
| [2](src/day2.rs)   |   ✅   |   ✅   |                                  |
| [3](src/day3.rs)   |   ✅   |   ✅   |                                  |
| [4](src/day4.rs)   |   ✅   |   ✅   |                                  |
| [5](src/day5.rs)   |   ✅   |   ✅   |                                  |
| [6](src/day6.rs)   |   ✅   |   ✅   |                                  |
| [7](src/day6.rs)   |   ✅   |   ✅   |                                  |
| [8](src/day6.rs)   |   ✅   |   ✅   |                                  |
| [9](src/day6.rs)   |   ✅   |   ✅   |                                  |
| [10](src/day10.rs) |   ✅   |   ✅   | [Commentary](#day-10-commentary) |
| [11](src/day11.rs) |   ✅   |   ✅   |                                  |
| [12](src/day12.rs) |   ✅   |   ✅   | [Commentary](#day-12-commentary) |
| [13](src/day13.rs) |   ✅   |   ✅   |                                  |
| [14](src/day14.rs) |   ✅   |   ✅   |                                  |
| [15](src/day15.rs) |   ✅   |   ✅   |                                  |
| [16](src/day16.rs) |   ✅   |   ✅   |                                  |
| [17](src/day17.rs) |   ✅   |   ✅   | [Commentary](#day-17-commentary) |
| [18](src/day18.rs) |   ✅   |   ✅   | [Commentary](#day-18-commentary) |
| [19](src/day19.rs) |   ✅   |   ✅   | [Commentary](#day-19-commentary) |
| [20](src/day20.rs) |   ✅   |   ✅   | [Commentary](#day-20-commentary) |
| [21](src/day21.rs) |   ✅   |   ✅   | [Commentary](#day-21-commentary) |
| [22](src/day22.rs) |   ✅   |   ✅   | [Commentary](#day-22-commentary) |
| [23](src/day23.rs) |   ✅   |   ✅   | [Commentary](#day-23-commentary) |
| [24](src/day24.rs) |   ✅   |   ✅   | [Commentary](#day-24-commentary) |
| [25](src/day25.rs) |   ✅   |   ❌   |                                  |

### Day 10 Commentary

Some good memes came out of this one

### Day 12 Commentary

This is insane, at least part 2

### Day 17 Commentary

I am amazed I got the first one. I'm working on the second one, but it just
feels like more of the same. Last year, the graph algorithms questions kicked my
butt; this year, my first attempt was a success! The general thought is take the
input and make it into a graph `G`, where each node is a block from the original
input, and the weight of each edge is the destination block's weight. Then,
create a new graph where each node is duplicated with counts; either 1, 2, or 3.
1 and 2 are allowed to connect forward (in the same direction) or orthogonally;
3 is only allowed orthogonally. Then run dijkstra's for each start direction
(down or right), and see all the ways we made it to the bottom right (because
you have counts 1, 2, 3; and certain directions!). Get the minimum one and
there's your answer. Part 2 was similar, but it also has a _minimum_ distance to
go... so when setting up the edges you need to be careful. The nice thing is that
the system itself didn't need any changes - once you get the edges set right,
the rest just works.

### Day 18 Commentary

I was basically able to get this myself; I just needed some time off to see how
to fix the area for part 2. Happy to be able to use Day 10 for part 1!

### Day 19 Commentary

Part 1 was a fun trip down parsing lane, and I got to practice borrowing instead
of cloning. Part 2 was insane but I'm so proud to say I got it right the first
try. Of note, I never had to worry about "paring down" two ranges that were
equal. That's what I was scared of, but thankfully I didn't have to

### Day 20 Commentary

The first part was really fun, figuring out how best to please the borrow
checker, and in the process learning more about my previous (invalid!)
assumptions. I love building a simulator! The second part was hard until it
wasn't. It's a trick that I think may have been used in a similar fashion
earlier, but basically you see when the inputs of the input to rx is satisfied
individually; then multiply the cycles that each is satisfied to get the cycle
overall the input will be satisfied. Really interesting!

## Day 21 Commentary

The first part was pretty straight forward, which honestly is what gave me
pause... I just knew that it would be too easy to just do it 64 times! The
second part is beyond insane, but I was finally able to solve it after I gave up
and looked up the solution. Following
[this solution writeup](https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21)
was especially helpful

### Day 22 Commentary

This was fun! Took me a few tries on the second part to finally understand that
I need to be working my way up the bricks. One of the harder ones to wrap my
mind around, but a really good puzzle!

### Day 23 Commentary

My solution is **not** optimal. The part 2 solution took a whole 12 minutes to
run! But it's done so I'm moving on.

### Day 24 Commentary

Part 1 was really fun, as I've said before I love simulations where you can
actually just run through it. The second part... honestly I'm kind of struggling
to figure out what it's really asking. I mean it makes sense, but golly I don't
think I even really know how to find it for, like, 2 or 3 hailstones, much less
hundreds. I finally was able to get it thanks to the code from
[@ash42](https://github.com/ash42/adventofcode/blob/main/adventofcode2023/src/nl/michielgraat/adventofcode2023/day24/Day24.java),
which does a wonderful job of walking you through the necessary system of
equations.
