use crate::{
    algorithm::Algorithm,
    random::{get_rng, random_seed, Seed},
    simulation::{SimResult, Simulation, SimulationBuilder, State},
    statistic::{ProcessingTime, TrackProcessingTime},
    termination::{StopFlag, Termination},
};
use chrono::{DateTime, Local};
use std::{
    error::Error,
    fmt::{self, Debug, Display},
    hash::Hash,
};

/// The `simulate` function creates a new `Simulator` for the given
/// `algorithm::Algorithm`.
pub fn simulate<A>(algorithm: A) -> SimulatorBuilderWithAlgorithm<A>
where
    A: Algorithm,
{
    SimulatorBuilderWithAlgorithm { algorithm }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatorBuilder<A, T>
where
    A: Algorithm,
    T: Termination<A>,
{
    algorithm: A,
    termination: T,
}

impl<A, T> SimulationBuilder<Simulator<A, T>, A> for SimulatorBuilder<A, T>
where
    A: Algorithm + TrackProcessingTime + Debug,
    <A as Algorithm>::Error: Eq + Hash + Display + Send + Sync,
    T: Termination<A>,
{
    fn build(self) -> Simulator<A, T> {
        Simulator {
            algorithm: self.algorithm,
            termination: self.termination,
            run_mode: RunMode::NotRunning,
            started_at: Local::now(),
            iteration: 0,
            processing_time: ProcessingTime::zero(),
            finished: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatorBuilderWithAlgorithm<A>
where
    A: Algorithm,
{
    algorithm: A,
}

impl<A> SimulatorBuilderWithAlgorithm<A>
where
    A: Algorithm,
{
    pub fn until<T>(self, termination: T) -> SimulatorBuilder<A, T>
    where
        T: Termination<A>,
    {
        SimulatorBuilder {
            algorithm: self.algorithm,
            termination,
        }
    }
}

/// The `RunMode` identifies whether the simulation is running and how it has
/// been started.
#[derive(Clone, Debug, PartialEq)]
enum RunMode {
    /// The simulation is running in loop mode. i.e. it was started by calling
    /// the `run` function.
    Loop,
    /// The simulation is running in step mode. i.e. it was started by calling
    /// the `step` function.
    Step,
    /// The simulation is not running.
    NotRunning,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SimError<A>
where
    A: Algorithm + Debug,
    <A as Algorithm>::Error: Eq + Hash + Debug,
{
    AlgorithmError(<A as Algorithm>::Error),
    SimulationAlreadyRunning(String),
    Unexpected(String),
}

impl<A> Display for SimError<A>
where
    A: Algorithm + Debug,
    <A as Algorithm>::Error: Eq + Hash + Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SimError::AlgorithmError(ref error) => write!(f, "algorithm error: {}", error),
            SimError::SimulationAlreadyRunning(ref message) => {
                write!(f, "simulation already running: {}", message)
            }
            SimError::Unexpected(ref message) => write!(f, "unexpected error: {}", message),
        }
    }
}

impl<A> Error for SimError<A>
where
    A: Algorithm + Debug,
    <A as Algorithm>::Error: 'static + Eq + Hash + Debug + Display,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SimError::AlgorithmError(ref error) => Some(error),
            SimError::SimulationAlreadyRunning(_) => None,
            SimError::Unexpected(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Simulator<A, T>
where
    A: Algorithm,
    T: Termination<A>,
{
    algorithm: A,
    termination: T,
    run_mode: RunMode,
    started_at: DateTime<Local>,
    iteration: u64,
    processing_time: ProcessingTime,
    finished: bool,
}

impl<A, T> Simulator<A, T>
where
    A: Algorithm + TrackProcessingTime + Debug,
    <A as Algorithm>::Error: Eq + Hash + Display + Send + Sync,
    T: Termination<A>,
{
    pub fn termination(&self) -> &T {
        &self.termination
    }

    /// Processes one iteration of the algorithm used in this simulation.
    fn process_one_iteration(
        &mut self,
        seed: Seed,
    ) -> Result<State<A>, <Self as Simulation<A>>::Error> {
        let loop_started_at = Local::now();

        self.iteration += 1;
        let result = self.algorithm.next(self.iteration, &mut get_rng(seed));

        let loop_duration = Local::now().signed_duration_since(loop_started_at);
        match result {
            Ok(result) => Ok(State {
                started_at: self.started_at,
                iteration: self.iteration,
                seed,
                duration: loop_duration,
                processing_time: *self.algorithm.processing_time(),
                result,
            }),
            Err(error) => Err(SimError::AlgorithmError(error)),
        }
    }
}

impl<A, T> Simulation<A> for Simulator<A, T>
where
    A: Algorithm + TrackProcessingTime + Debug,
    <A as Algorithm>::Error: Eq + Hash + Display + Send + Sync,
    T: Termination<A>,
{
    type Error = SimError<A>;

    fn run(&mut self) -> Result<SimResult<A>, Self::Error> {
        match self.run_mode {
            RunMode::Loop => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "in loop mode since {}",
                    &self.started_at
                )))
            }
            RunMode::Step => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "in step mode since {}",
                    &self.started_at
                )))
            }
            RunMode::NotRunning => {
                self.run_mode = RunMode::Loop;
                self.started_at = Local::now();
            }
        }
        let mut result = Err(SimError::Unexpected(format!(
            "no loop of the simulation has ever been processed!"
        )));
        self.finished = false;
        while !self.finished {
            result = self
                .process_one_iteration(random_seed())
                .and_then(|state| {
                    // Stage 5: Be aware of the termination:
                    Ok(match self.termination.evaluate(&state) {
                        StopFlag::Continue => SimResult::Intermediate(state),
                        StopFlag::StopNow(reason) => {
                            self.finished = true;
                            let processing_time = self.processing_time;
                            let duration = Local::now().signed_duration_since(self.started_at);
                            SimResult::Final(state, processing_time, duration, reason)
                        }
                    })
                })
                .or_else(|error| {
                    self.finished = true;
                    Err(error)
                });
        }
        self.run_mode = RunMode::NotRunning;
        result
    }

    fn step(&mut self) -> Result<SimResult<A>, Self::Error> {
        self.step_with_seed(random_seed())
    }

    fn step_with_seed(&mut self, seed: Seed) -> Result<SimResult<A>, Self::Error> {
        match self.run_mode {
            RunMode::Loop => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "in loop mode since {}",
                    &self.started_at
                )))
            }
            RunMode::Step => (),
            RunMode::NotRunning => {
                self.run_mode = RunMode::Step;
                self.started_at = Local::now();
            }
        }

        self.process_one_iteration(seed).and_then(|state|

            // Stage 5: Be aware of the termination:
            Ok(match self.termination.evaluate(&state) {
                StopFlag::Continue => {
                    SimResult::Intermediate(state)
                },
                StopFlag::StopNow(reason) => {
                    let processing_time = self.processing_time;
                    let duration = Local::now().signed_duration_since(self.started_at);
                    self.run_mode = RunMode::NotRunning;
                    SimResult::Final(state, processing_time, duration, reason)
                },
            }))
    }

    fn stop(&mut self) -> Result<bool, Self::Error> {
        match self.run_mode {
            RunMode::Loop | RunMode::Step => {
                self.finished = true;
                Ok(true)
            }
            RunMode::NotRunning => Ok(false),
        }
    }

    fn reset(&mut self) -> Result<bool, Self::Error> {
        match self.run_mode {
            RunMode::Loop => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "Simulation still running in loop mode since {}. Wait for the \
                     simulation to finish or stop it before resetting it.",
                    &self.started_at
                )))
            }
            RunMode::Step => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "Simulation still running in step mode since {}. Wait for the \
                     simulation to finish or stop it before resetting it.",
                    &self.started_at
                )))
            }
            RunMode::NotRunning => (),
        }
        self.run_mode = RunMode::NotRunning;
        self.processing_time = ProcessingTime::zero();
        self.iteration = 0;
        self.algorithm.reset().map_err(SimError::AlgorithmError)
    }
}
