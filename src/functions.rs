use std::time::{Duration, Instant};

use crate::TimingStats;

pub fn time<F, O>(label: &str, runs: usize, mut time_fn: F) -> TimingStats
where
    F: FnMut() -> O,
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

pub fn time_with_args<F, ArgsFn, Args, O>(
    label: &str,
    runs: usize,
    args_fn: ArgsFn,
    mut time_fn: F,
) -> TimingStats
where
    F: FnMut(Args) -> O,
    ArgsFn: Fn() -> Args,
{
    let label = label.to_string();

    let mut times = Vec::with_capacity(runs);

    (0..runs).for_each(|_| {
        let start = Instant::now();
        time_fn(args_fn());
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

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    #[test]
    fn can_time_with_no_args() {
        let label = "Test NoArgs Timer";
        let ms_wait_per_run = 5;
        let time_fn = || {
            thread::sleep(Duration::from_millis(ms_wait_per_run));
        };
        let runs = 10;

        let stats = time(label, runs, time_fn);

        dbg!(&stats);

        assert_eq!(stats.label, label);
        assert_eq!(stats.runs, runs);

        // Values of `max`, `min`, and `mean` should be at least as long as `ms_wait_per_run`
        // Value of `total_time` should be at least `ms_wait_per_run` * `runs`
        // Can be slightly longer due to CPU scheduling
        assert!(stats.total_time >= Duration::from_millis(ms_wait_per_run * runs as u64));
        assert!(stats.max >= Duration::from_millis(ms_wait_per_run));
        assert!(stats.min >= Duration::from_millis(ms_wait_per_run));
        assert!(stats.mean >= Duration::from_millis(ms_wait_per_run));
    }

    #[test]
    fn can_time_with_args() {
        let label = "Test Args Timer";
        let ms_wait_per_run = 5;
        let args_fn = || ms_wait_per_run;
        let time_fn = |args| thread::sleep(Duration::from_millis(args));
        let runs = 10;

        let stats = time_with_args(label, runs, args_fn, time_fn);

        dbg!(&stats);

        assert_eq!(stats.label, label);
        assert_eq!(stats.runs, runs);

        // Values of `max`, `min`, and `mean` should be at least as long as `ms_wait_per_run`
        // Value of `total_time` should be at least `ms_wait_per_run` * `runs`
        // Can be slightly longer due to CPU scheduling
        assert!(stats.total_time >= Duration::from_millis(ms_wait_per_run * runs as u64));
        assert!(stats.max >= Duration::from_millis(ms_wait_per_run));
        assert!(stats.min >= Duration::from_millis(ms_wait_per_run));
        assert!(stats.mean >= Duration::from_millis(ms_wait_per_run));
    }
}
