use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Card {
    winners: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    pub fn num_winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&n| self.winners.contains(n))
            .count()
    }
    pub fn score(&self) -> usize {
        let winners = u32::try_from(self.num_winning()).unwrap();
        if winners == 0 {
            0
        } else {
            2_usize.pow(winners - 1)
        }
    }
}

fn to_vec(list: &str) -> Vec<usize> {
    list.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(':').skip(1);
        let nums = nums.next().unwrap();
        let mut nums = nums.split('|');
        let winners = to_vec(nums.next().unwrap());
        let numbers = to_vec(nums.next().unwrap());

        Ok(Card { winners, numbers })
    }
}

#[aoc_generator(day4)]
pub fn gen(input: &str) -> Vec<Card> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day4, part1)]
pub fn part1(cards: &[Card]) -> usize {
    cards.iter().map(Card::score).sum()
}

#[aoc(day4, part2)]
pub fn part2(cards: &[Card]) -> usize {
    let mut copies = vec![1; cards.len()];
    for (c, card) in cards.iter().enumerate() {
        let matches = card.num_winning();
        for _ in 0..copies[c] {
            for c in copies.iter_mut().take(c + matches + 1).skip(c + 1) {
                *c += 1;
            }
        }
    }
    copies.iter().sum()
}
