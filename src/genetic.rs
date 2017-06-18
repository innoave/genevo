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
pub trait Phenotype<G>
    where G: Genotype
{
    /// Returns its genes as a `Vec<Genotype>`.
    ///
    /// Hint: The simulation may access this function several times. Therefore
    /// this method should be as fast as possible, e.g. through storing or
    /// caching the genes.
    fn genes(&self) -> Vec<G>;

    /// Clones this `Phenotype` into a new one but with the given genes.
    fn derive(&self, new_genes: Vec<G>) -> Self;
}

/// A `Genotype` defines those properties of a `Phenotype` that are relevant
/// for the genetic algorithm. Respectively they are used to determine the
/// `Fitness` value of the solution candidate. These properties are also called
/// chromosomes.
///
/// In order to achieve an efficient execution of the genetic algorithm these
/// properties should be stored in a compact form such as strings or vectors
/// of primitive types.
pub trait Genotype: Copy {}

/// The `Population` defines a set of possible solutions to the optimization
/// or search problem.
#[derive(Debug)]
pub struct Population<T, G>
    where T: Phenotype<G>, G: Genotype
{
    /// The individuals or members of the population.
    pub individuals: Vec<T>,
    // Just here to stop the compiler from complaining about the unused
    // type parameter `G`.
    phantom_type: PhantomData<G>,
}

impl<T, G> Population<T, G>
    where T: Phenotype<G>, G: Genotype
{
    /// Creates a new `Phenotype` with a initial population of the given
    /// individuals.
    pub fn new(individuals: Vec<T>) -> Population<T, G> {
        Population {
            individuals: individuals,
            phantom_type: PhantomData
        }
    }

    /// Returns a slice of all individuals of this `Population`.
    pub fn individuals(&self) -> &Vec<T> {
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
/// genetic algorithm wants to use three `Genotype`s for breeding.
pub trait Breeding<G>
    where G: Genotype
{
    type Parents;
}

/// A `GeneticOperator` defines a function used to guide the genetic algorithm
/// towards a solution to a given problem. There are three main types of
/// operators - Selection, Crossover and Mutation - which must work in
/// conjunction with one another in order for the algorithm to be successful.
///
/// There are unary operators that operate on one genotype at a time, e.g.
/// mutation operators and binary operators that work on two genotypes
/// at a time, e.g. crossover operators.
pub trait GeneticOperator<G>
    where G: Genotype
{
    /// The name of the operator used for display purposes. The name should
    /// make clear to the user of the simulation which implementation of which
    /// kind of operator is being performed.
    ///
    /// It is recommended to combine the some name of the method implemented by
    /// this operator (first part) with some name for the kind of operator
    /// (second part), e.g. "Flip-Bit-Mutation" or "Roulette-Wheel-Selection".
    fn name() -> str;
}

/// A `CrossoverOp` defines a function of how to crossover two `Genotype`s,
/// often called parent genotypes, to derive a new `Genotype`. It is analogous
/// to reproduction and biological crossover. Cross over is a process of taking
/// two parent solutions and producing a child solution from them.
pub trait CrossoverOp<P, G>: GeneticOperator<G>
    where P: Breeding<G>, G: Genotype
{
    /// Performs the crossover of the `Parents` and returns the result as a new
    /// `Genotype`.
    fn crossover(p: &<P>::Parents) -> G;
}

/// A `MutationOp` defines a function of how a `Genotype` mutates. It is used
/// to maintain genetic diversity from one generation of a population of
/// genetic algorithm genotypes to the next. It is analogous to biological
/// mutation. Mutation alters one or more gene values in a chromosome from its
/// initial state. In mutation, the solution may change entirely from the
/// previous solution. Hence GA can come to a better solution by using
/// mutation. Mutation occurs during evolution according to a user-definable
/// mutation probability. This probability should be set low. If it is set too
/// high, the search will turn into a primitive random search.
pub trait MutationOp<G>: GeneticOperator<G>
    where G: Genotype
{
    /// Mutates the given 'Genotype' and returns it as a new 'Genotype'.
    fn mutate(a: &G) -> G;
}

/// A `SelectionOp` defines the function of how to select solutions for being
/// the parents of the next generation. Typically this function gives
/// preference to the better solutions. The implemented method chooses which
/// solutions are considered to be 'best'.
pub trait SelectionOp<T, G, P>: GeneticOperator<G>
    where T: Phenotype<G>, G: Genotype, P: Breeding<G>
{
    /// Selects individuals from the given population according to the
    /// implemented selection strategy.
    fn selection(population: Population<T, G>) -> Vec<<P>::Parents>;
}

/// Defines the evaluation function to calculate the `Fitness` value of a
/// `Genotype` based on its properties.
pub trait FitnessEvaluation<G, F>
    where G: Genotype, F: Fitness
{
    /// Calculates the `Fitness` value of the given `Genotype`.
    fn fitness_of(a: G) -> F;

    /// Returns the very best of all theoretically possible `Fitness` values.
    fn best_possible_fitness() -> F;

    /// Returns the worst of all theoretically possible `Fitness` values.
    /// This is usually a value equivalent to zero.
    fn worst_possible_fitness() -> F;
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
pub trait Fitness: Eq + Ord + Add + Sub + Mul + Div + Sized {

    /// Returns the zero value of this `Fitness` value.
    /// The internal value should be 0.
    fn zero() -> Self;

    /// Returns true if this `Fitness` value is equal to zero.
    fn is_zero(&self) -> bool;

    /// Returns the absolute difference between this `Fitness` value and the
    /// other one, i.e. result = |self| - |other|
    fn abs_diff(&self, other: &Self) -> Self;

}
