//! The `algorithm` module defines traits and structs for implementing
//! concrete algorithms such as the `ga::GeneticAlgorithm` and various
//! operators as defined in the `operator` module.

use genetic::{Fitness, Genotype};
use random::Prng;
use chrono::{DateTime, Local};
use std::fmt::Debug;
use std::rc::Rc;

/// An `Algorithm` defines the steps to be processed in a
/// `simulation::Simulation`. The `Simulation` uses an implementation of an
/// `Algorithm` to perform one iteration of the evaluation stage.
pub trait Algorithm {
    type Output: Clone + Debug + PartialEq;
    type Error: Clone + Debug + PartialEq;

    fn next(&mut self, iteration: u64, rng: &mut Prng) -> Result<Self::Output, Self::Error>;

    fn reset(&mut self) -> Result<bool, Self::Error>;

}

pub trait OptimizationResult<G, F>
    where G: Genotype, F: Fitness
{
    fn best_solution(&self) -> &BestSolution<G, F>;
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

/// The `EvaluatedPopulation` holds the results of the evaluation stage of
/// the genetic algorithm. It is used to pass these values to the
/// `operator::SelectionOp` to enable this operator to do its job.
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
            individuals,
            fitness_values,
            highest_fitness,
            lowest_fitness,
            average_fitness
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

    /// Returns the `Evaluated` individual with a given `genetic::Fitness`
    /// value.
    ///
    /// Note: This function might be more expensive due to the data structure
    /// chosen for this struct. So use it sparingly.
    pub fn evaluated_individual_with_fitness(&self, fitness: &F) -> Option<Evaluated<G, F>> {
        self.index_of_fitness(&fitness)
            .map(|index| Evaluated {
                genome: self.individuals[index].clone(),
                fitness: self.fitness_values[index].clone(),
            })
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
