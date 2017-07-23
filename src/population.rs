
use genetic::{Genotype};
use random::{Prng, Rng, RngJump, SampleRange, Seed, get_rng, random_seed};
use fixedbitset::FixedBitSet;
use rayon;
use std::marker::PhantomData;
use std::fmt::Debug;


/// The `Population` defines a set of possible solutions to the optimization
/// or search problem.
#[derive(Debug)]
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
            individuals: individuals,
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

/// A `PopulationBuilder` creates a new `Population` with a number of newly
/// created individuals or just individual `genetic::Genotype`s.
///
/// Typically the `PopulationBuilder` is used to create the initial
/// population with randomly created individuals.
///
/// To use this `PopulationBuilder` for a custom `genetic::Genotype` the trait
/// `GenomeBuilder` must be implemented for the custom `genetic::Genotype`.
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
                individuals: individuals,
            }
        }
    }

}

pub trait GenomeBuilder<G>: Sync where G: Genotype {

    fn build_genome<R>(&self, index: usize, rng: &mut R) -> G
        where R: Rng + Sized;

}

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
            genome_builder: genome_builder,
        }
    }
}

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
            population_size: population_size,
        }
    }
}

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

pub struct BinaryEncodedGenomeBuilder {
    genome_length: usize
}

impl BinaryEncodedGenomeBuilder {

    pub fn new(genome_length: usize) -> Self {
        BinaryEncodedGenomeBuilder {
            genome_length: genome_length,
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

pub struct ValueEncodedGenomeBuilder<V> {
    genome_length: usize,
    min_value: V,
    max_value: V,
}

impl<V> ValueEncodedGenomeBuilder<V> {
    pub fn new(genome_length: usize, min_value: V, max_value: V) -> Self {
        ValueEncodedGenomeBuilder {
            genome_length: genome_length,
            min_value: min_value,
            max_value: max_value,
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
