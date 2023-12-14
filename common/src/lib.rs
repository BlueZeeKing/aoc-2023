use std::fmt::Debug;

pub mod field;

pub trait IterExt: Iterator + Sized {
    fn consume(self) {
        self.for_each(|_| {});
    }
}

impl<I: Iterator + Sized> IterExt for I {}

pub struct DebugIter<T: Debug, I: Iterator<Item = T>> {
    iter: I,
}

impl<T: Debug, I: Iterator<Item = T>> Iterator for DebugIter<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        eprintln!("Debug iter: {:#?}", item);
        Some(item)
    }
}

pub trait IterDebugExt<T: Debug>: Iterator<Item = T> + Sized {
    fn debug(self) -> DebugIter<Self::Item, Self> {
        DebugIter { iter: self }
    }
}

impl<T: Debug, I: Iterator<Item = T> + Sized> IterDebugExt<T> for I {}

pub trait EqualExt<T: PartialEq>: Iterator<Item = T> + Sized {
    fn equals<I: Iterator<Item = T>>(self, other: I) -> bool {
        self.zip(other).all(|(me, you)| me == you)
    }
}

impl<T: PartialEq, I: Iterator<Item = T> + Sized> EqualExt<T> for I {}
