
pub mod simulator;

use algorithm::Algorithm;
use random::Seed;
use statistic::ProcessingTime;
use termination::StopReason;
use chrono::{DateTime, Duration, Local};


/// A `Simulation` is the execution of an algorithm.
pub trait Simulation<A> where A: Algorithm {
    type Error;

    /// Runs this simulation completely. The simulation ends when the
    /// termination criteria are met.
    fn run(&mut self) -> Result<SimResult<A>, Self::Error>;

    /// Makes one step in this simulation. One step in the simulation performs
    /// one time the complete loop of the genetic algorithm.
    fn step(&mut self) -> Result<SimResult<A>, Self::Error>;

    /// Makes one step in this simulation using the given seed. This function
    /// can be used to replay previous simulation steps.
    fn step_with_seed(&mut self, seed: Seed) -> Result<SimResult<A>, Self::Error>;

    /// Stops the simulation after the current loop is finished.
    fn stop(&mut self) -> Result<bool, Self::Error>;

    /// Resets the simulation in order to be able to rerun it again. This
    /// method resets the simulation in its initial state, as if it's just
    /// newly created.
    fn reset(&mut self) -> Result<bool, Self::Error>;
}

/// The `SimulationBuilder` creates a new `Simulation` with given parameters
/// and options. It forms the initialization stage of the algorithm.
pub trait SimulationBuilder<S, A>
    where S: Simulation<A>, A: Algorithm
{
    /// Finally build the Simulation.
    fn build(self) -> S;
}

/// The `State` struct holds the state of the `Simulation`.
#[derive(Debug, PartialEq)]
pub struct State<A> where A: Algorithm {
    /// The local time when this simulation started.
    pub started_at: DateTime<Local>,
    /// The number of the iteration that this state represents. Iterations
    /// are counted from 1 and increased by 1 on each iteration of the
    /// simulation loop.
    pub iteration: u64,
    /// The seed used to generate random values.
    pub seed: Seed,
    /// Duration of processing the current iteration. This is the time it
    /// took to process one iteration of the algorithm.
    pub duration: Duration,
    /// Accumulated time spent by each thread in case of parallel processing.
    /// In case of sequential processing this time is nearly the same as the
    /// `duration` value. In case of parallel processing this time is usually
    /// a multitude of the `duration`.
    pub processing_time: ProcessingTime,
    /// The result of this iteration.
    pub result: <A as Algorithm>::Output,
}

/// The result of running a step in the `Simulation`.
#[derive(Debug, PartialEq)]
pub enum SimResult<A> where A: Algorithm {
    /// The step was successful, but the simulation has not finished.
    ///
    /// The `State` contains the result of the last processed generation.
    Intermediate(State<A>),
    /// The simulation is finished, and this is the final result.
    ///
    /// The parameters are:
    /// * The `State` of last processed generation.
    /// * The total processing time of the simulation.
    /// * The `StopReason` is the matching criteria why the simulation stopped.
    Final(State<A>, ProcessingTime, Duration, StopReason),
}
