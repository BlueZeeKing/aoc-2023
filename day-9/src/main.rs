use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let sum = lines
        .map(|vals| {
            let history = vals
                .split(" ")
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            process_history(&history)
        })
        .sum::<i64>();

    println!("{}", sum);
}

fn process_history(history: &[i64]) -> i64 {
    if history.iter().all(|val| *val == 0) {
        return 0;
    }

    let new_history: Vec<_> = history
        .iter()
        .take(history.len() - 1)
        .enumerate()
        .map(|(idx, first)| {
            let next = history[idx + 1];
            next - first
        })
        .collect();

    history[0] - process_history(&new_history)
}
