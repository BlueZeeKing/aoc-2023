use common::field::Field;
use core::panic;
use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug, Clone, Copy)]
enum Mirror {
    Horizontal(usize),
    Vertical(usize),
}

fn main() {
    let sum = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|field| {
            let height = field.len();
            let width = field[0].len();

            Field::new(
                height,
                width,
                field
                    .iter()
                    .map(|line| {
                        line.chars().map(|char| match char {
                            '.' => Tile::Ash,
                            '#' => Tile::Rock,
                            _ => panic!(),
                        })
                    })
                    .flatten()
                    .collect(),
            )
        })
        .map(|field| {
            {
                let mut cols = field.cols().enumerate();
                let mut cols_next = field.cols();
                cols_next.next().unwrap();

                while let (Some((x, col)), Some(col_next)) = (cols.next(), cols_next.next()) {
                    if col.zip(col_next).filter(|(a, b)| **a != **b).count() <= 1
                        && (0..x + 1)
                            .rev()
                            .zip(x + 1..field.num_cols())
                            .map(|(a, b)| (field.col(a), field.col(b)))
                            .map(|(a, b)| a.zip(b).filter(|(a, b)| **a != **b).count())
                            .sum::<usize>()
                            == 1
                    {
                        return Mirror::Vertical(x);
                    }
                }
            }

            {
                let mut rows = field.rows().enumerate();
                let mut rows_next = field.rows();
                rows_next.next().unwrap();

                while let (Some((y, row)), Some(row_next)) = (rows.next(), rows_next.next()) {
                    if row.zip(row_next).filter(|(a, b)| **a != **b).count() <= 1
                        && (0..y + 1)
                            .rev()
                            .zip(y + 1..field.num_rows())
                            .map(|(a, b)| (field.row(a), field.row(b)))
                            .map(|(a, b)| a.zip(b).filter(|(a, b)| **a != **b).count())
                            .sum::<usize>()
                            == 1
                    {
                        return Mirror::Horizontal(y);
                    }
                }
            }

            panic!("Could not find reflection")
        })
        .map(|mirror| match mirror {
            Mirror::Horizontal(num) => (num + 1) * 100,
            Mirror::Vertical(num) => num + 1,
        })
        .sum::<usize>();

    dbg!(sum);
}
