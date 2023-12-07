use std::{cmp::Ordering, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Card {
    Ace,
    King,
    Queen,
    Joker,
    Number(u8),
}

impl Card {
    fn to_number_ord(&self) -> u8 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Joker => 1,
            Card::Number(n) => *n,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_number_ord().partial_cmp(&other.to_number_ord())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_number_ord().cmp(&other.to_number_ord())
    }
}

#[derive(Debug)]
pub struct CardError(char);

impl Display for CardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not create card from char: {}", self.0)
    }
}

#[derive(Debug)]
pub struct StringError;

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not create hand from string")
    }
}

impl TryFrom<char> for Card {
    type Error = CardError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Joker),
            'T' => Ok(Card::Number(10)),
            '9' => Ok(Card::Number(9)),
            '8' => Ok(Card::Number(8)),
            '7' => Ok(Card::Number(7)),
            '6' => Ok(Card::Number(6)),
            '5' => Ok(Card::Number(5)),
            '4' => Ok(Card::Number(4)),
            '3' => Ok(Card::Number(3)),
            '2' => Ok(Card::Number(2)),
            n => Err(CardError(n)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn to_num_ord(&self) -> u8 {
        match self {
            HandType::FiveOfKind => 6,
            HandType::FourOfKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_num_ord().partial_cmp(&other.to_num_ord())
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_num_ord().cmp(&other.to_num_ord())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hand {
    hand: [Card; 5],
    hand_type: HandType,
    bid: u64,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand = self.hand_type.cmp(&other.hand_type);

        match hand {
            Ordering::Equal => {
                for (mine, other) in self.hand.iter().zip(other.hand.iter()) {
                    match mine.cmp(other) {
                        Ordering::Equal => continue,
                        n => return Some(n),
                    }
                }
                Some(Ordering::Equal)
            }
            n => Some(n),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    pub fn get_bid(&self) -> u64 {
        self.bid
    }

    fn calc_type(cards: [Card; 5], bid: u64) -> Self {
        let mut amounts = Vec::new();

        let num_jokers = cards
            .into_iter()
            .filter(|card| *card == Card::Joker)
            .count() as u64;

        for card in cards.into_iter().filter(|card| *card != Card::Joker) {
            if let Some((_, amount)) = amounts
                .iter_mut()
                .find(|(check_card, _)| card == *check_card)
            {
                *amount += 1;
            } else {
                amounts.push((card, 1u64));
            }
        }

        amounts.sort_by_key(|val| val.1);
        amounts.reverse();

        if amounts.len() > 0 {
            amounts[0].1 += num_jokers;
        } else {
            amounts.push((Card::Joker, num_jokers));
        }

        let hand_type = if amounts[0].1 == 5 {
            HandType::FiveOfKind
        } else if amounts[0].1 == 4 {
            HandType::FourOfKind
        } else if amounts[0].1 == 3 && amounts[1].1 == 2 {
            HandType::FullHouse
        } else if amounts[0].1 == 3 {
            HandType::ThreeOfKind
        } else if amounts[0].1 == 2 && amounts[1].1 == 2 {
            HandType::TwoPair
        } else if amounts[0].1 == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        Self {
            hand: cards,
            hand_type,
            bid,
        }
    }
}

impl FromStr for Hand {
    type Err = StringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sides = s.split(" ");

        let mut cards_iter = sides
            .next()
            .unwrap()
            .chars()
            .map(|card| Card::try_from(card).unwrap());

        let bid = sides.next().unwrap().parse::<u64>().unwrap();

        Ok(Self::calc_type(
            [
                cards_iter.next().unwrap(),
                cards_iter.next().unwrap(),
                cards_iter.next().unwrap(),
                cards_iter.next().unwrap(),
                cards_iter.next().unwrap(),
            ],
            bid,
        ))
    }
}
