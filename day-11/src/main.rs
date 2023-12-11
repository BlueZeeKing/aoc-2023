use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let image = Image::new(height, width, &input);

    let empty_cols = image
        .cols()
        .enumerate()
        .filter_map(|(x, mut col)| {
            if !col.any(|is_galaxy| is_galaxy) {
                Some(x)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let empty_rows = image
        .rows()
        .enumerate()
        .filter_map(|(y, mut row)| {
            if !row.any(|is_galaxy| is_galaxy) {
                Some(y)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let galaxies = image
        .iter()
        .filter(|(_pos, is_galaxy)| *is_galaxy)
        .collect::<Vec<_>>();

    let total_dist: usize = PairsIter::new(&galaxies)
        .map(|((a, _), (b, _))| {
            let x_range = if a.0 > b.0 { b.0..a.0 } else { a.0..b.0 };
            let y_range = if a.1 > b.1 { b.1..a.1 } else { a.1..b.1 };

            let x_num = empty_cols.iter().filter(|x| x_range.contains(*x)).count() * (1000000 - 1);
            let y_num = empty_rows.iter().filter(|y| y_range.contains(*y)).count() * (1000000 - 1);

            (x_range.end - x_range.start + x_num) + (y_range.end - y_range.start + y_num)
        })
        .sum();

    dbg!(total_dist);
}

struct PairsIter<'a, T> {
    slice: &'a [T],
    current_index: usize,
    sub_index: usize,
}

impl<'a, T> PairsIter<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        Self {
            slice,
            current_index: 0,
            sub_index: 1,
        }
    }
}

impl<'a, T> Iterator for PairsIter<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.slice.len() - 1 {
            return None;
        }

        let res = (&self.slice[self.current_index], &self.slice[self.sub_index]);

        self.sub_index += 1;
        if self.sub_index >= self.slice.len() {
            self.current_index += 1;
            self.sub_index = self.current_index + 1;
        }

        Some(res)
    }
}

struct Image {
    width: usize,
    height: usize,
    image: Vec<bool>,
}

struct ImageIter<'a> {
    image: &'a Image,
    x: usize,
    y: usize,
}

impl<'a> Iterator for ImageIter<'a> {
    type Item = ((usize, usize), bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.image.height {
            None
        } else {
            let res = ((self.x, self.y), self.image.get((self.x, self.y)));

            self.x += 1;
            if self.x >= self.image.width {
                self.x = 0;
                self.y += 1;
            }

            Some(res)
        }
    }
}

impl Image {
    fn new(height: usize, width: usize, input: &str) -> Self {
        Self {
            width,
            height,
            image: input
                .replace('\n', "")
                .chars()
                .map(|char| char == '#')
                .collect(),
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> bool {
        self.image[y * self.width + x]
    }

    fn rows(&self) -> ImageRowIter<'_> {
        ImageRowIter { image: &self, y: 0 }
    }

    fn cols(&self) -> ImageColIter<'_> {
        ImageColIter { image: &self, x: 0 }
    }

    fn iter(&self) -> ImageIter<'_> {
        ImageIter {
            image: &self,
            x: 0,
            y: 0,
        }
    }
}

pub struct ImageRowIter<'a> {
    image: &'a Image,
    y: usize,
}

pub struct ImageRowIterIndividual<'a> {
    image: &'a Image,
    y: usize,
    x: usize,
}

impl<'a> Iterator for ImageRowIter<'a> {
    type Item = ImageRowIterIndividual<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.image.height {
            None
        } else {
            let res = Some(ImageRowIterIndividual {
                image: self.image,
                y: self.y,
                x: 0,
            });

            self.y += 1;

            res
        }
    }
}

impl<'a> Iterator for ImageRowIterIndividual<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.image.width {
            None
        } else {
            let res = Some(self.image.get((self.x, self.y)));

            self.x += 1;

            res
        }
    }
}

pub struct ImageColIter<'a> {
    image: &'a Image,
    x: usize,
}

pub struct ImageColIterIndividual<'a> {
    image: &'a Image,
    y: usize,
    x: usize,
}

impl<'a> Iterator for ImageColIter<'a> {
    type Item = ImageColIterIndividual<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.image.width {
            None
        } else {
            let res = Some(ImageColIterIndividual {
                image: self.image,
                y: 0,
                x: self.x,
            });

            self.x += 1;

            res
        }
    }
}

impl<'a> Iterator for ImageColIterIndividual<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.image.height {
            None
        } else {
            let res = Some(self.image.get((self.x, self.y)));

            self.y += 1;

            res
        }
    }
}
