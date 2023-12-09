use core::panic;
use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let dirs = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next().unwrap();

    let items: HashMap<&str, [&str; 2], RandomState> = HashMap::from_iter(lines.map(|line| {
        let mut parts = line.split(" = ");

        let start = parts
            .next()
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')');

        let mut next_part = parts
            .next()
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ");

        let (left, right) = (next_part.next().unwrap(), next_part.next().unwrap());

        (start, [left, right])
    }));

    let mut current_nodes = items
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| *node)
        .collect::<Vec<_>>();

    let mut history = vec![Vec::<(&str, usize)>::new(); current_nodes.len()];
    let mut history_done = vec![None; current_nodes.len()];

    let mut current_dir_idx = 0;

    while history_done.iter().any(|val| val.is_none()) {
        let dir_idx: usize = match dirs[current_dir_idx] {
            'R' => 1,
            'L' => 0,
            _ => panic!(),
        };

        for (idx, current_node) in current_nodes.iter_mut().enumerate() {
            if history_done[idx].is_some() {
                continue;
            }

            if let Some((done_idx, _)) =
                history[idx]
                    .iter()
                    .enumerate()
                    .find(|(_, (current_node_check, dir_idx))| {
                        **current_node_check == **current_node && *dir_idx == current_dir_idx
                    })
            {
                history_done[idx] = Some(done_idx);
            } else {
                history[idx].push((*current_node, current_dir_idx));
            }

            *current_node = items.get(current_node).unwrap()[dir_idx];
        }

        current_dir_idx += 1;
        if current_dir_idx == dirs.len() {
            current_dir_idx = 0;
        }
    }

    let mut loops: Vec<_> = history
        .into_iter()
        .zip(history_done.into_iter())
        .map(|(history, loop_idx)| {
            let end_idx = history.iter().position(|val| val.0.ends_with('Z')).unwrap();
            let loop_len = (history.len() - end_idx) + (end_idx - loop_idx.unwrap());

            LoopIter {
                end_idx,
                loop_len,
                current_loop: 0,
            }
        })
        .collect();

    let current_val = 'outer: loop {
        let current_val = loops[0].next().unwrap();
        for loop_iter in &mut loops[1..] {
            while loop_iter.peek() < current_val {
                loop_iter.advance();
            }

            if loop_iter.peek() == current_val {
                loop_iter.advance();
            } else {
                continue 'outer;
            }
        }
        break current_val;
    };

    println!("{}", current_val);
}

struct LoopIter {
    end_idx: usize,
    loop_len: usize,
    current_loop: usize,
}

impl Iterator for LoopIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let val = Some(self.current_loop * self.loop_len + self.end_idx);
        self.current_loop += 1;
        val
    }
}

impl LoopIter {
    pub fn peek(&self) -> usize {
        self.current_loop * self.loop_len + self.end_idx
    }

    pub fn advance(&mut self) {
        self.current_loop += 1;
    }
}

