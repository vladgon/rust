use crate::OptionExt;

pub trait IteratorExt<T, I: Iterator<Item=T>> {
    fn tap<F: Fn(&T)>(self, f: F) -> Tap<I, F>;
    fn tap_ignore_result<_T, _E, F: Fn(&T) -> Result<_T, _E>>(self, f: F) -> TapIgnoreResult<I, F>;
}

impl<T, I: Iterator<Item=T>> IteratorExt<T, I> for I {
    fn tap<F>(self, f: F) -> Tap<I, F>
        where Self: Sized + Iterator,
              F: Fn(&T) {
        Tap { iter: self, f }
    }

    fn tap_ignore_result<_T, _E, F>(self, f: F) -> TapIgnoreResult<I, F>
        where Self: Sized + Iterator,
              F: Fn(&T) -> Result<_T, _E> {
        TapIgnoreResult { iter: self, f }
    }
}


pub struct Tap<I, F> {
    iter: I,
    f: F,
}

pub struct TapIgnoreResult<I, F> {
    iter: I,
    f: F,
}

impl<T, I, F> Iterator for Tap<I, F>
    where
        I: Iterator<Item=T>,
        F: Fn(&I::Item) {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().tap(|v| (self.f)(v))
    }
}

impl<T, I, F, _T, _E> Iterator for TapIgnoreResult<I, F>
    where
        I: Iterator<Item=T>,
        F: Fn(&I::Item) -> Result<_T, _E> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().tap_ignore_result(|v| (self.f)(v).ok())
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use anyhow::bail;

    use super::*;

    #[test]
    #[allow(unreachable_code)]
    fn tap() {
        // let counter = RefCell::new(0);
        let counter = AtomicUsize::new(0);
        let max = 5;
        let vec = (0..max)
            .tap(|i| println!("Value {}", i))
            .tap_ignore_result(|_| Some(counter.fetch_add(1, Ordering::Relaxed)).ok_or(""))
            .tap_ignore_result(|i| bail!("Error {i}") as Result<(), _>)
            .collect::<Vec<_>>();
        println!("{:?}", counter);
        assert_eq!(counter.fetch_add(0, Ordering::Relaxed), max);
        assert_eq!(vec.len(), max);
        println!("{:?}", vec);
    }
}