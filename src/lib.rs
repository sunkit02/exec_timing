use std::{fmt::Display, time::Duration};

pub mod functions;
pub mod timers;

pub use functions::*;
pub use timers::*;

pub trait Timer {
    fn time(&self) -> TimingStats;

    fn time_and_print(&self) {
        println!("{}", self.time());
    }
}

#[derive(Debug, PartialEq)]
pub struct TimingStats {
    label: String,
    runs: usize,
    total_time: Duration,
    mean: Duration,
    max: Duration,
    min: Duration,
}

impl TimingStats {
    pub fn label(&self) -> &str {
        return &self.label;
    }

    pub fn runs(&self) -> usize {
        return self.runs;
    }

    pub fn total_time(&self) -> Duration {
        return self.total_time;
    }

    pub fn mean(&self) -> Duration {
        return self.mean;
    }

    pub fn max(&self) -> Duration {
        return self.max;
    }

    pub fn min(&self) -> Duration {
        return self.min;
    }
}

impl Display for TimingStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ({} runs)\n", self.label, self.runs))?;
        f.write_fmt(format_args!(
            "\t{:>15}: {:>20}\n",
            "total",
            format!("{:?}", self.total_time)
        ))?;
        f.write_fmt(format_args!(
            "\t{:>15}: {:>20}\n",
            "mean",
            format!("{:?}", self.mean)
        ))?;
        f.write_fmt(format_args!(
            "\t{:>15}: {:>20}\n",
            "max",
            format!("{:?}", self.max)
        ))?;
        f.write_fmt(format_args!(
            "\t{:>15}: {:>20}\n",
            "min",
            format!("{:?}", self.min)
        ))
    }
}
