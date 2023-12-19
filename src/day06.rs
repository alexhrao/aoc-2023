use std::fs;

use super::Day;

pub struct Day06;

fn num_ways_to_win(race: (usize, usize)) -> usize {
    // Naive. Just look at all possible races
    let (time, dist) = race;
    (1..time).filter(|t| ((time - t) * t) > dist).count()
}

impl Day for Day06 {
    fn task1(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let mut lines = backing.lines();
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
        println!("{:?}", races);
        let total: usize = races.into_iter().map(num_ways_to_win).product();
        println!("{}", total);
    }
    fn task2(&self, file: &std::path::Path) {
        let backing = fs::read_to_string(file).unwrap();
        let mut lines = backing.lines();
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
        println!("{}", num_ways_to_win((time, dist)));
        // let races: Vec<(usize, usize)> = times
        //     .zip(dists)
        //     .map(|(t, d)| (t.parse().unwrap(), d.parse().unwrap()))
        //     .collect();
        // println!("{:?}", races);
        // let total: usize = races.into_iter().map(num_ways_to_win).product();
        // println!("{}", total);
    }
}
