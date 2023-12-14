// TODO: Double ended iterators

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
        FieldRowIter { field: &self, y: 0 }
    }

    pub fn row(&self, y: usize) -> FieldRowIterIndividual<'_, T> {
        FieldRowIterIndividual {
            field: &self,
            y,
            x: 0,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.height
    }

    pub fn cols(&self) -> FieldColIter<'_, T> {
        FieldColIter { field: &self, x: 0 }
    }

    pub fn col(&self, x: usize) -> FieldColIterIndividual<'_, T> {
        FieldColIterIndividual {
            field: &self,
            x,
            y: 0,
        }
    }

    pub fn num_cols(&self) -> usize {
        self.width
    }

    pub fn iter(&self) -> FieldIter<'_, T> {
        FieldIter {
            field: &self,
            x: 0,
            y: 0,
        }
    }
}

pub struct FieldIter<'a, T> {
    field: &'a Field<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for FieldIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.field.height {
            None
        } else {
            let res = ((self.x, self.y), self.field.get((self.x, self.y)));

            self.x += 1;
            if self.x >= self.field.width {
                self.x = 0;
                self.y += 1;
            }

            Some(res)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let idx = self.y * self.field.width + self.x;
        let remaining = self.field.field.len() - idx;

        (remaining, Some(remaining))
    }
}

pub struct FieldRowIter<'a, T> {
    field: &'a Field<T>,
    y: usize,
}

#[derive(Clone)]
pub struct FieldRowIterIndividual<'a, T> {
    field: &'a Field<T>,
    y: usize,
    x: usize,
}

impl<'a, T> Iterator for FieldRowIter<'a, T> {
    type Item = FieldRowIterIndividual<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.field.height {
            None
        } else {
            let res = Some(FieldRowIterIndividual {
                field: self.field,
                y: self.y,
                x: 0,
            });

            self.y += 1;

            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.field.height - self.y, Some(self.field.height - self.y))
    }
}

impl<'a, T> Iterator for FieldRowIterIndividual<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.field.width {
            None
        } else {
            let res = Some(self.field.get((self.x, self.y)));

            self.x += 1;

            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.field.width - self.x, Some(self.field.width - self.x))
    }
}

pub struct FieldColIter<'a, T> {
    field: &'a Field<T>,
    x: usize,
}

#[derive(Clone)]
pub struct FieldColIterIndividual<'a, T> {
    field: &'a Field<T>,
    y: usize,
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
        if self.x >= self.field.width {
            None
        } else {
            let res = Some(FieldColIterIndividual {
                field: self.field,
                y: 0,
                x: self.x,
            });

            self.x += 1;

            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.field.width - self.x, Some(self.field.width - self.x))
    }
}

impl<'a, T> Iterator for FieldColIterIndividual<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.field.height {
            None
        } else {
            let res = Some(self.field.get((self.x, self.y)));

            self.y += 1;

            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.field.height - self.y, Some(self.field.height - self.y))
    }
}

impl<'a, T> ExactSizeIterator for FieldColIter<'a, T> {}
impl<'a, T> ExactSizeIterator for FieldColIterIndividual<'a, T> {}
impl<'a, T> ExactSizeIterator for FieldRowIter<'a, T> {}
impl<'a, T> ExactSizeIterator for FieldRowIterIndividual<'a, T> {}
impl<'a, T> ExactSizeIterator for FieldIter<'a, T> {}
