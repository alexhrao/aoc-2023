use std::{collections::HashMap, fs, str::FromStr};

use super::Day;
pub struct Day07;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum WildCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for WildCard {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WildHand {
    cards: [WildCard; 5],
    bid: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Hand {
    pub fn get_strength(&self) -> HandStrength {
        let mut cards = self.cards;
        cards.sort();
        // Naive?
        let mut buckets: HashMap<_, usize> = HashMap::new();
        for card in cards {
            *buckets.entry(card).or_insert(0) += 1;
        }
        let buckets = buckets;
        if buckets.len() == 1 {
            return HandStrength::FiveKind;
        } else if buckets.len() == 5 {
            // All different
            return HandStrength::HighCard;
        }
        let max_count = *buckets.values().max().unwrap();
        if max_count == 4 {
            HandStrength::FourKind
        } else if max_count == 2 && buckets.len() == 4 {
            HandStrength::OnePair
        } else if max_count == 2 && buckets.len() == 3 {
            HandStrength::TwoPair
        } else if max_count == 3 && buckets.len() == 2 {
            HandStrength::FullHouse
        } else {
            HandStrength::ThreeKind
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        if self.cards.eq(&other.cards) {
            return std::cmp::Ordering::Equal;
        }
        let cmp = self.get_strength().cmp(&other.get_strength());
        if cmp != std::cmp::Ordering::Equal {
            cmp
        } else {
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                let cmp = c1.cmp(c2);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let mut cards = [Card::Two; 5];
        for (i, c) in split.next().unwrap().chars().enumerate() {
            cards[i] = c.into();
        }
        let bid = split.next().unwrap().parse().unwrap();
        Ok(Hand { bid, cards })
    }
}

fn score(cards: &[WildCard]) -> HandStrength {
    if cards.is_empty() {
        panic!("Must be given at least one card");
    }
    if cards.len() == 1 {
        HandStrength::HighCard
    } else if cards.len() == 2 {
        if cards[0] == cards[1] {
            HandStrength::OnePair
        } else {
            HandStrength::HighCard
        }
    } else if cards.len() == 3 {
        if cards[0] == cards[1] && cards[1] == cards[2] {
            HandStrength::ThreeKind
        } else if cards[0] == cards[1] || cards[0] == cards[2] || cards[1] == cards[2] {
            HandStrength::OnePair
        } else {
            HandStrength::HighCard
        }
    } else {
        let mut buckets: HashMap<_, usize> = HashMap::new();
        for card in cards {
            *buckets.entry(card).or_insert(0) += 1;
        }
        let buckets = buckets;
        if buckets.len() == cards.len() {
            return HandStrength::HighCard;
        } else if buckets.len() == 1 {
            return match cards.len() {
                1 => HandStrength::HighCard,
                2 => HandStrength::OnePair,
                3 => HandStrength::ThreeKind,
                4 => HandStrength::FourKind,
                5 => HandStrength::FiveKind,
                _ => unreachable!(),
            };
        }
        let max_count = *buckets.values().max().unwrap();
        if max_count == 4 {
            HandStrength::FourKind
        } else if max_count == 2 {
            if buckets.values().filter(|&&c| c == 2).count() == 1 {
                HandStrength::OnePair
            } else {
                HandStrength::TwoPair
            }
        } else if max_count == 3 {
            if buckets.values().any(|&c| c == 2) {
                HandStrength::FullHouse
            } else {
                HandStrength::ThreeKind
            }
        } else {
            unreachable!()
        }
    }
}

impl WildHand {
    pub fn get_strength(&self) -> HandStrength {
        let mut cards = self.cards;
        cards.sort();
        // Naive?
        let num_jokers = cards.iter().filter(|&&c| c == WildCard::Joker).count();
        if num_jokers == 0 {
            // No jokers, just as normal
            let mut buckets: HashMap<_, usize> = HashMap::new();
            for card in cards {
                *buckets.entry(card).or_insert(0) += 1;
            }
            let buckets = buckets;
            if buckets.len() == 1 {
                return HandStrength::FiveKind;
            } else if buckets.len() == 5 {
                // All different
                return HandStrength::HighCard;
            }
            let max_count = *buckets.values().max().unwrap();
            if max_count == 4 {
                HandStrength::FourKind
            } else if max_count == 2 && buckets.len() == 4 {
                HandStrength::OnePair
            } else if max_count == 2 && buckets.len() == 3 {
                HandStrength::TwoPair
            } else if max_count == 3 && buckets.len() == 2 {
                HandStrength::FullHouse
            } else {
                HandStrength::ThreeKind
            }
        } else if num_jokers == 5 {
            HandStrength::FiveKind
        } else {
            // See what I can make with the cards that aren't jokers, then depending
            // on the number of jokers, make the right move
            // Possibilities:
            // * Non-jokers make 4 of a kind, so now I can get FiveKind
            // * Non-jokers make 3 of a kind. If I have 1 joker, can make 4; otherwise, can make 5
            // * Non-jokers make two pair. I'll have 1 joker, I can make a FullHouse
            // * Non-jokers make one pair. If I have 3 jokers, can make 5; if I have 2 jokers, can make 4; if I have 1 joker, can make 3
            // * Non-jokers make high card. If I have 4 jokers, I can get FiveKind; if I have 3 jokers, I can make FourKind; if I have 2 jokers, I can make 3Kind; if I have 1 joker, I can make a pair
            use HandStrength as HS;
            let normals: Vec<_> = self
                .cards
                .iter()
                .filter_map(|&wc| {
                    if wc != WildCard::Joker {
                        Some(wc)
                    } else {
                        None
                    }
                })
                .collect();
            let strength = score(&normals);
            match strength {
                HS::FiveKind | HS::FourKind => HS::FiveKind,
                HS::FullHouse => HS::FullHouse,
                HS::ThreeKind => {
                    if num_jokers == 1 {
                        HS::FourKind
                    } else {
                        HS::FiveKind
                    }
                }
                HS::TwoPair => HS::FullHouse,
                HS::OnePair => {
                    if num_jokers == 3 {
                        HS::FiveKind
                    } else if num_jokers == 2 {
                        HS::FourKind
                    } else {
                        HS::ThreeKind
                    }
                }
                HS::HighCard => {
                    if num_jokers == 4 {
                        HS::FiveKind
                    } else if num_jokers == 3 {
                        HS::FourKind
                    } else if num_jokers == 2 {
                        HS::ThreeKind
                    } else {
                        HS::OnePair
                    }
                }
            }
        }
    }
}

impl FromStr for WildHand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let mut cards = [WildCard::Joker; 5];
        for (i, c) in split.next().unwrap().chars().enumerate() {
            cards[i] = c.into();
        }
        let bid = split.next().unwrap().parse().unwrap();
        Ok(WildHand { bid, cards })
    }
}

impl Ord for WildHand {
    fn cmp(&self, other: &WildHand) -> std::cmp::Ordering {
        if self.cards.eq(&other.cards) {
            return std::cmp::Ordering::Equal;
        }
        let cmp = self.get_strength().cmp(&other.get_strength());
        if cmp != std::cmp::Ordering::Equal {
            cmp
        } else {
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                let cmp = c1.cmp(c2);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for WildHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Day for Day07 {
    fn task1(&self, file: &std::path::Path) {
        let mut hands: Vec<Hand> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        hands.sort();

        let total = hands
            .iter()
            .enumerate()
            .map(|(i, c)| (i + 1) * c.bid)
            .sum::<usize>();
        println!("{}", total);
    }
    fn task2(&self, file: &std::path::Path) {
        let mut hands: Vec<WildHand> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        hands.sort();

        let total = hands
            .iter()
            .enumerate()
            .map(|(i, c)| (i + 1) * c.bid)
            .sum::<usize>();
        println!("{}", total);
    }
}
