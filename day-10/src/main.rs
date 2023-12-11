use std::fs;

const DIRS: &[Direction] = &[
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];

struct Path {
    path: Vec<Option<Tile>>,
    width: usize,
    height: usize,
}

impl Path {
    fn set(&mut self, (x, y): (usize, usize), tile: Tile) {
        self.path[y * self.width + x] = Some(tile);
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            path: vec![None; width * height],
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<Tile> {
        self.path[y * self.width + x]
    }
}

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

    let (mut path, start_dir) = DIRS
        .iter()
        .find_map(|dir| {
            let loop_iter = LoopIter {
                field: &field,
                dir: *dir,
                position: start.0,
            };
            let mut path = Path::new(field.width, field.height);
            for val in loop_iter {
                match val {
                    Err(()) => return None,
                    Ok((pos, tile)) => path.set(pos, tile),
                }
            }

            Some((path, dir))
        })
        .unwrap();

    let (other_dir, _) = DIRS
        .iter()
        .filter(|dir| **dir != *start_dir)
        .filter_map(|dir| Some((dir, dir.apply(start.0).ok()?)))
        .filter_map(|(dir, pos)| Some((dir, path.get(pos)?)))
        .find(|(dir, tile)| match dir {
            Direction::Up => *tile == Tile::BL || *tile == Tile::BR || *tile == Tile::Vertical,
            Direction::Down => *tile == Tile::TL || *tile == Tile::TR || *tile == Tile::Vertical,
            Direction::Left => *tile == Tile::BR || *tile == Tile::TR || *tile == Tile::Horizontal,
            Direction::Right => *tile == Tile::BL || *tile == Tile::TL || *tile == Tile::Horizontal,
        })
        .unwrap();

    let start_tile = match (start_dir, other_dir) {
        (Direction::Up, Direction::Down) => Tile::Vertical,
        (Direction::Up, Direction::Left) => Tile::TL,
        (Direction::Up, Direction::Right) => Tile::TR,
        (Direction::Down, Direction::Up) => Tile::Vertical,
        (Direction::Down, Direction::Left) => Tile::BL,
        (Direction::Down, Direction::Right) => Tile::BR,
        (Direction::Left, Direction::Up) => Tile::TL,
        (Direction::Left, Direction::Down) => Tile::BL,
        (Direction::Left, Direction::Right) => Tile::Horizontal,
        (Direction::Right, Direction::Up) => Tile::TR,
        (Direction::Right, Direction::Down) => Tile::BR,
        (Direction::Right, Direction::Left) => Tile::Horizontal,
        _ => unreachable!(),
    };

    path.set(start.0, start_tile);

    let path = path;

    let full = field
        .iter()
        .filter(|(pos, _tile)| path.get(*pos).is_none())
        .filter(|(pos, _tile)| {
            let mut prev_tile = None;
            let mut count = 0;
            for (x, y) in (pos.0..field.width).map(|x| (x, pos.1)) {
                let Some(tile) = path.get((x, y)) else {
                    continue;
                };

                let tile = tile;

                if tile == Tile::Horizontal {
                    continue;
                }

                if tile == Tile::Vertical {
                    count += 1;
                    continue;
                }

                if let Some(prev_tile) = prev_tile.take() {
                    let top_prev = match prev_tile {
                        Tile::TR => true,
                        Tile::TL => true,
                        Tile::BR => false,
                        Tile::BL => false,
                        _ => unreachable!(),
                    };

                    let top = match tile {
                        Tile::TR => true,
                        Tile::TL => true,
                        Tile::BR => false,
                        Tile::BL => false,
                        _ => unreachable!(),
                    };

                    if top != top_prev {
                        count += 1;
                    }
                } else {
                    prev_tile = Some(tile);
                }
            }

            count % 2 == 1
        })
        // .for_each(|val| println!("{:?}", val));
        .count();

    dbg!(full);
}

struct LoopIter<'a> {
    position: (usize, usize),
    dir: Direction,
    field: &'a PipeField,
}

impl<'a> Iterator for LoopIter<'a> {
    type Item = Result<((usize, usize), Tile), ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let Ok(pos) = self.dir.apply(self.position) else {
            return Some(Err(()));
        };
        self.position = pos;

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
    fn apply(&self, pos: (usize, usize)) -> Result<(usize, usize), ()> {
        if (pos.0 == 0 && *self == Direction::Left) || (pos.1 == 0 && *self == Direction::Up) {
            return Err(());
        }

        Ok(match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        })
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
