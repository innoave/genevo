
pub mod ga;

use algorithm::Algorithm;
use genetic::{Fitness, Genotype};
use random::Seed;
use termination::StopReason;
use chrono::{DateTime, Duration, Local};
use std::rc::Rc;


/// A `Simulation` is the execution of an algorithm.
pub trait Simulation<A, G, F>
    where A: Algorithm, G: Genotype, F: Fitness
{
    /// Runs this simulation completely. The simulation ends when the
    /// termination criteria are met.
    fn run(&mut self) -> Result<SimResult<G, F>, SimError>;

    /// Makes one step in this simulation. One step in the simulation performs
    /// one time the complete loop of the genetic algorithm.
    fn step(&mut self) -> Result<SimResult<G, F>, SimError>;

    /// Makes one step in this simulation using the given seed. This function
    /// can be used to replay previous simulation steps.
    fn step_with_seed(&mut self, seed: Seed) -> Result<SimResult<G, F>, SimError>;

    /// Stops the simulation after the current loop is finished.
    fn stop(&mut self) -> Result<bool, SimError>;

    /// Resets the simulation in order to be able to rerun it again. This
    /// method resets the simulation in its initial state, as if it's just
    /// newly created.
    fn reset(&mut self) -> Result<bool, SimError>;
}

/// The `SimulationBuilder` creates a new `Simulation` with given parameters
/// and options. It forms the initialization stage of the algorithm.
pub trait SimulationBuilder<S, A, G, F>
    where S: Simulation<A, G, F>, A: Algorithm, G: Genotype, F: Fitness
{
    /// Finally build the Simulation.
    fn build(self) -> S;
}

/// The `Evaluated` type marks an individual as evaluated. Mostly this means
/// that the `genetic::Fitness` value has been calculated for this individual.
///
/// This structure is used to store the fitness value, so that the fitness
/// value needs to be calculated only one time for each individual. For
/// simulation with more sophisticated fitness calculations this can improve
/// performance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evaluated<G, F>
    where G: Genotype, F: Fitness
{
    /// The `genetic::Genotype` that has been evaluated.
    pub genome: G,
    /// The `genetic::Fitness` value of the evaluated `genetic::Genotype`.
    pub fitness: F,
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
               highest_fitness: F,
               lowest_fitness: F,
               average_fitness: F
            ) -> Self {
        EvaluatedPopulation {
            individuals: individuals,
            fitness_values: fitness_values,
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

    /// Returns the highest `genetic::Fitness` value found in the evaluated
    /// population.
    pub fn highest_fitness(&self) -> &F {
        &self.highest_fitness
    }

    /// Returns the lowest `genetic::Fitness` value found in the evaluated
    /// population.
    pub fn lowest_fitness(&self) -> &F {
        &self.lowest_fitness
    }

    /// Returns the average of all `genetic::Fitness` values of the evaluated
    /// population.
    pub fn average_fitness(&self) -> &F {
        &self.average_fitness
    }

    /// Returns the individual at the given index.
    pub fn individual(&self, index: usize) -> Option<&G> {
        self.individuals.get(index)
    }

    /// Returns the `genetic::Fitness` value of the given individual.
    ///
    /// Note: This function might be more expensive due to the data structure
    /// chosen for this struct. So use it sparingly.
    pub fn fitness_of_individual(&self, individual: &G) -> Option<&F> {
        self.index_of_individual(individual).map(|index|
            &self.fitness_values[index])
    }

    /// Returns the `genetic::Genotype` of the individual with a given
    /// `genetic::Fitness` value.
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
/// loop, i.e. the processing of the evolution from one generation to the next
/// generation.
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
    /// The seed used to derive the population of this generation.
    pub seed: Seed,
    /// The population of the current generation.
    pub population: Vec<G>,
    /// The fitness values of all individuals of the current population.
    pub fitness_values: Vec<F>,
    /// Duration of processing the current generation. This is the time it
    /// took to process one iteration of the algorithm.
    pub duration: Duration,
    /// Accumulated time spent by each thread in case of parallel processing.
    /// In case of sequential processing this time is nearly the same as the
    /// `duration` value. In case of parallel processing this time is usually
    /// a multitude of the `duration`.
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
    /// The evaluated `genetic::Genotype` that is considered to be best.
    pub solution: Evaluated<G, F>,
}

/// The result of running a step in the `Simulation`.
#[derive(Debug, PartialEq, Eq)]
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
#[derive(Debug)]
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
