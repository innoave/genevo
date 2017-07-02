
use genetic::{Genotype};
use fixedbitset::FixedBitSet;
use rand::{Rng, thread_rng};
use rand::distributions::range::SampleRange;
use rayon;
use std::marker::PhantomData;
use std::fmt::Debug;


#[derive(Debug)]
pub struct Population<G> where G: Genotype {
    individuals: Vec<G>,
}

impl<G> Population<G> where G: Genotype {

    pub fn with_individuals(individuals: Vec<G>) -> Population<G> {
        Population {
            individuals: individuals,
        }
    }

    pub fn individuals(&self) -> &[G] {
        &self.individuals
    }

    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}

pub struct PopulationBuilder;

impl PopulationBuilder {

    pub fn build_population<B, G, TR, R>(genome_builder: &B, size: usize,
                                      thread_rng: &TR) -> Population<G>
        where B: GenomeBuilder<G>, G: Genotype, TR: Fn() -> R + Send + Sync, R: Rng + Sized
    {
        if size < 60 {
            let mut rng = thread_rng();
            Population {
                individuals: (0..size).map(|index|
                    genome_builder.build_genome(index, &mut rng)
                ).collect(),
            }
        } else {
            let left_size = size / 2;
            let right_size = size - left_size;
            let (left_population, right_population) = rayon::join(
                || Self::build_population(genome_builder, left_size, thread_rng),
                || Self::build_population(genome_builder, right_size, thread_rng)
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

pub struct PopulationWithGenomeBuilderBuilder<B, G> where B: GenomeBuilder<G>, G: Genotype {
    _g: PhantomData<G>,
    genome_builder: B,
}

impl<B, G> PopulationWithGenomeBuilderBuilder<B, G> where B: GenomeBuilder<G>, G: Genotype {

    pub fn of_size(self, population_size: usize) -> PopulationWithGenomeBuilderAndSizeBuilder<B, G> {
        PopulationWithGenomeBuilderAndSizeBuilder {
            _g: self._g,
            genome_builder: self.genome_builder,
            population_size: population_size,
        }
    }
}

pub struct PopulationWithGenomeBuilderAndSizeBuilder<B, G> where B: GenomeBuilder<G>, G: Genotype {
    _g: PhantomData<G>,
    genome_builder: B,
    population_size: usize,
}

impl<B, G> PopulationWithGenomeBuilderAndSizeBuilder<B, G> where B: GenomeBuilder<G>, G: Genotype {

    pub fn uniform_at_random(self) -> Population<G> {
        PopulationBuilder::build_population(&self.genome_builder, self.population_size, &thread_rng)
    }

    pub fn using_number_generator<TR, R>(self, thread_rng: &TR) -> Population<G>
        where TR: Fn() -> R + Send + Sync, R: Rng + Sized {
        PopulationBuilder::build_population(&self.genome_builder, self.population_size, &thread_rng)
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

    fn build_genome<R>(&self, index: usize, rng: &mut R) -> FixedBitSet where R: Rng + Sized {
        let mut genome = FixedBitSet::with_capacity(self.genome_length);
        for bit in 0..self.genome_length {
            genome.set(bit, rng.gen());
        }
        genome
    }
}

impl GenomeBuilder<Vec<bool>> for BinaryEncodedGenomeBuilder {

    fn build_genome<R>(&self, index: usize, rng: &mut R) -> Vec<bool> where R: Rng + Sized {
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
    where V: Clone + Debug + PartialEq + PartialOrd + SampleRange + Send + Sync {

    fn build_genome<R>(&self, index: usize, rng: &mut R) -> Vec<V> where R: Rng + Sized {
        (0..self.genome_length).map(|_|
            rng.gen_range(self.min_value.clone(), self.max_value.clone())
        ).collect()
    }
}
