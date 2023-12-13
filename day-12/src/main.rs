use core::panic;
use std::{collections::HashMap, fs};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum = input
        .lines()
        .par_bridge()
        .map(|line| {
            let mut parts = line.split(" ");

            let mut row = parts
                .next()
                .unwrap()
                .chars()
                .map(|char| match char {
                    '.' => State::Operational,
                    '#' => State::Damaged,
                    '?' => State::Unknown,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();

            row.insert(0, State::Unknown);

            let amount = row.len() * 5;
            let mut row = row.into_iter().cycle().take(amount);
            row.next().unwrap();
            let row = row.collect::<Vec<_>>();

            let groups: Vec<usize> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|val| val.parse().unwrap())
                .collect();

            let amount = groups.len() * 5;
            let groups = groups.into_iter().cycle().take(amount).collect::<Vec<_>>();

            (row, groups)
        })
        .map(|(row, groups)| calculate_number_of_combos(0, 0, &groups, &row, &mut HashMap::new()))
        .sum::<usize>();

    println!("{sum}");
}

fn calculate_number_of_combos(
    group_idx: usize,
    row_idx: usize,
    groups: &[usize],
    row: &[State],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(res) = cache.get(&(group_idx, row_idx)) {
        return *res;
    }
    let mut number_combos = 0;

    for (idx, state) in row
        .iter()
        .enumerate()
        .filter(|(_, state)| **state != State::Operational)
    {
        if idx + groups[0] > row.len() {
            // If there isn't enough room for the next group stop
            break;
        }

        if row[idx..idx + groups[0]]
            .iter()
            .all(|val| *val == State::Unknown || *val == State::Damaged)
            && (idx + groups[0] >= row.len() || row[idx + groups[0]] != State::Damaged)
        {
            // If we can create a contigous group that is the correct length
            if groups.len() == 1 {
                // If there is only one contigous group left
                if row[idx + groups[0]..].iter().contains(&State::Damaged) {
                    // If there are more damaged springs later on so we either end processing
                    // because this can never work (if this is a guaranteed damage) or just try the next spot
                    if *state == State::Damaged {
                        break;
                    }
                    continue;
                }
                number_combos += 1;
                if *state == State::Damaged {
                    // If this is damaged, we know this is the last one from the check above and
                    // there can be no future matches
                    break;
                }
                continue;
            } else if idx + groups[0] + 1 >= row.len() {
                // If we have run out of springs in the row stop early
                break;
            }
            // Check the number of combanations of the rest of the row
            number_combos += calculate_number_of_combos(
                group_idx + 1,
                row_idx + idx + groups[0] + 1,
                &groups[1..],
                &row[idx + groups[0] + 1..],
                cache,
            );

            if *state == State::Damaged {
                break;
            }
        } else if *state == State::Damaged {
            // If this is a damaged piece and it couldn't be fit stop processing
            break;
        }
    }

    cache.insert((group_idx, row_idx), number_combos);

    number_combos
}
