use std::fs;

use phf::phf_map;

static PATTERNS: phf::Map<&str, u32> = phf_map! {
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let sum: u32 = lines
        .map(|val| {
            let (mut first_index, mut first) = (i32::MAX, 0);

            let (mut last_index, mut last) = (i32::MIN, 0);

            for (index, number) in PATTERNS
                .entries()
                .map(|(key, value)| (*key, *value))
                .map(|(key, value)| {
                    val.match_indices(key)
                        .map(move |(position, _)| (position, value))
                })
                .flatten()
            {
                if (index as i32) < first_index {
                    first_index = index as i32;
                    first = number;
                }
                if (index as i32) > last_index {
                    last_index = index as i32;
                    last = number;
                }
            }
            first * 10 + last
        })
        .sum();

    println!("{}", sum);
}

