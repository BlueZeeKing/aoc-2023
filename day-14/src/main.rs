use core::panic;
use std::fs;

use common::field::Field;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Round,
    Square,
    Ground,
}

impl Into<char> for &Tile {
    fn into(self) -> char {
        match self {
            Tile::Round => 'O',
            Tile::Square => '#',
            Tile::Ground => '.',
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut field = Field::new(
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
        input
            .lines()
            .map(|line| {
                line.chars().map(|tile| match tile {
                    'O' => Tile::Round,
                    '#' => Tile::Square,
                    '.' => Tile::Ground,
                    _ => panic!(),
                })
            })
            .flatten()
            .collect(),
    );

    let blank_field = Field::new(
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
        input
            .lines()
            .map(|line| {
                line.chars().map(|tile| match tile {
                    '#' => Tile::Square,
                    '.' | 'O' => Tile::Ground,
                    _ => panic!(),
                })
            })
            .flatten()
            .collect(),
    );

    let mut history = Vec::new();

    let end_index = loop {
        match spin_cycle(&field, &blank_field, &mut history) {
            Either::A(new_field) => field = new_field,
            Either::B(new_end_index) => break new_end_index,
        }
    };

    let history = history;

    let amount_left = 1000000000 - end_index;
    let history_size = history.len() - end_index;

    let field = (amount_left % history_size) + end_index - 1;

    let sum: usize = history[field]
        .iter()
        .filter(|(_, tile)| **tile == Tile::Round)
        .map(|((_x, y), _)| history[0].num_rows() - y)
        .sum();

    println!("{sum}");
}

enum Either<A, B> {
    A(A),
    B(B),
}

fn spin_cycle(
    field: &Field<Tile>,
    empty: &Field<Tile>,
    history: &mut Vec<Field<Tile>>,
) -> Either<Field<Tile>, usize> {
    if let Some(idx) = history[0..history.len().checked_sub(1).unwrap_or(0)]
        .iter()
        .position(|check_field| *field == *check_field)
    {
        history.pop();
        Either::B(idx)
    } else {
        let north = spin_cycle_north(field, empty);
        let west = spin_cycle_west(&north, empty);
        let south = spin_cycle_south(&west, empty);
        let east = spin_cycle_east(&south, empty);

        history.push(east.clone());

        Either::A(east)
    }
}

fn spin_cycle_north(field: &Field<Tile>, empty: &Field<Tile>) -> Field<Tile> {
    let mut new_field = empty.clone();

    field.cols().enumerate().for_each(|(x, col)| {
        let mut previous_index: i64 = -1;
        let mut previous_end_index = 0;

        col.clone()
            .enumerate()
            .filter(|(_idx, item)| **item == Tile::Round)
            .map(|(index, _item)| index)
            .for_each(|index| {
                let new_idx = ((previous_index + 1) as usize..index)
                    .rev()
                    .map(|y| (y, col.get(y)))
                    .find(|(_y, tile)| **tile != Tile::Ground)
                    .map(|(y, _tile)| y + 1)
                    .unwrap_or(previous_end_index);

                previous_end_index = new_idx + 1;
                previous_index = index as i64;

                *new_field.get_mut((x, new_idx)) = Tile::Round;
            });
    });

    new_field
}

fn spin_cycle_west(field: &Field<Tile>, empty: &Field<Tile>) -> Field<Tile> {
    let mut new_field = empty.clone();

    field.rows().enumerate().for_each(|(y, row)| {
        let mut previous_index: i64 = -1;
        let mut previous_end_index = 0;

        row.clone()
            .enumerate()
            .filter(|(_idx, item)| **item == Tile::Round)
            .map(|(index, _item)| index)
            .for_each(|index| {
                let new_idx = ((previous_index + 1) as usize..index)
                    .rev()
                    .map(|x| (x, row.get(x)))
                    .find(|(_x, tile)| **tile != Tile::Ground)
                    .map(|(x, _tile)| x + 1)
                    .unwrap_or(previous_end_index);

                previous_end_index = new_idx + 1;
                previous_index = index as i64;

                *new_field.get_mut((new_idx, y)) = Tile::Round;
            });
    });

    new_field
}

fn spin_cycle_south(field: &Field<Tile>, empty: &Field<Tile>) -> Field<Tile> {
    let mut new_field = empty.clone();

    field.cols().enumerate().for_each(|(x, col)| {
        let mut previous_index: i64 = field.num_rows() as i64;
        let mut previous_end_index = field.num_rows() - 1;

        col.clone()
            .enumerate()
            .rev()
            .filter(|(_idx, item)| **item == Tile::Round)
            .map(|(index, _item)| index)
            .for_each(|index| {
                let new_idx = (index + 1..previous_index as usize)
                    .map(|y| (y, col.get(y)))
                    .find(|(_y, tile)| **tile != Tile::Ground)
                    .map(|(y, _tile)| y - 1)
                    .unwrap_or(previous_end_index);

                if index > 0 {
                    previous_end_index = new_idx - 1;
                    previous_index = index as i64;
                }

                *new_field.get_mut((x, new_idx)) = Tile::Round;
            });
    });

    new_field
}

fn spin_cycle_east(field: &Field<Tile>, empty: &Field<Tile>) -> Field<Tile> {
    let mut new_field = empty.clone();

    field.rows().enumerate().for_each(|(y, row)| {
        let mut previous_index: i64 = field.num_cols() as i64;
        let mut previous_end_index = field.num_cols() - 1;

        row.clone()
            .enumerate()
            .rev()
            .filter(|(_idx, item)| **item == Tile::Round)
            .map(|(index, _item)| index)
            .for_each(|index| {
                let new_idx = (index + 1..previous_index as usize)
                    .map(|x| (x, row.get(x)))
                    .find(|(_x, tile)| **tile != Tile::Ground)
                    .map(|(x, _tile)| x - 1)
                    .unwrap_or(previous_end_index);

                if index > 0 {
                    previous_end_index = new_idx - 1;
                    previous_index = index as i64;
                }

                *new_field.get_mut((new_idx, y)) = Tile::Round;
            });
    });

    new_field
}
