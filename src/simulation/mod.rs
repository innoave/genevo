
pub mod ga;

use chrono::{DateTime, Duration, Local};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use genetic::{Fitness, FitnessEvaluation, Genotype, Population, Breeding};
use operator::{CrossoverOp, MutationOp, SelectionOp};
use termination::{StopReason, Termination};


/// A `Simulation` is the execution of a genetic algorithm.
pub trait Simulation<G, F, E, S, Q, C, M, P>
    where G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>, Self: Sized
{
    /// A `SimulationBuilder` that can build this `Simulation`.
    type Builder: SimulationBuilder<Self, G, F, E, S, Q, C, M, P>;

    /// Start building a new instance of a `Simulation`.
    fn builder(evaluator: E, selector: S, breeder: C, mutator: M, termination: Q) -> Self::Builder;

    /// Runs this simulation completely. The simulation ends when the
    /// termination criteria are met.
    fn run(&mut self) -> BoxFuture<SimResult<G, F>, SimError>;

    /// Makes one step in this simulation. One step in the simulation performs
    /// one time the complete loop of the genetic algorithm.
    fn step(&mut self) -> BoxFuture<SimResult<G, F>, SimError>;

    /// Runs the simulation while streaming the results of each step.
    /// The simulation runs without stopping after each step but the
    /// results of each step are provided as a `Stream`.
    fn stream(&mut self) -> BoxStream<SimResult<G, F>, SimError>;

    /// Resets the simulation in order to be able to rerun it again. This
    /// method resets the simulation in its initial state, as if it's just
    /// newly created.
    fn reset(&mut self);
}

/// The `SimulationBuilder` creates a new `Simulation` with given parameters
/// and options. It forms the initialization stage of the genetic algorithm.
pub trait SimulationBuilder<Sim, G, F, E, S, Q, C, M, P>
    where Sim: Simulation<G, F, E, S, Q, C, M, P>,
          G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    /// Finally initializes the `Simulation` with the given `Population`
    /// and returns the newly created `Simulation`.
    ///
    /// Note: This operation is made the last operation in the chain of
    /// configuration option methods to be able to reuse a previously
    /// configured `SimulationBuilder` with a different initial population.
    fn initialize(&mut self, population: Population<G>) -> Sim;
}

/// The `Evaluated` type marks an individual as evaluated. Mostly this means
/// that the `Fitness` value has been calculated for this individual.
///
/// This structure is used to store the fitness value, so that the fitness
/// value needs to be calculated only one time for each individual. For
/// simulation with more sophisticated fitness calculations this can improve
/// performance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evaluated<G, F>
    where G: Genotype, F: Fitness
{
    /// The `Genotype` that has been evaluated.
    pub genome: G,
    /// The `Fitness` value of the evaluated `Phenotype`.
    pub fitness: F,
    /// The normalized fitness value.
    pub normalized_fitness: F,
}

#[derive(Debug, Eq, PartialEq)]
pub struct State<G, F>
    where G: Genotype, F: Fitness
{
    /// The local time when this simulation started.
    pub started_at: DateTime<Local>,
    /// The number of the generation currently evaluated. Generations are
    /// counted from 1 and increased by 1 each time a new generation is
    /// accepted, e.i. each iteration of the genetic algorithm.
    pub generation: u64,
    /// The population of the current generation.
    pub population: Vec<G>,
    /// The fitness values of all individuals of the current population.
    pub fitness_values: Vec<F>,
    /// The normalized fitness values of all individuals of the current
    /// population.
    pub normalized_fitness: Vec<F>,
    /// Duration of processing the current generation.
    pub duration: Duration,
    /// Accumulated time spent by each thread in case of parallel processing.
    pub processing_time: Duration,
    /// Average fitness value of the current generation.
    pub average_fitness: F,
    /// Highest fitness value within the current generation.
    pub highest_fitness: F,
    /// Lowest fitness value within the current generation.
    pub lowest_fitness: F,
    /// Best solution of this generation.
    pub best_solution: BestSolution<G, F>
}

/// The best solution found by the `Simulation`. If the simulation is not
/// finished this is the best solution of the generation currently evaluated.
/// If the solution is finished this is the overall best solution found by the
/// simulation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BestSolution<G, F>
    where G: Genotype, F: Fitness
{
    /// The local time at which this solution is found.
    pub found_at: DateTime<Local>,
    /// The number of the generation in which this solution is found.
    pub generation: u64,
    /// The evaluated `Phenotype` that is considered to be best.
    pub solution: Evaluated<G, F>,
}

/// The result of running a step in the `Simulation`.
#[derive(PartialEq, Eq, Debug)]
pub enum SimResult<G, F>
    where G: Genotype, F: Fitness
{
    /// The step was successful, but the simulation has not finished.
    ///
    /// The `State` contains the result of the last processed generation.
    Intermediate(State<G, F>),
    /// The simulation is finished, and this is the final result.
    ///
    /// The parameters are:
    /// * The `State` of last processed generation.
    /// * The total processing time of the simulation.
    /// * The `StopReason` is the matching criteria why the simulation stopped.
    Final(State<G, F>, Duration, StopReason),
}

/// An error occurred during `Simulation`.
pub enum SimError {
    /// The simulation has been created with an empty population.
    EmptyPopulation(String),
    /// It has been tried to call run, step or stream while the simulation
    /// is already running. E.g. the step method has been called and now step,
    /// run or stream is called before the simulation of the previous step is
    /// finished.
    SimulationAlreadyRunning(String),
}
