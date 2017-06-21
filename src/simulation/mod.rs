
pub mod ga;

use chrono::{DateTime, Duration, Local};
use genetic::{Fitness, FitnessEvaluation, Genotype, Population, Breeding};
use operator::{CrossoverOp, MutationOp, SelectionOp};
use termination::{StopReason, Termination};
use std::rc::Rc;


/// A `Simulation` is the execution of a genetic algorithm.
pub trait Simulation<G, F, E, S, Q, C, M, P>
    where G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, P>, Q: Termination<G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>, Self: Sized
{
    /// A `SimulationBuilder` that can build this `Simulation`.
    type Builder: SimulationBuilder<Self, G, F, E, S, Q, C, M, P>;

    /// Start building a new instance of a `Simulation`.
    fn builder(evaluator: E, selector: S, breeder: C, mutator: M, termination: Q) -> Self::Builder;

    /// Runs this simulation completely. The simulation ends when the
    /// termination criteria are met.
    fn run(&mut self) -> Result<SimResult<G, F>, SimError>;

    /// Makes one step in this simulation. One step in the simulation performs
    /// one time the complete loop of the genetic algorithm.
    fn step(&mut self) -> Result<SimResult<G, F>, SimError>;

    /// Stops the simulation after the current loop is finished.
    fn stop(&mut self) -> Result<bool, SimError>;

    /// Resets the simulation in order to be able to rerun it again. This
    /// method resets the simulation in its initial state, as if it's just
    /// newly created.
    fn reset(&mut self) -> Result<bool, SimError>;
}

/// The `SimulationBuilder` creates a new `Simulation` with given parameters
/// and options. It forms the initialization stage of the genetic algorithm.
pub trait SimulationBuilder<Sim, G, F, E, S, Q, C, M, P>
    where Sim: Simulation<G, F, E, S, Q, C, M, P>,
          G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, P>, Q: Termination<G, F>,
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
    /// The `Fitness` value of the evaluated `Genotype`.
    pub fitness: F,
    /// The normalized fitness value.
    pub normalized_fitness: F,
}

/// The `EvaluatedPopulation` holds the results of the evaluation stage of
/// the genetic algorithm. It is used to pass these values to the
/// `operator::SelectionOp` for enable this operator to do its job.
///
/// Currently is contains the fitness value of each individual in a population,
/// their normalized fitness values and highest and average fitness of the
/// population.
///
/// As the information in this struct is only used to pass the output of the
/// evaluation stage to the selection operator and this happens once for every
/// population the types of the fields are designed to avoid cloning of whole
/// data structures. To be able to change the fields internally later when
/// new optimization are found the fields are kept private.
#[derive(Debug, Eq, PartialEq)]
pub struct EvaluatedPopulation<G, F>
    where G: Genotype, F: Fitness
{
    individuals: Rc<Vec<G>>,
    fitness_values: Vec<F>,
    normalized_fitness: Vec<F>,
    highest_fitness: F,
    lowest_fitness: F,
    average_fitness: F,
}

impl<G, F> EvaluatedPopulation<G, F>
    where G: Genotype, F: Fitness
{
    /// Construct a new instance of the `EvaluatedPopulation` struct.
    pub fn new(individuals: Rc<Vec<G>>,
               fitness_values: Vec<F>,
               normalized_fitness: Vec<F>,
               highest_fitness: F,
               lowest_fitness: F,
               average_fitness: F
            ) -> EvaluatedPopulation<G, F> {
        EvaluatedPopulation {
            individuals: individuals,
            fitness_values: fitness_values,
            normalized_fitness: normalized_fitness,
            highest_fitness: highest_fitness,
            lowest_fitness: lowest_fitness,
            average_fitness: average_fitness
        }
    }

    /// Returns the individuals of the population that has been evaluated.
    pub fn individuals(&self) -> Rc<Vec<G>> {
        self.individuals.clone()
    }

    /// Returns the fitness values of all individuals of the evaluated
    /// population.
    ///
    /// The returned slice contains the fitness values of the individuals
    /// in the same order as the slice returned by function `individuals`
    /// contains the individuals itself, i.e. for individual with index `i`
    /// in `individuals()[i]` the fitness value is stored in
    /// `fitness_values()[i]`.
    pub fn fitness_values(&self) -> &[F] {
        &self.fitness_values
    }

    /// Returns the normalized fitness values of all individuals of the
    /// evaluated population.
    ///
    /// The returned slice contains the normalized fitness values
    /// in the same order as the slice returned by function `individuals`
    /// contains the individuals itself, i.e. for individual with index `i`
    /// in `individuals()[i]` the normalized fitness value is stored in
    /// `normalized_fitness()[i]`.
    pub fn normalized_fitness(&self) -> &[F] {
        &self.normalized_fitness
    }

    /// Returns the highest `Fitness` value found in the evaluated population.
    pub fn highest_fitness(&self) -> &F {
        &self.highest_fitness
    }

    /// Returns the lowest `Fitness` value found in the evaluated population.
    pub fn lowest_fitness(&self) -> &F {
        &self.lowest_fitness
    }

    /// Returns the average of all `Fitness` values of the evaluated
    /// population.
    pub fn average_fitness(&self) -> &F {
        &self.average_fitness
    }

    /// Returns the individual at the given index.
    pub fn individual(&self, index: usize) -> Option<&G> {
        self.individuals.get(index)
    }

    /// Returns the `Fitness` value of the given individual.
    ///
    /// Note: This function might be more expensive due to the data structure
    /// chosen for this struct. So use it sparingly.
    pub fn fitness_of_individual(&self, individual: &G) -> Option<&F> {
        self.index_of_individual(individual).map(|index|
            &self.fitness_values[index])
    }

    /// Returns the normalized `Fitness` value of the given individual.
    ///
    /// Note: This function might be more expensive due to the data structure
    /// chosen for this struct. So use it sparingly.
    pub fn normalized_fitness_of_individual(&self, individual: &G) -> Option<&F> {
        self.index_of_individual(individual).map(|index|
            &self.fitness_values[index])
    }

    /// Returns the `Genotype` of the individual with a given `Fitness` value.
    ///
    /// Note: This function might be more expensive due to the data structure
    /// chosen for this struct. So use it sparingly.
    pub fn individual_with_fitness(&self, fitness: &F) -> Option<&G> {
        self.index_of_fitness(fitness).map(|index|
            &self.individuals[index])
    }

    /// Determines the index in the `individuals` slice of an individual.
    fn index_of_individual(&self, individual: &G) -> Option<usize> {
        self.individuals.iter().position(|v| *v == *individual)
    }

    /// Determines the index in the `fitness_values` slice of a fitness value.
    fn index_of_fitness(&self, fitness: &F) -> Option<usize> {
        self.fitness_values.iter().position(|v| *v == *fitness)
    }

}

/// The `State` struct holds the results of one pass of the genetic algorithm
/// loop, i.e. the processing of the loop for one generation.
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
    /// The evaluated `Genotype` that is considered to be best.
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
    /// The (initial) population is smaller than the required minimum.
    PopulationTooSmall(String),
    /// It has been tried to call run, step or stream while the simulation
    /// is already running. E.g. the step method has been called and now step,
    /// run or stream is called before the simulation of the previous step is
    /// finished.
    SimulationAlreadyRunning(String),
    /// An unexpected error occurred. This error should be used only in cases
    /// that theoretically are possible in the code but are avoided through
    /// other mains on the domain layer.
    UnexpectedError(String),
}
