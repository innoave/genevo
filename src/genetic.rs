//! The 'genetic' module defines types for the genetic algorithm. The traits
//! defined in this module should be implemented to formulate an optimization
//! or search problem. The types are named after terms as they are found in
//! genetic biology.

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};

/// A `Phenotype` is a candidate solution of the optimization or search problem.
/// Phenotypes are also called individuals or creatures. A `Phenotype` is the
/// type of data for which the optimal value for solving an optimization or
/// search problem should be found.
///
/// The `Phenotype` represents a subject in the problem domain. It holds its
/// genes which are its representation in the search space of the genetic
/// algorithm. The genes are represented as a vector of `Genotype`s.
pub trait Phenotype<G>: Clone + Send + Sync
    where G: Genotype
{
    /// Returns its genes as a `Genotype`.
    ///
    /// Hint: The simulation may access this function several times. Therefore
    /// this method should be as fast as possible, e.g. through storing or
    /// caching the genes.
    fn genes(&self) -> G;

    /// Clones this `Phenotype` into a new one but with the given genes.
    fn derive(&self, new_genes: G) -> Self;
}

/// A `Genotype` defines those properties of a `Phenotype` that are relevant
/// for the genetic algorithm. Respectively they are used to determine the
/// `Fitness` value of the solution candidate. These properties are also called
/// chromosomes.
///
/// In order to achieve an efficient execution of the genetic algorithm these
/// properties should be stored in a compact form such as strings or vectors
/// of primitive types.
pub trait Genotype: Clone + Send + Sync {}

/// The `Population` defines a set of possible solutions to the optimization
/// or search problem.
#[derive(Debug)]
pub struct Population<T, G>
    where T: Phenotype<G>, G: Genotype
{
    /// The individuals or members of the population.
    individuals: Vec<T>,
    // Just here to stop the compiler from complaining about the unused
    // type parameter `G`.
    _g: PhantomData<G>,
}

impl<T, G> Population<T, G>
    where T: Phenotype<G>, G: Genotype
{
    /// Creates a new `Phenotype` with a initial population of the given
    /// individuals.
    pub fn new(individuals: Vec<T>) -> Population<T, G> {
        Population {
            individuals: individuals,
            _g: PhantomData
        }
    }

    /// Returns a slice of all individuals of this `Population`.
    pub fn individuals(&self) -> &[T] {
        &self.individuals
    }

    /// Returns the number of individuals in this `Population`.
    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}

/// A `Breeding` defines the type of `Parents` returned by the `SelectionOp`
/// and used for breeding in the `CrossoverOp`. Usually parents will be
/// defined as tuple of two `Genotype`s but maybe some derivation of the
/// genetic algorithm wants to use three or more `Genotype`s for breeding.
pub trait Breeding<G>
    where G: Genotype
{
    type Parents;
}

/// Defines the evaluation function to calculate the `Fitness` value of a
/// `Genotype` based on its properties.
pub trait FitnessEvaluation<G, F>: Clone
    where G: Genotype, F: Fitness
{
    /// Calculates the `Fitness` value of the given `Genotype`.
    fn fitness_of(&mut self, a: &G) -> F;

    /// Normalizes the given `Fitness` values and returns the normalized
    /// values in a new vector.
    fn normalize(&mut self, a: &[F]) -> Vec<F>;

    /// Calculates the average `Fitness` value of the given `Fitness` values.
    fn average(&mut self, a: &[F]) -> F;

    /// Returns the very best of all theoretically possible `Fitness` values.
    fn best_possible_fitness(&self) -> F;

    /// Returns the worst of all theoretically possible `Fitness` values.
    /// This is usually a value equivalent to zero.
    fn worst_possible_fitness(&self) -> F;
}

/// A `Fitness` value is used to determine the quality of a `Genotype`.
/// `Fitness` values should have an ordering, also called ranking.
///
/// **Make sure the following statement holds:**
/// A `Genotype` with a `Fitness` value of `f1` performs better than another
/// `Genotype` with a `Fitness` value of `f2` if `f1 > f2`.
///
/// It also has to implement the Add, Sub, Mul and Div trait so that the
/// simulation can normalize the fitness value of each individual across
/// a population.
pub trait Fitness: Eq + Ord + Add + Sub + Mul + Div + Clone + Sized + Send + Sync {

    /// Returns the zero value of this `Fitness` value.
    /// The internal value should be 0.
    fn zero() -> Self;

    /// Returns the absolute difference between this `Fitness` value and the
    /// other one, i.e. result = |self| - |other|
    fn abs_diff(&self, other: &Self) -> Self;

}
