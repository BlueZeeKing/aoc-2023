use core::panic;
use std::fs;

const DIRS: &[Direction] = &[
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let height = input.lines().count();

    let field = PipeField::infer_width(
        input
            .lines()
            .map(|line| {
                line.chars().map(|tile| match tile {
                    'S' => Tile::Start,
                    '-' => Tile::Horizontal,
                    '|' => Tile::Vertical,
                    '.' => Tile::Ground,
                    'J' => Tile::TL,
                    'L' => Tile::TR,
                    '7' => Tile::BL,
                    'F' => Tile::BR,
                    _ => panic!(),
                })
            })
            .flatten()
            .collect(),
        height,
    );

    let start = field.iter().find(|(_, tile)| *tile == Tile::Start).unwrap();

    let loop_list = DIRS
        .iter()
        .find_map(|dir| {
            LoopIter {
                field: &field,
                dir: *dir,
                position: start.0,
            }
            .collect::<Result<Vec<_>, ()>>()
            .ok()
        })
        .unwrap();

    dbg!((loop_list.len() + 1) / 2);
}

struct LoopIter<'a> {
    position: (usize, usize),
    dir: Direction,
    field: &'a PipeField,
}

impl<'a> Iterator for LoopIter<'a> {
    type Item = Result<((usize, usize), Tile), ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dir.apply(&mut self.position).is_err() {
            return Some(Err(()));
        }

        let Some(new_tile) = self.field.maybe_get(self.position.0, self.position.1) else {
            return Some(Err(()));
        };

        if new_tile == Tile::Start {
            return None;
        }

        if new_tile.apply_to_direction(&mut self.dir).is_err() {
            return Some(Err(()));
        }

        Some(Ok((self.position, new_tile)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, pos: &mut (usize, usize)) -> Result<(), ()> {
        if (pos.0 == 0 && *self == Direction::Left) || (pos.1 == 0 && *self == Direction::Up) {
            return Err(());
        }

        match self {
            Direction::Up => pos.1 -= 1,
            Direction::Down => pos.1 += 1,
            Direction::Left => pos.0 -= 1,
            Direction::Right => pos.0 += 1,
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Start,
    TR,
    TL,
    BR,
    BL,
    Vertical,
    Horizontal,
    Ground,
}

impl Tile {
    fn apply_to_direction(&self, dir: &mut Direction) -> Result<(), ()> {
        match self {
            Tile::Start => Ok(()),
            Tile::BL => match dir {
                Direction::Down | Direction::Left => Err(()),
                Direction::Up => {
                    *dir = Direction::Left;
                    Ok(())
                }
                Direction::Right => {
                    *dir = Direction::Down;
                    Ok(())
                }
            },
            Tile::BR => match dir {
                Direction::Down | Direction::Right => Err(()),
                Direction::Up => {
                    *dir = Direction::Right;
                    Ok(())
                }
                Direction::Left => {
                    *dir = Direction::Down;
                    Ok(())
                }
            },
            Tile::TL => match dir {
                Direction::Up | Direction::Left => Err(()),
                Direction::Down => {
                    *dir = Direction::Left;
                    Ok(())
                }
                Direction::Right => {
                    *dir = Direction::Up;
                    Ok(())
                }
            },
            Tile::TR => match dir {
                Direction::Up | Direction::Right => Err(()),
                Direction::Down => {
                    *dir = Direction::Right;
                    Ok(())
                }
                Direction::Left => {
                    *dir = Direction::Up;
                    Ok(())
                }
            },
            Tile::Vertical => match dir {
                Direction::Left | Direction::Right => Err(()),
                Direction::Up | Direction::Down => Ok(()),
            },
            Tile::Horizontal => match dir {
                Direction::Up | Direction::Down => Err(()),
                Direction::Left | Direction::Right => Ok(()),
            },
            Tile::Ground => Err(()),
        }
    }
}

struct PipeField {
    field: Vec<Tile>,
    width: usize,
    height: usize,
}

impl PipeField {
    fn infer_width(field: Vec<Tile>, height: usize) -> Self {
        Self {
            height,
            width: field.len() / height,
            field,
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.field[y * self.width + x]
    }

    fn maybe_get(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.get(x, y))
        }
    }

    fn iter(&self) -> PipeFieldIterator {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a PipeField {
    type Item = ((usize, usize), Tile);
    type IntoIter = PipeFieldIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PipeFieldIterator {
            field: self,
            current_pos: (0, 0),
        }
    }
}

struct PipeFieldIterator<'a> {
    field: &'a PipeField,
    current_pos: (usize, usize),
}

impl<'a> Iterator for PipeFieldIterator<'a> {
    type Item = ((usize, usize), Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos.1 >= self.field.height {
            None
        } else {
            let res = Some((
                self.current_pos,
                self.field.get(self.current_pos.0, self.current_pos.1),
            ));

            self.current_pos.0 += 1;

            if self.current_pos.0 >= self.field.width {
                self.current_pos.0 = 0;
                self.current_pos.1 += 1;
            }

            res
        }
    }
}
