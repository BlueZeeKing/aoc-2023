use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|number| number.parse::<u64>().unwrap());

    lines.next().unwrap();

    let maps: HashMap<_, _, RandomState> = HashMap::from_iter(
        lines
            .collect::<Vec<_>>()
            .split(|line| line.is_empty())
            .map(|map| {
                let mut title = map[0].trim_end_matches(" map:").split("-");

                let source = title.next().unwrap();
                title.next().unwrap();
                let dest = title.next().unwrap();

                let items = map[1..]
                    .iter()
                    .map(|line| {
                        let mut vals = line.split(" ").map(|val| val.parse::<u64>().unwrap());
                        let dest_start = vals.next().unwrap();
                        let source_start = vals.next().unwrap();
                        let len = vals.next().unwrap();
                        (
                            dest_start..=dest_start + len,
                            source_start..=source_start + len,
                        )
                    })
                    .collect::<Vec<_>>();

                (source, (dest, items))
            }),
    );

    let min = seeds
        .map(|seed| {
            let mut ty = "seed";
            let mut value = seed;

            while ty != "location" {
                let (name, items) = maps.get(ty).unwrap();

                value = items
                    .iter()
                    .find(|(_dest, source)| source.contains(&value))
                    .map(|(dest, source)| *dest.start() + (value - *source.start()))
                    .unwrap_or(value);

                ty = name;
            }

            value
        })
        .min()
        .unwrap();

    println!("{}", min);
}
