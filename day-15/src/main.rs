use std::fs;

struct Step<'a> {
    box_id: usize,
    label: &'a str,
    action: Action,
}

enum Action {
    Add(u8),
    Remove,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let steps = input.trim().split(',').map(|step| {
        if step.ends_with('-') {
            let label = step.trim_end_matches('-');
            Step {
                label,
                box_id: hash(label) as usize,
                action: Action::Remove,
            }
        } else {
            let mut parts = step.split('=');
            let label = parts.next().unwrap();
            let lens = parts.next().unwrap().parse::<u8>().unwrap();
            Step {
                label,
                box_id: hash(label) as usize,
                action: Action::Add(lens),
            }
        }
    });

    let mut boxes: [Vec<(&str, u8)>; 256] = std::array::from_fn(|_| vec![]);

    steps.for_each(|step| {
        let current_box = &mut boxes[step.box_id];

        match step.action {
            Action::Add(lens) => {
                if let Some(idx) = current_box
                    .iter()
                    .position(|(label, _lens)| **label == *step.label)
                {
                    current_box[idx].1 = lens
                } else {
                    current_box.push((step.label, lens));
                }
            }
            Action::Remove => current_box.retain(|(label, _lens)| **label != *step.label),
        }
    });

    let sum = boxes
        .into_iter()
        .enumerate()
        // .filter_map(|(idx, val)| Some((idx, val?)))
        .map(|(idx, current_box)| {
            let box_num = idx + 1;
            current_box
                .into_iter()
                .enumerate()
                .map(|(idx, (_label, lens))| {
                    let lens_slot = idx + 1;

                    lens_slot * lens as usize * box_num
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    dbg!(sum);
}

fn hash(input: &str) -> u8 {
    input.chars().fold(0u8, |accum, val| {
        let ascii = val as u8;
        ((((accum as u64) + (ascii as u64)) * 17) % 256) as u8
    })
}
