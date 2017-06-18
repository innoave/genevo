
pub mod ga;

use chrono::{DateTime, Duration, Local};
use futures::{Future, Stream};
use genetic::{Fitness, FitnessEvaluation, Genotype, Phenotype, Population, Breeding};
use operator::{CrossoverOp, MutationOp, SelectionOp};
use termination::Termination;
use std::marker::PhantomData;


/// A `Simulation` is the execution of a genetic algorithm.
pub trait Simulation<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    /// Start building a new instance of a `Simulation`.
    fn builder<B>(evaluator: E, selector: S, breeder: C, mutator: M, termination: Q) -> B
        where B: SimulationBuilder<'a, Self, T, G, F, E, S, Q, C, M, P>, Self: Sized;

    /// Runs this simulation completely.
    fn run(&mut self) -> Future<Item=Result<'a, T, G, F>, Error=Error>;

    /// Makes one step in this simulation.
    fn step(&mut self) -> Future<Item=Result<'a, T, G, F>, Error=Error>;

    /// Runs the simulation while streaming the results of each step.
    /// The simulation runs without stopping after each step but the
    /// results of each step are provided as a `Stream`.
    fn stream(&mut self) -> Stream<Item=Result<'a, T, G, F>, Error=Error>;

    /// Resets the simulation to rerun it again. This methods resets the
    /// simulation in its initial state, as if its just newly created.
    fn reset(&mut self);

    //TODO should we have statistics? what should they be?
    // Returns the `SimStatistics` of the last run of the simulation.
//    fn statistics(&self) -> SimStatistics;
}

/// The `SimulationBuilder` creates a new `Simulation` with given parameters
/// and options.
pub trait SimulationBuilder<'a, Sim, T, G, F, E, S, Q, C, M, P>
    where Sim: Simulation<'a, T, G, F, E, S, Q, C, M, P>,
          T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    /// Finally initializes the `Simulation` with the given `Population`
    /// and returns the newly created `Simulation`.
    ///
    /// Note: This operation is made the last operation in the chain of
    /// configuration option methods to be able to reuse a previously
    /// configured `SimulationBuilder` with a different initial population.
    fn initialize(&self, population: Population<T, G>) -> Sim;
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
pub struct Evaluated<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// The `Phenotype` that has been evaluated.
    phenotype: &'a T,
    /// The `Fitness` value of the evaluated `Phenotype`.
    fitness: F,
    // Needed to calm down the compiler ;-)
    phantom_type: PhantomData<G>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct State<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// The local time when this simulation started.
    pub started_at: DateTime<Local>,
    /// The number of the generation currently evaluated.
    pub generation: u64,
    /// Time spent for the current generation.
    pub time: Duration,
    /// Average fitness value of the current generation.
    pub average_fitness: F,
    /// Best solution of this generation.
    pub best_solution: BestSolution<'a, T, G, F>
}

/// The best solution found by the `Simulation`. If the simulation is not
/// finished this is the best solution of the generation currently evaluated.
/// If the solution is finished this is the overall best solution found by the
/// simulation.
#[derive(Debug, Eq, PartialEq)]
pub struct BestSolution<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// The local time at which this solution is found.
    found_at: DateTime<Local>,
    /// The number of the generation in which this solution is found.
    generation: u64,
    /// The evaluated `Phenotype` that is considered to be best.
    solution: Evaluated<'a, T, G, F>,
}

/// The result of running a step in the `Simulation`.
#[derive(PartialEq, Eq, Debug)]
pub enum Result<'a, T, G, F>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    /// The step was successful, but the simulation has not finished.
    Intermediate(State<'a, T, G, F>),
    /// The simulation is finished, and this is the final result.
    ///
    /// The `BestSolution` value represents the fittest individual
    /// found during this simulation over all generations.
    Final(BestSolution<'a, T, G, F>),
}

/// An error occurred during `Simulation`.
pub enum Error<'a> {
    /// The simulation has been created with an empty population.
    EmptyPopulation(&'a str),
    /// It has been tried to call run, step or stream while the simulation
    /// is already running. E.g. the step method has been called and now step,
    /// run or stream is called before the simulation of the previous step is
    /// finished.
    SimulationAlreadyRunning(&'a str),
}
