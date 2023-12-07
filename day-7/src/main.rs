use std::{fs, str::FromStr};

use day_7::Hand;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut hands = input
        .lines()
        .map(|line| Hand::from_str(line).unwrap())
        .collect::<Vec<_>>();

    hands.sort();

    let sum = hands
        .into_iter()
        .enumerate()
        .map(|(index, val)| (index as u64 + 1) * val.get_bid())
        .sum::<u64>();

    println!("{}", sum);
}



