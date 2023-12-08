use std::{fs, str::FromStr};

use super::Day;

pub struct Day4;

#[derive(Debug, Clone)]
struct Card {
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
        let winners = self.num_winning() as u32;
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

impl Day for Day4 {
    fn task1(&self, file: &std::path::Path) {
        let cards: Vec<Card> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let total: usize = cards.iter().map(Card::score).sum();
        println!("{}", total);
    }
    fn task2(&self, file: &std::path::Path) {
        let cards: Vec<Card> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let mut copies = vec![1; cards.len()];
        for (c, card) in cards.iter().enumerate() {
            let matches = card.num_winning();
            for _ in 0..copies[c] {
                for c in copies.iter_mut().take(c + matches + 1).skip(c + 1) {
                    *c += 1;
                }
            }
        }
        println!("{:?}", copies.iter().sum::<usize>());
    }
}
