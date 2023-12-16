use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

use common::field::Field;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Into<char> for &Tile {
    fn into(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::ForwardMirror => '/',
            Tile::BackwardMirror => '\\',
            Tile::VerticalSplitter => '|',
            Tile::HorizontalSplitter => '-',
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let field = Field::new(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        input
            .lines()
            .map(|row| {
                row.chars().map(|tile| match tile {
                    '.' => Tile::Empty,
                    '/' => Tile::ForwardMirror,
                    '\\' => Tile::BackwardMirror,
                    '|' => Tile::VerticalSplitter,
                    '-' => Tile::HorizontalSplitter,
                    _ => panic!(),
                })
            })
            .flatten()
            .collect(),
    );

    let first_col = field
        .cols()
        .next()
        .unwrap()
        .enumerate()
        .map(|(y, _tile)| ((0, y), Direction::Right));
    let last_col = field
        .cols()
        .next_back()
        .unwrap()
        .enumerate()
        .map(|(y, _tile)| ((field.num_cols() - 1, y), Direction::Left));

    let first_row = field
        .rows()
        .next()
        .unwrap()
        .enumerate()
        .map(|(x, _tile)| ((x, 0), Direction::Down));
    let last_row = field
        .rows()
        .next_back()
        .unwrap()
        .enumerate()
        .map(|(x, _tile)| ((x, field.num_rows() - 1), Direction::Up));

    let starting_positions = first_col.chain(last_col).chain(first_row).chain(last_row);

    let max = starting_positions
        .map(|(pos, dir)| {
            let pos = Position(pos.0, pos.1);
            let mut map = HashMap::new();
            traverse_field(&field, dir, pos, &mut map);
            map.len()
        })
        .max()
        .unwrap();

    dbg!(max);
}

#[derive(Clone, Copy, Debug)]
struct DirectionMap(u8);

impl DirectionMap {
    fn add(&mut self, dir: Direction) {
        let mask = Self::dir_to_mask(dir);
        self.0 |= mask;
    }

    fn has(&self, dir: Direction) -> bool {
        let mask = Self::dir_to_mask(dir);
        self.0 & mask != 0
    }

    fn dir_to_mask(dir: Direction) -> u8 {
        1 << match dir {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    fn new(dir: Direction) -> Self {
        let mut map = DirectionMap(0);
        map.add(dir);
        map
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    fn appy_direction(&self, dir: Direction, field: &Field<Tile>) -> Option<Self> {
        let mut x = self.0;
        let mut y = self.1;

        match dir {
            Direction::Up => y = y.checked_sub(1)?,
            Direction::Down => y += 1,
            Direction::Left => x = x.checked_sub(1)?,
            Direction::Right => x += 1,
        }

        if x >= field.num_cols() || y >= field.num_rows() {
            None
        } else {
            Some(Self(x, y))
        }
    }
}

impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

fn traverse_field(
    field: &Field<Tile>,
    direction: Direction,
    position: Position,
    already_traveled: &mut HashMap<Position, DirectionMap>,
) {
    if matches!(already_traveled.get(&position), Some(map) if map.has(direction)) {
        return;
    } else {
        match already_traveled.entry(position) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().add(direction);
            }
            Entry::Vacant(entry) => {
                entry.insert(DirectionMap::new(direction));
            }
        }
    }

    let tile = field.get(position.into());
    match tile {
        Tile::Empty => {
            if let Some(new_pos) = position.appy_direction(direction, field) {
                traverse_field(field, direction, new_pos, already_traveled)
            }
        }
        Tile::ForwardMirror => {
            let dir = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };

            if let Some(new_pos) = position.appy_direction(dir, field) {
                traverse_field(field, dir, new_pos, already_traveled)
            }
        }
        Tile::BackwardMirror => {
            let dir = match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };

            if let Some(new_pos) = position.appy_direction(dir, field) {
                traverse_field(field, dir, new_pos, already_traveled)
            }
        }
        Tile::VerticalSplitter => match direction {
            Direction::Up | Direction::Down => {
                if let Some(new_pos) = position.appy_direction(direction, field) {
                    traverse_field(field, direction, new_pos, already_traveled)
                }
            }
            Direction::Left | Direction::Right => {
                if let Some(new_pos) = position.appy_direction(Direction::Up, field) {
                    traverse_field(field, Direction::Up, new_pos, already_traveled)
                }

                if let Some(new_pos) = position.appy_direction(Direction::Down, field) {
                    traverse_field(field, Direction::Down, new_pos, already_traveled)
                }
            }
        },
        Tile::HorizontalSplitter => match direction {
            Direction::Left | Direction::Right => {
                if let Some(new_pos) = position.appy_direction(direction, field) {
                    traverse_field(field, direction, new_pos, already_traveled)
                }
            }
            Direction::Down | Direction::Up => {
                if let Some(new_pos) = position.appy_direction(Direction::Right, field) {
                    traverse_field(field, Direction::Right, new_pos, already_traveled)
                }

                if let Some(new_pos) = position.appy_direction(Direction::Left, field) {
                    traverse_field(field, Direction::Left, new_pos, already_traveled)
                }
            }
        },
    }
}
