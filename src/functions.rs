use std::time::{Duration, Instant};

use crate::TimingStats;

pub fn time<F>(label: &str, runs: usize, time_fn: F) -> TimingStats
where
    F: Fn(),
{
    let label = label.to_string();

    let mut times = Vec::with_capacity(runs);

    (0..runs).for_each(|_| {
        let start = Instant::now();
        time_fn();
        let time = Instant::now() - start;
        times.push(time);
    });

    let total_time: Duration = times.iter().sum();
    let mean = total_time.div_f64(runs as f64);
    let max = *times.iter().max().unwrap();
    let min = *times.iter().min().unwrap();

    return TimingStats {
        label,
        runs,
        total_time,
        mean,
        max,
        min,
    };
}

pub fn time_with_args<F, ArgsFn, Args>(
    label: &str,
    runs: usize,
    args: ArgsFn,
    time_fn: F,
) -> TimingStats
where
    F: Fn(Args),
    ArgsFn: Fn() -> Args,
{
    let label = label.to_string();

    let mut times = Vec::with_capacity(runs);

    (0..runs).for_each(|_| {
        let start = Instant::now();
        time_fn(args());
        let time = Instant::now() - start;
        times.push(time);
    });

    let total_time: Duration = times.iter().sum();
    let mean = total_time.div_f64(runs as f64);
    let max = *times.iter().max().unwrap();
    let min = *times.iter().min().unwrap();

    return TimingStats {
        label,
        runs,
        total_time,
        mean,
        max,
        min,
    };
}
