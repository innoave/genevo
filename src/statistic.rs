//! The `statistic` module provides functionality to collect and display
//! statistic about a genetic algorithm application and its execution.

use types::fmt::Display;
use chrono::{Duration, Local};
use std::ops::{Add, AddAssign};
use std::convert::From;
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct ProcessingTime {
    duration: Duration,
}

impl ProcessingTime {
    pub fn zero() -> Self {
        ProcessingTime {
            duration: Duration::zero(),
        }
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }
}

impl From<Duration> for ProcessingTime {
    fn from(duration: Duration) -> Self {
        ProcessingTime {
            duration: duration,
        }
    }
}

impl fmt::Debug for ProcessingTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.duration, f)
    }
}

impl fmt::Display for ProcessingTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.duration, f)
    }
}

impl Display for ProcessingTime {
    fn fmt(&self) -> String {
        self.duration.fmt()
    }
}

impl Add for ProcessingTime {
    type Output = ProcessingTime;
    fn add(self, other: Self) -> Self::Output {
        ProcessingTime::from(self.duration + other.duration)
    }
}

impl AddAssign for ProcessingTime {
    fn add_assign(&mut self, other: Self) {
        self.duration = self.duration + other.duration
    }
}

pub trait TrackProcessingTime {
    fn processing_time(&self) -> &ProcessingTime;
}

pub struct TimedResult<U> {
    pub result: U,
    pub time: ProcessingTime,
}

pub fn timed<F, U>(op: F) -> TimedFn<F, U> where F: FnOnce() -> U {
    TimedFn {
        function: op,
    }
}

pub struct TimedFn<F, U> where F: FnOnce() -> U {
    function: F,
}

impl<F, U> TimedFn<F, U> where F: FnOnce() -> U {
    pub fn run(self) -> TimedResult<U> {
        let started_at = Local::now();
        let result = (self.function)();
        let time = Local::now().signed_duration_since(started_at);
        TimedResult {
            result: result,
            time: ProcessingTime::from(time),
        }
    }
}
