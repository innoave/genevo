//! The `population` module defines the `Population` struct and the
//! `PopulationBuilder` for building random populations.
//!
//! To use the `PopulationBuilder` for building `Population`s of a custom
//! `genetic::Genotype` an implementation of the `GenomeBuilder` must be
//! provided. A `GenomeBuilder` can build new individuals of a custom
//! `genetic::Genotype`.
//!
//! Default implementations of `GenomeBuilder` are provided for the binary
//! encoded types `fixedbitset::FixedBitSet` and `Vec<bool>` and for the
//! value encoded type `Vec<T>`.
//!
//! ## Examples
//!
//! In the first example we build a population of binary encoded genomes. Each
//! genome has a length of 12 bits and the population comprises 200 individuals.
//!
//! ```rust
//! extern crate genevo;
//! extern crate fixedbitset;
//!
//! use genevo::prelude::*;
//! use genevo::population::BinaryEncodedGenomeBuilder;
//! use fixedbitset::FixedBitSet;
//!
//! fn main() {
//!     let population: Population<FixedBitSet> = build_population()
//!         .with_genome_builder(BinaryEncodedGenomeBuilder::new(12))
//!         .of_size(200)
//!         .uniform_at_random();
//!
//!     println!("{:?}", population);
//!     assert_eq!(200, population.size());
//! }
//! ```
//!
//! The next example builds a population of value encoded genomes. Each genome
//! is represented by a `Vec` of 4 `i64` values in the range of -200 to +200.
//! The generated population consists of 200 individuals.
//!
//! ```rust
//! extern crate genevo;
//!
//! use genevo::prelude::*;
//! use genevo::population::ValueEncodedGenomeBuilder;
//!
//! fn main() {
//!     let population: Population<Vec<i64>> = build_population()
//!         .with_genome_builder(ValueEncodedGenomeBuilder::new(4, -200, 201))
//!         .of_size(200)
//!         .uniform_at_random();
//!
//!     println!("{:?}", population);
//!     assert_eq!(200, population.size());
//! }
//! ```
//!
//! In the following example we demonstrate how to generate a population
//! containing individuals of the custom type `Pos`. Each genome consists of 8
//! `Pos` values. The generated population comprises 200 individuals.
//!
//! ```rust
//! extern crate genevo;
//!
//! use genevo::prelude::*;
//!
//! #[derive(Clone,Debug,PartialEq)]
//! struct Pos {
//!     x: usize,
//!     y: usize,
//! }
//!
//! struct PositionsBuilder;
//! impl GenomeBuilder<Vec<Pos>> for PositionsBuilder {
//!
//!     fn build_genome<R>(&self, _: usize, rng: &mut R) -> Vec<Pos>
//!         where R: Rng + Sized
//!     {
//!         (0..8).map(|row|
//!             Pos {
//!                 x: row,
//!                 y: rng.gen_range(0, 8)
//!             }
//!         ).collect()
//!     }
//! }
//!
//! fn main() {
//!     let population: Population<Vec<Pos>> = build_population()
//!         .with_genome_builder(PositionsBuilder)
//!         .of_size(200)
//!         .uniform_at_random();
//!
//!     println!("{:?}", population);
//!     assert_eq!(200, population.size());
//! }
//! ```

use genetic::Genotype;
use random::{Prng, Rng, RngJump, SampleRange, Seed, get_rng, random_seed};
use fixedbitset::FixedBitSet;
use rayon;
use std::marker::PhantomData;
use std::fmt::Debug;


/// The `Population` defines a set of possible solutions to the optimization
/// or search problem.
#[derive(Clone, Debug, PartialEq)]
pub struct Population<G>
    where G: Genotype
{
    /// The individuals or members of the population.
    individuals: Vec<G>,
}

impl<G> Population<G>
    where G: Genotype
{
    /// Creates a new `Population` with the given individuals as members.
    pub fn with_individuals(individuals: Vec<G>) -> Population<G> {
        Population {
            individuals,
        }
    }

    /// Returns a slice of all individuals of this `Population`.
    pub fn individuals(&self) -> &[G] {
        &self.individuals
    }

    /// Returns the number of individuals in this `Population`.
    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}

/// The `PopulationBuilder` creates a new `Population` with a number of newly
/// created individuals or just individual `genetic::Genotype`s.
///
/// Typically the `PopulationBuilder` is used to create the initial population
/// with randomly created individuals.
///
/// To use this `PopulationBuilder` for a custom `genetic::Genotype` the trait
/// `GenomeBuilder` must be implemented for the custom `genetic::Genotype`.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct PopulationBuilder;

impl PopulationBuilder {

    fn build_population<B, G>(genome_builder: &B, size: usize, rng: Prng) -> Population<G>
        where B: GenomeBuilder<G>, G: Genotype
    {
        if size < 60 {
            let mut rng = rng;
            Population {
                individuals: (0..size).map(|index|
                    genome_builder.build_genome(index, &mut rng)
                ).collect(),
            }
        } else {
            let mut rng1 = rng; rng1.jump(1);
            let mut rng2 = rng; rng2.jump(2);
            let left_size = size / 2;
            let right_size = size - left_size;
            let (left_population, right_population) = rayon::join(
                || Self::build_population(genome_builder, left_size, rng1),
                || Self::build_population(genome_builder, right_size, rng2)
            );
            let mut right_individuals = right_population.individuals;
            let mut individuals = left_population.individuals;
            individuals.append(&mut right_individuals);
            Population {
                individuals,
            }
        }
    }

}

/// A `GenomeBuilder` defines how to build individuals of a population for
/// custom `genetic::Genotype`s.
///
/// Typically the individuals are generated randomly.
pub trait GenomeBuilder<G>: Sync where G: Genotype {

    /// Builds a new genome of type `genetic::Genotype` for the given
    /// `index` using the given random number generator `rng`.
    fn build_genome<R>(&self, index: usize, rng: &mut R) -> G
        where R: Rng + Sized;

}

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct EmptyPopulationBuilder {
    // Phantom data to prevent direct instantiation by lib users.
    _empty: PhantomData<bool>,
}

impl EmptyPopulationBuilder {
    pub fn with_genome_builder<B, G>(self, genome_builder: B)
        -> PopulationWithGenomeBuilderBuilder<B, G>
        where B: GenomeBuilder<G>, G: Genotype
    {
        PopulationWithGenomeBuilderBuilder {
            _g: PhantomData,
            genome_builder,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PopulationWithGenomeBuilderBuilder<B, G>
    where B: GenomeBuilder<G>, G: Genotype
{
    _g: PhantomData<G>,
    genome_builder: B,
}

impl<B, G> PopulationWithGenomeBuilderBuilder<B, G>
    where B: GenomeBuilder<G>, G: Genotype
{
    pub fn of_size(self, population_size: usize) -> PopulationWithGenomeBuilderAndSizeBuilder<B, G> {
        PopulationWithGenomeBuilderAndSizeBuilder {
            _g: self._g,
            genome_builder: self.genome_builder,
            population_size,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PopulationWithGenomeBuilderAndSizeBuilder<B, G>
    where B: GenomeBuilder<G>, G: Genotype
{
    _g: PhantomData<G>,
    genome_builder: B,
    population_size: usize,
}

impl<B, G> PopulationWithGenomeBuilderAndSizeBuilder<B, G>
    where B: GenomeBuilder<G>, G: Genotype
{
    pub fn uniform_at_random(self) -> Population<G> {
        PopulationBuilder::build_population(&self.genome_builder, self.population_size, get_rng(random_seed()))
    }

    pub fn using_seed(self, seed: Seed) -> Population<G> {
        PopulationBuilder::build_population(&self.genome_builder, self.population_size, get_rng(seed))
    }
}

pub fn build_population() -> EmptyPopulationBuilder {
    EmptyPopulationBuilder {
        _empty: PhantomData
    }
}

/// A `GenomeBuilder` that builds binary encoded `genetic::Genotype`s.
///
/// The default implementation can build `fixedbitset::FixedBitSet` genomes
/// and `Vec<bool>` genomes.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryEncodedGenomeBuilder {
    genome_length: usize
}

impl BinaryEncodedGenomeBuilder {

    /// Returns a new instance of the `BinaryEncodedGenomeBuilder` that builds
    /// binary encoded genomes of length specified by the given `genome_length`.
    pub fn new(genome_length: usize) -> Self {
        BinaryEncodedGenomeBuilder {
            genome_length,
        }
    }
}

impl GenomeBuilder<FixedBitSet> for BinaryEncodedGenomeBuilder {

    fn build_genome<R>(&self, _: usize, rng: &mut R) -> FixedBitSet
        where R: Rng + Sized
    {
        let mut genome = FixedBitSet::with_capacity(self.genome_length);
        for bit in 0..self.genome_length {
            genome.set(bit, rng.gen());
        }
        genome
    }
}

impl GenomeBuilder<Vec<bool>> for BinaryEncodedGenomeBuilder {

    fn build_genome<R>(&self, _: usize, rng: &mut R) -> Vec<bool>
        where R: Rng + Sized
    {
        (0..self.genome_length).map(|_|
            rng.gen()
        ).collect()
    }
}

/// A `GenomeBuilder` that builds value encoded `genetic::Genotype`s.
///
/// The default implementation can build `Vec<T>` genomes. The values of
/// `T` are generated randomly in the range between a min value and a max
/// value.
#[derive(Clone, Debug, PartialEq)]
pub struct ValueEncodedGenomeBuilder<V> {
    genome_length: usize,
    min_value: V,
    max_value: V,
}

impl<V> ValueEncodedGenomeBuilder<V> {

    /// Returns a new instance of the `ValueEncodedGenomeBuilder` that builds
    /// value encoded genomes of length specified by the given `genome_length`.
    ///
    /// The values of the generated genomes are in the range between the given
    /// `min_value` (inclusive) and `max_value` (exclusive).
    pub fn new(genome_length: usize, min_value: V, max_value: V) -> Self {
        ValueEncodedGenomeBuilder {
            genome_length,
            min_value,
            max_value,
        }
    }
}

impl<V> GenomeBuilder<Vec<V>> for ValueEncodedGenomeBuilder<V>
    where V: Clone + Debug + PartialEq + PartialOrd + SampleRange + Send + Sync
{
    fn build_genome<R>(&self, _: usize, rng: &mut R) -> Vec<V> where R: Rng + Sized
    {
        (0..self.genome_length).map(|_|
            rng.gen_range(self.min_value.clone(), self.max_value.clone())
        ).collect()
    }
}
