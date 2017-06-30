//! The `statistic` module provides functionality to collect and display
//! statistic about a genetic algorithm application and its execution.

use chrono::{Duration, Local};
use std::ops::{Add, AddAssign};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct ProcessingTime(Duration);

impl ProcessingTime {
    pub fn duration(&self) -> Duration {
        self.0
    }
}

impl Add for ProcessingTime {
    type Output = ProcessingTime;
    fn add(self, other: Self) -> Self::Output {
        ProcessingTime(self.0 + other.0)
    }
}

impl AddAssign for ProcessingTime {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.0
    }
}

impl Debug for ProcessingTime {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for ProcessingTime {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct Timed<F, U> where F: Fn() -> U {
    function: F,
}

pub struct TimedResult<U> {
    pub result: U,
    pub time: ProcessingTime
}

pub fn timed<F, U>(op: F) -> Timed<F, U> where F: Fn() -> U {
    Timed {
        function: op,
    }
}

impl<F, U> Timed<F, U> where F: Fn() -> U {
    pub fn run(&self) -> TimedResult<U> {
        let started_at = Local::now();
        let result = (self.function)();
        let time = Local::now().signed_duration_since(started_at);
        TimedResult {
            result: result,
            time: ProcessingTime(time),
        }
    }
}
