use std::{fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let digits = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, char)| Some((col, char.to_digit(10)?)))
                .map(move |(col, char)| ((col, row), char))
        })
        .flatten()
        .grouping_iter()
        .collect::<Vec<_>>();

    let sum: u32 = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| ((col, row), char))
        })
        .flatten()
        .filter(|(_, char)| *char == '*')
        .filter_map(|((x, y), _)| {
            let mut surrounding_nums = SurroundingIter::new(x..=x, y)
                .filter_map(|(x, y)| {
                    digits
                        .iter()
                        .find(|((dx, dy), _)| dx.contains(&x) && *dy == y)
                })
                .collect::<Vec<_>>();

            surrounding_nums.dedup();

            if surrounding_nums.len() != 2 {
                None
            } else {
                Some(surrounding_nums[0].1 * surrounding_nums[1].1)
            }
        })
        .sum();

    println!("{}", sum);
}

struct SurroundingIter {
    x: RangeInclusive<usize>,
    y: usize,
    cx: usize,
    cy: usize,
}

impl SurroundingIter {
    pub fn new(x: RangeInclusive<usize>, y: usize) -> Self {
        Self {
            cx: x.start().checked_sub(1).unwrap_or(*x.start()),
            cy: y.checked_sub(1).unwrap_or(y),
            x,
            y,
        }
    }
}

impl Iterator for SurroundingIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cy > self.y + 1 {
            return None;
        }

        let res = (self.cx, self.cy);

        self.cx += 1;

        if self.cx > self.x.end() + 1 {
            self.cx = self.x.start().checked_sub(1).unwrap_or(*self.x.start());
            self.cy += 1;
        }

        while self.cy == self.y && self.x.contains(&self.cx) {
            self.cx;
            self.cx += 1;
        }

        Some(res)
    }
}

struct GroupingIter<I: Iterator<Item = ((usize, usize), u32)>> {
    iter: I,
    prev: Option<((usize, usize), u32)>,
}

impl<I: Iterator<Item = ((usize, usize), u32)>> Iterator for GroupingIter<I> {
    type Item = ((RangeInclusive<usize>, usize), u32);

    fn next(&mut self) -> Option<Self::Item> {
        let ((mut x, y), mut val) = self.prev.take().or_else(|| self.iter.next())?;
        let first_x = x;

        loop {
            let Some(((new_x, new_y), new_val)) = self.iter.next() else {
                break;
            };

            if new_x == x + 1 && new_y == y {
                x = new_x;
                val = val * 10 + new_val;
            } else {
                self.prev = Some(((new_x, new_y), new_val));
                break;
            }
        }

        Some(((first_x..=x, y), val))
    }
}

trait IntoGroupingIter: Iterator<Item = ((usize, usize), u32)> + Sized {
    fn grouping_iter(self) -> GroupingIter<Self> {
        GroupingIter {
            iter: self,
            prev: None,
        }
    }
}

impl<I: Iterator<Item = ((usize, usize), u32)> + Sized> IntoGroupingIter for I {}
