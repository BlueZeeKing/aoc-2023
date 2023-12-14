use std::{
    collections::{hash_map::RandomState, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let num_lines = input.lines().count();
    let mut number_of_cards = vec![1; num_lines];

    lines
        .enumerate()
        .filter_map(|(index, line)| {
            let mut parts = line.split(": ");
            let _start = parts.next().unwrap();
            let mut numbers = parts.next().unwrap().split('|');
            let winning = numbers.next().unwrap();
            let have = numbers.next().unwrap();

            let winning: HashSet<u32, RandomState> = HashSet::from_iter(
                winning
                    .trim()
                    .split(' ')
                    .filter_map(|name| name.parse::<u32>().ok()),
            );

            let have: HashSet<u32, RandomState> = HashSet::from_iter(
                have.trim()
                    .split(' ')
                    .filter_map(|name| name.parse::<u32>().ok()),
            );

            let count = winning.intersection(&have).into_iter().count();

            if count != 0 {
                Some((index, count))
            } else {
                None
            }
        })
        .for_each(|(index, score)| {
            let number_for_line = number_of_cards[index];
            for index in index + 1..=index + score {
                number_of_cards[index] += number_for_line;
            }
        });

    println!("{}", number_of_cards.iter().sum::<u32>())
}
