use itertools::Itertools;
use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
    ops::RangeInclusive,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let chunks = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(" ")
        .map(|number| number.parse::<u64>().unwrap())
        .chunks(2);

    let seeds: Interval = chunks
        .into_iter()
        .map(|mut chunk| {
            let first = chunk.next().unwrap();
            let len = chunk.next().unwrap();
            Range::new(first..=first + len - 1)
        })
        .into();

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

                let items: Interval = map[1..]
                    .iter()
                    .map(|line| {
                        let mut vals = line.split(" ").map(|val| val.parse::<u64>().unwrap());
                        let dest_start = vals.next().unwrap();
                        let source_start = vals.next().unwrap();
                        let offset = dest_start as i64 - source_start as i64;
                        let len = vals.next().unwrap();
                        Range {
                            range: source_start..=source_start + len - 1,
                            offset,
                        }
                    })
                    .into();

                (source, (dest, items))
            }),
    );

    let mut ranges = seeds;
    let mut current_type = "seed";

    while current_type != "location" {
        let (destination, map) = maps.get(current_type).unwrap();

        ranges = ranges.combine(map).apply_offsets();

        current_type = destination;
    }

    println!("{}", ranges.min());
}

#[derive(Clone, Debug)]
struct Interval {
    intervals: Vec<Range>,
}

#[derive(Clone, Debug)]
struct Range {
    range: RangeInclusive<u64>,
    offset: i64,
}

impl Range {
    fn new(range: RangeInclusive<u64>) -> Self {
        Self { range, offset: 0 }
    }

    fn new_with_offset(range: RangeInclusive<u64>, offset: i64) -> Self {
        Self { range, offset }
    }

    fn apply_offset(self) -> Self {
        let (start, end) = self.range.into_inner();
        let (start, end) = (start as i64, end as i64);
        Self {
            range: (start + self.offset) as u64..=(end + self.offset) as u64,
            offset: 0,
        }
    }
}

impl<I: IntoIterator<Item = Range>> From<I> for Interval {
    fn from(value: I) -> Self {
        Self {
            intervals: value.into_iter().collect(),
        }
        .normalize()
    }
}

impl Interval {
    fn min(&self) -> u64 {
        self.intervals
            .iter()
            .map(|val| *val.range.start())
            .min()
            .unwrap()
    }

    fn apply_offsets(self) -> Self {
        Self {
            intervals: self
                .intervals
                .into_iter()
                .map(|val| val.apply_offset())
                .collect(),
        }
        .normalize()
    }

    fn normalize(self) -> Self {
        let mut intervals = self.intervals;
        intervals.sort_unstable_by_key(|range| *range.range.start());

        let mut index = 0;
        while index < intervals.len() - 1 {
            let this = &intervals[index];
            let next = &intervals[index + 1];

            if this.offset != next.offset {
                index += 1;
                continue;
            }

            if this.range.end() >= next.range.start() {
                intervals[index] =
                    Range::new_with_offset(*this.range.start()..=*next.range.end(), this.offset);
                intervals.remove(index + 1);
            } else {
                index += 1;
            }
        }

        Self { intervals }
    }

    fn combine(self, other: &Interval) -> Self {
        let mut new_intervals = Vec::new();

        let mut idx = 0;

        for mut this_range in self.intervals {
            loop {
                if idx >= other.intervals.len() {
                    new_intervals.push(this_range.clone());
                    break;
                }

                let other_range = &other.intervals[idx];

                if (*other_range.range.start() < *this_range.range.start()
                    && *other_range.range.end() < *this_range.range.start())
                    || (*other_range.range.start() > *this_range.range.end()
                        && *other_range.range.end() > *this_range.range.end())
                {
                    idx += 1;
                    continue;
                }

                if *other_range.range.start() > *this_range.range.start() {
                    new_intervals.push(Range {
                        range: *this_range.range.start()..=*other_range.range.start() - 1,
                        offset: 0,
                    });
                }

                let start = (*other_range.range.start()).max(*this_range.range.start());

                if *this_range.range.end() <= *other_range.range.end() {
                    new_intervals.push(Range {
                        range: start..=*this_range.range.end(),
                        offset: other_range.offset,
                    });
                    break;
                }

                new_intervals.push(Range {
                    range: start..=*other_range.range.end(),
                    offset: other_range.offset,
                });
                this_range.range = *other_range.range.end() + 1..=*this_range.range.end();
                idx += 1;
            }
        }

        Self {
            intervals: new_intervals,
        }
        .normalize()
    }
}
