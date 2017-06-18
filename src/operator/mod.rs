
use genetic::{Breeding, Genotype};

/// A `GeneticOperator` defines a function used to guide the genetic algorithm
/// towards a solution to a given problem. There are three main types of
/// operators - Selection, Crossover and Mutation - which must work in
/// conjunction with one another in order for the algorithm to be successful.
///
/// There are unary operators that operate on one genotype at a time, e.g.
/// mutation operators and binary operators that work on two genotypes
/// at a time, e.g. crossover operators.
pub trait GeneticOperator<G>: Clone
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
    fn crossover(&mut self, p: &<P>::Parents) -> G;
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
    fn mutate(&mut self, a: &G) -> G;
}

/// A `SelectionOp` defines the function of how to select solutions for being
/// the parents of the next generation. Typically this function gives
/// preference to the better solutions. The implemented method chooses which
/// solutions are considered to be 'best'.
pub trait SelectionOp<G, P>: GeneticOperator<G>
    where G: Genotype, P: Breeding<G>
{
    /// Selects individuals from the given population according to the
    /// implemented selection strategy.
    fn selection(&mut self, population: &[G]) -> Vec<<P>::Parents>;
}
