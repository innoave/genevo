
pub mod ga;

use chrono::{DateTime, Duration, Local};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use genetic::{Fitness, FitnessEvaluation, Genotype, Phenotype, Population, Breeding};
use operator::{CrossoverOp, MutationOp, SelectionOp};
use termination::Termination;
use std::marker::PhantomData;
use std::sync::Arc;


/// A `Simulation` is the execution of a genetic algorithm.
pub trait Simulation<T, G, F, E, S, Q, C, M, P>
    where T: Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>, Self: Sized
{
    /// A `SimulationBuilder` that can build this `Simulation`.
    type Builder: SimulationBuilder<Self, T, G, F, E, S, Q, C, M, P>;

    /// Start building a new instance of a `Simulation`.
    fn builder(evaluator: E, selector: S, breeder: C, mutator: M, termination: Q) -> Self::Builder;

    /// Runs this simulation completely. The simulation ends when the
    /// termination criteria are met.
    fn run(&mut self) -> BoxFuture<SimResult<T, G, F>, SimError>;

    /// Makes one step in this simulation. One step in the simulation performs
    /// one time the complete loop of the genetic algorithm.
    fn step(&mut self) -> BoxFuture<SimResult<T, G, F>, SimError>;

    /// Runs the simulation while streaming the results of each step.
    /// The simulation runs without stopping after each step but the
    /// results of each step are provided as a `Stream`.
    fn stream(&mut self) -> BoxStream<SimResult<T, G, F>, SimError>;

    /// Resets the simulation in order to be able to rerun it again. This
    /// method resets the simulation in its initial state, as if it's just
    /// newly created.
    fn reset(&mut self);
}

/// The `SimulationBuilder` creates a new `Simulation` with given parameters
/// and options.
pub trait SimulationBuilder<Sim, T, G, F, E, S, Q, C, M, P>
    where Sim: Simulation<T, G, F, E, S, Q, C, M, P>,
          T: Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    /// Finally initializes the `Simulation` with the given `Population`
    /// and returns the newly created `Simulation`.
    ///
    /// Note: This operation is made the last operation in the chain of
    /// configuration option methods to be able to reuse a previously
    /// configured `SimulationBuilder` with a different initial population.
    fn initialize(&mut self, population: Population<T, G>) -> Sim;
}

/// A `PopulationGenerator` creates a new `Population` with a number of newly
/// created individuals or just individual `Phenotype`s.
///
/// Typically the `PopulationGenerator` is used to create the initial
/// population with randomly created individuals.
pub trait PopulationGenerator<T, G>
    where T: Phenotype<G>, G: Genotype
{
    /// Generates a new `Population` containing the given number of individuals.
    fn generate_population(&self, size: usize) -> Population<T, G> {
        let individuals = (0..size).map(|_| {
            self.generate_phenotype()
        }).collect::<Vec<T>>();
        Population::new(individuals)
    }

    /// Generates a new `Phenotype`.
    ///
    /// An implementation typically generates a randomly created `Phenotype`.
    fn generate_phenotype(&self) -> T;
}

/// The `Evaluated` type marks an individual as evaluated. Mostly this means
/// that the `Fitness` value has been calculated for this individual.
///
/// This structure is used to store the fitness value, so that the fitness
/// value needs to be calculated only one time for each individual. For
/// simulation with more sophisticated fitness calculations this can improve
/// performance.
#[derive(Debug, Eq, PartialEq)]
pub struct Evaluated<T, G, F>
    where T: Phenotype<G>, G: Genotype, F: Fitness
{
    /// The `Phenotype` that has been evaluated.
    pub phenotype: Arc<T>,
    /// The `Fitness` value of the evaluated `Phenotype`.
    pub fitness: F,
    /// The normalized fitness value.
    pub normalized_fitness: F,
    // Needed to calm down the compiler
    _g: PhantomData<G>,
}

impl<T, G, F> Evaluated<T, G, F>
    where T: Phenotype<G>, G: Genotype, F: Fitness
{
    pub fn new(phenotype: Arc<T>, fitness: F, normalized_fitness: F) -> Self {
        Evaluated {
            phenotype: phenotype,
            fitness: fitness,
            normalized_fitness: normalized_fitness,
            _g: PhantomData,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct State<T, G, F>
    where T: Phenotype<G>, G: Genotype, F: Fitness
{
    /// The local time when this simulation started.
    pub started_at: DateTime<Local>,
    /// The number of the generation currently evaluated. Generations are
    /// counted from 1 and increased by 1 each time a new generation is
    /// accepted, e.i. each iteration of the genetic algorithm.
    pub generation: u64,
    /// The population of the current generation.
    pub population: Arc<Vec<G>>,
    /// The fitness values of all individuals of the current population.
    pub fitness_values: Arc<Vec<F>>,
    /// The normalized fitness values of all individuals of the current
    /// population.
    pub normalized_fitness: Arc<Vec<F>>,
    /// Time spent for the current generation.
    pub time: Duration,
    /// Average fitness value of the current generation.
    pub average_fitness: F,
    /// Highest fitness value within the current generation.
    pub highest_fitness: F,
    /// Lowest fitness value within the current generation.
    pub lowest_fitness: F,
    /// Best solution of this generation.
    pub best_solution: Arc<BestSolution<T, G, F>>
}

/// The best solution found by the `Simulation`. If the simulation is not
/// finished this is the best solution of the generation currently evaluated.
/// If the solution is finished this is the overall best solution found by the
/// simulation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BestSolution<T, G, F>
    where T: Phenotype<G>, G: Genotype, F: Fitness
{
    /// The local time at which this solution is found.
    pub found_at: DateTime<Local>,
    /// The number of the generation in which this solution is found.
    pub generation: u64,
    /// The evaluated `Phenotype` that is considered to be best.
    pub solution: Arc<Evaluated<T, G, F>>,
}

/// The result of running a step in the `Simulation`.
#[derive(PartialEq, Eq, Debug)]
pub enum SimResult<T, G, F>
    where T: Phenotype<G>, G: Genotype, F: Fitness
{
    /// The step was successful, but the simulation has not finished.
    Intermediate(Arc<State<T, G, F>>),
    /// The simulation is finished, and this is the final result.
    ///
    /// The `BestSolution` value represents the fittest individual
    /// found during this simulation over all generations.
    Final(Arc<BestSolution<T, G, F>>),
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
