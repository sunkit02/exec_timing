use crate::{time, TimingStats};

pub trait IteratorTimingExt<I: Iterator = Self>: Iterator {
    fn time<F, O>(&mut self, label: &str, time_fn: F) -> TimingStats
    where
        F: Fn(<Self as Iterator>::Item) -> O,
    {
        time(label, 1, || {
            while let Some(e) = self.next() {
                time_fn(e);
            }
        })
    }
}

impl<T> IteratorTimingExt for T where T: Iterator {}
