//! The `operator` module defines the types of genetic operators as traits.
//! A genetic operator defines a function that performs a specific stage in
//! the genetic algorithm. Each of these genetic operator can be implemented
//! in variety of ways using different algorithms and methods.
//!
//! The genetic operators are the building blocks of the genetic algorithm.
//! Their different implementations can be combined in a variety of ways to
//! make up the actual simulation of a specific problem.

use genetic::{Breeding, Fitness, Genotype};
use simulation::{EvaluatedPopulation, SimError};


/// Marker trait for genetic operators and functions that are used for
/// single-objective optimization.
pub trait SingleObjective {}

/// Marker trait for genetic operators and functions that are used for
/// multi-objective optimization.
pub trait MultiObjective {}

/// A `GeneticOperator` defines a function used to guide the genetic algorithm
/// towards a solution to a given problem. There are three main types of
/// operators - Selection, Crossover and Mutation - which must work in
/// conjunction with one another in order for the algorithm to be successful.
///
/// There are unary operators that operate on one genotype at a time, e.g.
/// mutation operators, and binary operators that work on two genotypes
/// at a time, e.g. crossover operators.
pub trait GeneticOperator: Clone {
    /// The name of the operator used for display purposes. The name should
    /// make clear to the user of the simulation which implementation of which
    /// kind of operator is being performed.
    ///
    /// It is recommended to combine some name of the method implemented by
    /// this operator (first part) with some name for the kind of operator
    /// (second part), e.g. "Flip-Bit-Mutation" or "Roulette-Wheel-Selection".
    fn name() -> String;
}

/// A `SelectionOp` defines the function of how to select solutions for being
/// the parents of the next generation.
pub trait SelectionOp<G, F, B>: GeneticOperator
    where G: Genotype, F: Fitness, B: Breeding<G>
{
    /// Selects individuals from the given population according to the
    /// implemented selection strategy.
    fn selection(&self, population: &EvaluatedPopulation<G, F>) -> Result<Vec<<B>::Parents>, SimError>;
}

/// A `CrossoverOp` defines a function of how to crossover two `Genotype`s,
/// often called parent genotypes, to derive a new `Genotype`. It is analogous
/// to reproduction and biological crossover. Cross over is a process of taking
/// two parent solutions and producing an offspring solution from them.
pub trait CrossoverOp<B, G>: GeneticOperator
    where B: Breeding<G>, G: Genotype
{
    /// Performs the crossover of the `Parents` and returns the result as a new
    /// `Genotype` - the offspring.
    fn crossover(&self, parents_list: &<B>::Parents) -> Result<G, SimError>;
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
pub trait MutationOp<G>: GeneticOperator
    where G: Genotype
{
    /// Mutates the given 'Genotype' and returns it as a new 'Genotype'.
    fn mutate(&self, genome: &G) -> Result<G, SimError>;
}

/// A `ReinsertionOp` defines a function that combines the offspring with the
/// current population to create the population for the next generation.
pub trait ReinsertionOp<G, F>: GeneticOperator
    where G: Genotype, F: Fitness
{
    /// Combines the given offspring with the current population to create
    /// the population of the next generation.
    ///
    /// The offspring parameter is passed as mutable borrow. It can be
    /// mutated to avoid cloning. The `Genotype`s that make it up into the
    /// new population should be moved instead of cloned. After this function
    /// finishes the offspring vector should hold only those `Genotype`s that
    /// have not been included in the resulting population. If by the end of
    /// this function all `Genotype`s in offspring have been moved to the
    /// resulting population the offspring vector the offspring vector should
    /// be left empty.
    fn combine(&self, offspring: &mut Vec<G>, population: &EvaluatedPopulation<G, F>) -> Result<Vec<G>, SimError>;
}
