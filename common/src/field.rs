use std::{fmt::Debug, iter::once};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Field<T> {
    width: usize,
    height: usize,
    field: Vec<T>,
}

impl<T> Debug for Field<T>
where
    for<'a> &'a T: Into<char>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field = self
            .rows()
            .map(|row| {
                once('\t')
                    .chain(row.map(|tile| Into::into(tile)).chain(once('\n')))
                    .collect::<String>()
            })
            .collect::<String>();

        write!(
            f,
            "Field: {{\n\twidth: {},\n\theight: {},\n\tfield:\n\n{}\n}}",
            self.width, self.height, field
        )
    }
}

impl<T> Field<T> {
    pub fn new(height: usize, width: usize, input: Vec<T>) -> Self {
        Self {
            width,
            height,
            field: input,
        }
    }

    pub fn get(&self, (x, y): (usize, usize)) -> &T {
        &self.field[y * self.width + x]
    }

    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.field[y * self.width + x]
    }

    pub fn rows(&self) -> FieldRowIter<'_, T> {
        FieldRowIter {
            field: &self,
            y: 0,
            y_back: self.height - 1,
        }
    }

    pub fn row(&self, y: usize) -> FieldRowIterIndividual<'_, T> {
        FieldRowIterIndividual {
            field: &self,
            y,
            x: 0,
            x_back: self.width - 1,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.height
    }

    pub fn cols(&self) -> FieldColIter<'_, T> {
        FieldColIter {
            field: &self,
            x: 0,
            x_back: self.width - 1,
        }
    }

    pub fn col(&self, x: usize) -> FieldColIterIndividual<'_, T> {
        FieldColIterIndividual {
            field: &self,
            x,
            y: 0,
            y_back: self.height - 1,
        }
    }

    pub fn num_cols(&self) -> usize {
        self.width
    }

    pub fn iter(&self) -> FieldIter<'_, T> {
        FieldIter {
            field: &self,
            idx: 0,
            idx_back: self.field.len() - 1,
        }
    }

    fn idx_to_coords(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.height)
    }

    fn get_idx(&self, idx: usize) -> &T {
        &self.field[idx]
    }
}

pub struct FieldIter<'a, T> {
    field: &'a Field<T>,
    idx: usize,
    idx_back: usize,
}

impl<'a, T> Iterator for FieldIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > self.idx_back {
            None
        } else {
            let res = (
                self.field.idx_to_coords(self.idx),
                self.field.get_idx(self.idx),
            );

            self.idx += 1;

            Some(res)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.idx_back + 1 - self.idx;

        (remaining, Some(remaining))
    }
}

impl<'a, T> DoubleEndedIterator for FieldIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx > self.idx_back {
            None
        } else {
            let res = (
                self.field.idx_to_coords(self.idx_back),
                self.field.get_idx(self.idx_back),
            );

            self.idx_back -= 1;

            Some(res)
        }
    }
}

pub struct FieldRowIter<'a, T> {
    field: &'a Field<T>,
    y: usize,
    y_back: usize,
}

#[derive(Clone)]
pub struct FieldRowIterIndividual<'a, T> {
    field: &'a Field<T>,
    y: usize,
    x: usize,
    x_back: usize,
}

impl<'a, T> Iterator for FieldRowIter<'a, T> {
    type Item = FieldRowIterIndividual<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.y_back {
            None
        } else {
            let res = Some(FieldRowIterIndividual {
                field: self.field,
                y: self.y,
                x: 0,
                x_back: self.field.width - 1,
            });

            self.y += 1;

            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.y_back + 1 - self.y, Some(self.y_back + 1 - self.y))
    }
}

impl<'a, T> DoubleEndedIterator for FieldRowIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y > self.y_back {
            None
        } else {
            let res = Some(FieldRowIterIndividual {
                field: self.field,
                y: self.y_back,
                x: 0,
                x_back: self.field.width - 1,
            });

            match self.y_back.checked_sub(1) {
                Some(val) => self.y_back = val,
                None => {
                    self.y_back = 0;
                    self.y = 1;
                }
            }

            res
        }
    }
}

impl<'a, T> Iterator for FieldRowIterIndividual<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.x_back {
            None
        } else {
            let res = Some(self.field.get((self.x, self.y)));

            self.x += 1;

            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.x_back + 1 - self.x, Some(self.x_back + 1 - self.x))
    }
}

impl<'a, T> DoubleEndedIterator for FieldRowIterIndividual<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x > self.x_back {
            None
        } else {
            let res = Some(self.field.get((self.x_back, self.y)));

            match self.x_back.checked_sub(1) {
                Some(val) => self.x_back = val,
                None => {
                    self.x_back = 0;
                    self.x = 1;
                }
            }

            res
        }
    }
}

pub struct FieldColIter<'a, T> {
    field: &'a Field<T>,
    x: usize,
    x_back: usize,
}

#[derive(Clone)]
pub struct FieldColIterIndividual<'a, T> {
    field: &'a Field<T>,
    y: usize,
    y_back: usize,
    x: usize,
}

impl<'a, T> FieldColIterIndividual<'a, T> {
    pub fn get(&self, y: usize) -> &'a T {
        self.field.get((self.x, y))
    }
}

impl<'a, T> FieldRowIterIndividual<'a, T> {
    pub fn get(&self, x: usize) -> &'a T {
        self.field.get((x, self.y))
    }
}

impl<'a, T> Iterator for FieldColIter<'a, T> {
    type Item = FieldColIterIndividual<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.x_back {
            None
        } else {
            let res = Some(FieldColIterIndividual {
                field: self.field,
                y: 0,
                y_back: self.field.height - 1,
                x: self.x,
            });

            self.x += 1;

            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.x_back + 1 - self.x, Some(self.x_back + 1 - self.x))
    }
}

impl<'a, T> DoubleEndedIterator for FieldColIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x > self.x_back {
            None
        } else {
            let res = Some(FieldColIterIndividual {
                field: self.field,
                y: 0,
                y_back: self.field.height - 1,
                x: self.x_back,
            });

            match self.x_back.checked_sub(1) {
                Some(val) => self.x_back = val,
                None => {
                    self.x_back = 0;
                    self.x = 1;
                }
            }

            res
        }
    }
}

impl<'a, T> Iterator for FieldColIterIndividual<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.y_back {
            None
        } else {
            let res = Some(self.field.get((self.x, self.y)));

            self.y += 1;

            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.y_back + 1 - self.y, Some(self.y_back + 1 - self.y))
    }
}

impl<'a, T> DoubleEndedIterator for FieldColIterIndividual<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y > self.y_back {
            None
        } else {
            let res = Some(self.field.get((self.x, self.y_back)));

            match self.y_back.checked_sub(1) {
                Some(val) => self.y_back = val,
                None => {
                    self.y_back = 0;
                    self.y = 1;
                }
            }

            res
        }
    }
}

impl<'a, T> ExactSizeIterator for FieldColIter<'a, T> {}
impl<'a, T> ExactSizeIterator for FieldColIterIndividual<'a, T> {}
impl<'a, T> ExactSizeIterator for FieldRowIter<'a, T> {}
impl<'a, T> ExactSizeIterator for FieldRowIterIndividual<'a, T> {}
impl<'a, T> ExactSizeIterator for FieldIter<'a, T> {}
