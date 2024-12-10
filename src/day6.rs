use aoc_runner_derive::aoc;

fn num_ways_to_win(race: (usize, usize)) -> usize {
    // Naive. Just look at all possible races
    let (time, dist) = race;
    (1..time).filter(|t| ((time - t) * t) > dist).count()
}

#[aoc(day6, part1)]
pub fn part1(s: &str) -> usize {
    let mut lines = s.lines();
    let times = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace();
    let dists = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace();
    let races: Vec<(usize, usize)> = times
        .zip(dists)
        .map(|(t, d)| (t.parse().unwrap(), d.parse().unwrap()))
        .collect();
    races.into_iter().map(num_ways_to_win).product()
}

#[aoc(day6, part2)]
pub fn part2(s: &str) -> usize {
    let mut lines = s.lines();
    let times = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace();
    let dists = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace();
    let time = times.collect::<Vec<_>>().join("").parse().unwrap();
    let dist = dists.collect::<Vec<_>>().join("").parse().unwrap();
    num_ways_to_win((time, dist))
}
