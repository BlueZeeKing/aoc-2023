use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .trim()
        .split_whitespace()
        .fold(String::new(), |acc, val| acc + val.trim())
        .parse::<i64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .trim()
        .split_whitespace()
        .fold(String::new(), |acc, val| acc + val.trim())
        .parse::<i64>()
        .unwrap();

    let (min, max) = quadratic_formula(-1, time, -distance);

    println!("{}", max - min + 1);
}

fn quadratic_formula(a: i64, b: i64, c: i64) -> (i64, i64) {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;

    let solution_1 = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let solution_2 = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);

    let (min, max) = (solution_1.min(solution_2), solution_1.max(solution_2));

    let min = if min == min.ceil() {
        min + 1.0
    } else {
        min.ceil()
    };

    let max = if max == max.floor() {
        max - 1.0
    } else {
        max.floor()
    };

    (min as i64, max as i64)
}
