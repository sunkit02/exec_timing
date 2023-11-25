use std::{time::{Duration, Instant}, fmt::Display};

const DEFAULT_RUNS: usize = 1;
const DEFAULT_LABEL: &str = "No Label";

#[derive(Debug, PartialEq)]
pub struct TimerBuilder {
    label: Option<String>,
    runs: Option<usize>,
}

impl TimerBuilder {
    pub fn new() -> Self {
        return Self { label: None, runs: None };
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        return self;
    }

    pub fn runs(mut self, runs: usize) -> Self {
        self.runs = Some(runs);
        return self;
    }

    pub fn args_fn<ArgsFn, Args>(
        self,
        args_f: ArgsFn
    ) -> ArgsTimerBuilder<ArgsFn, Args> 
    where 
        ArgsFn: Fn() -> Args
    {
        return ArgsTimerBuilder { 
            label: self.label,
            runs: self.runs.unwrap_or(DEFAULT_RUNS),
            args_f,
        };
    }

    pub fn time_fn<F: Fn()>(self, time_fn: F) -> NoArgsTimer<F> {
        return NoArgsTimer { 
            label: self.label,
            runs: self.runs.unwrap_or(DEFAULT_RUNS),
            time_fn,
        };
    }
}

pub struct ArgsTimerBuilder<ArgsFn, Args>
where
    ArgsFn: Fn() -> Args,
{
    label: Option<String>,
    runs: usize,
    args_f: ArgsFn,
}

impl<ArgsFn, Args> ArgsTimerBuilder<ArgsFn, Args>
where
    ArgsFn: Fn() -> Args,
{
    pub fn time_fn<F>(self, time_fn: F) -> ArgsTimer<F, ArgsFn, Args>
    where
        F: Fn(Args),
    {
        ArgsTimer { 
            label: self.label,
            time_fn,
            args_fn: self.args_f,
            runs: self.runs 
        }
    }
}

pub trait Timer {
    fn time(&self) -> TimingStats;
}

#[derive(Debug, PartialEq)]
pub struct ArgsTimer<F, ArgsFn, Args>
where
    F: Fn(Args),
    ArgsFn: Fn() -> Args
{
    label: Option<String>,
    runs: usize,
    time_fn: F,
    args_fn: ArgsFn,
}

impl<F, ArgsFn, Args> Timer for ArgsTimer<F, ArgsFn, Args>
where
    F: Fn(Args),
    ArgsFn: Fn() -> Args
{
    fn time(&self) -> TimingStats {
        let label = self.label.clone()
            .unwrap_or_else(|| DEFAULT_LABEL.to_string());

        let mut times = Vec::with_capacity(self.runs);

        for _run in 0..self.runs {
            let start = Instant::now();
            (self.time_fn)((self.args_fn)());
            let time = Instant::now() - start;
            times.push(time);
        }

        let total: Duration = times.iter().sum();
        let mean = total.div_f64(self.runs as f64);
        let max = *times.iter().max().unwrap();
        let min = *times.iter().min().unwrap();

        println!("Finished {}...", label);

        return TimingStats { 
            label,
            runs: self.runs,
            total_time: total,
            mean, 
            max,
            min 
        };
    }
}

pub struct NoArgsTimer<F: Fn()> {
    label: Option<String>,
    runs: usize,
    time_fn: F,
}

impl<F: Fn()> Timer for NoArgsTimer<F> {
    fn time(&self) -> TimingStats {
        let label = self.label.clone()
            .unwrap_or_else(|| DEFAULT_LABEL.to_string());

        let mut times = Vec::with_capacity(self.runs);

        for _run in 0..self.runs {
            let start = Instant::now();
            (self.time_fn)();
            let time = Instant::now() - start;
            times.push(time);
        }

        let total: Duration = times.iter().sum();
        let mean = total.div_f64(self.runs as f64);
        let max = *times.iter().max().unwrap();
        let min = *times.iter().min().unwrap();

        println!("Finished {}...", label);

        return TimingStats { 
            label,
            runs: self.runs,
            total_time: total,
            mean, 
            max,
            min 
        };
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

impl Display for TimingStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ({} runs)\n", self.label, self.runs))?;
        f.write_fmt(format_args!("\t{:>15}: {:>20}\n", "total", format!("{:?}", self.total_time)))?;
        f.write_fmt(format_args!("\t{:>15}: {:>20}\n", "mean", format!("{:?}", self.mean)))?;
        f.write_fmt(format_args!("\t{:>15}: {:>20}\n", "max", format!("{:?}", self.max)))?;
        f.write_fmt(format_args!("\t{:>15}: {:>20}\n", "min", format!("{:?}", self.min)))
    }
}


#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    #[test]
    fn can_build_no_args_timer() {
        let label = "Test NoArgs Timer";
        let ms_wait_per_run = 5;
        let time_fn = || { thread::sleep(Duration::from_millis(ms_wait_per_run)); };
        let runs = 10;

        let stats = TimerBuilder::new()
            .with_label(label)
            .runs(runs)
            .time_fn(time_fn)
            .time();

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
    fn can_build_args_timer() {
        let label = "Test Args Timer";
        let ms_wait_per_run = 5;
        let args_fn = || { ms_wait_per_run };
        let time_fn = |args| { thread::sleep(Duration::from_millis(args)) };
        let runs = 10;

        let stats = TimerBuilder::new()
            .with_label(label)
            .runs(runs)
            .args_fn(args_fn)
            .time_fn(time_fn)
            .time();

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
