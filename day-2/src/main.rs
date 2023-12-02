use std::fs;

use phf::phf_map;

static MAP: phf::Map<&str, u32> = phf_map! {
    "red" => 0,
    "green" => 1,
    "blue" => 2,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut parts = line.trim_start_matches("Game ").split(":");
            let _id: u32 = parts.next().unwrap().parse().unwrap();
            let dice_counts = parts.next().unwrap().split(';').map(|round| {
                let dice_counts = round.split(',').map(|dice| {
                    let mut parts = dice.trim().split(' ');
                    let count: u32 = parts.next().unwrap().trim().parse().unwrap();
                    let color = parts.next().unwrap();
                    let id = MAP.get(color).unwrap();
                    (*id, count)
                });

                let mut combined = [0; 3];

                for (index, count) in dice_counts {
                    combined[index as usize] += count;
                }

                combined
            });

            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;

            for counts in dice_counts {
                if counts[0] > max_red {
                    max_red = counts[0];
                }
                if counts[1] > max_green {
                    max_green = counts[1];
                }
                if counts[2] > max_blue {
                    max_blue = counts[2];
                }
            }

            max_red * max_green * max_blue
        })
        .sum();

    println!("{}", sum);
}

