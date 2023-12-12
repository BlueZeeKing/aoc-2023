use core::panic;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum State {
    Operational,
    Damaged,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum = input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");

            let row = parts
                .next()
                .unwrap()
                .chars()
                .map(|char| match char {
                    '.' => Some(State::Operational),
                    '#' => Some(State::Damaged),
                    '?' => None,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();

            let groups: Vec<usize> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|val| val.parse().unwrap())
                .collect();

            (row, groups)
        })
        .enumerate()
        .map(|(idx, (row, groups))| {
            let damaged = row
                .iter()
                .enumerate()
                .filter(|(_, value)| matches!(value, Some(State::Damaged)))
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>();
            let num_damaged = damaged.len();
            let num_should_damaged: usize = groups.iter().sum();
            let num_remaining = num_should_damaged - num_damaged;

            if num_remaining == 0 {
                return 1;
            }

            let values = row
                .iter()
                .enumerate()
                .filter(|(_, value)| value.is_none())
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>();

            let permutations = permutations(&values, num_remaining);
            let mut num_options = 0;

            for mut permutation in permutations {
                permutation.reserve(num_damaged);
                for damage in &damaged {
                    permutation.push(*damage);
                }

                permutation.sort_unstable();

                let mut groups_check = vec![];
                let mut idxs = vec![];
                for idx in permutation.iter() {
                    if idxs.len() == 0 {
                        idxs.push(idx);
                    } else if **idxs.last().unwrap() + 1 != *idx {
                        groups_check.push(idxs.len());
                        idxs = vec![idx];
                    } else {
                        idxs.push(idx);
                    }
                }
                groups_check.push(idxs.len());
                if groups_check == groups {
                    num_options += 1;
                }
            }

            num_options
        })
        .sum::<usize>();

    println!("{sum}");
}

fn permutations(values: &[usize], len: usize) -> Vec<Vec<usize>> {
    let mut res = vec![];
    for (idx, value) in values.iter().enumerate() {
        let value = *value;
        if len > 1 {
            for mut permutation in permutations(&values[idx + 1..], len - 1) {
                permutation.insert(0, value);
                res.push(permutation);
            }
        } else {
            res.push(vec![value]);
        }
    }
    res
}
