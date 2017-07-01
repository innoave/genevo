
use algorithm::*;
use genetic::{Genotype};
use fixedbitset::FixedBitSet;
use rand::{Rand, Rng, thread_rng};
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

pub trait PopulationBuilder {

    fn build_population(population_size: usize, length_of_genome: usize) -> Population<Self>
        where Self: Genotype + Sized {
        if population_size < 60 {
            Population {
                individuals: (0..population_size).map(|_|
                    Self::build_individual(length_of_genome)
                ).collect(),
            }
        } else {
            let left_size = population_size / 2;
            let right_size = population_size - left_size;
            let (mut left_population, mut right_population) = rayon::join(
                || Self::build_population(left_size, length_of_genome),
                || Self::build_population(right_size, length_of_genome)
            );
            left_population.individuals.append(&mut right_population.individuals);
            Population {
                individuals: left_population.individuals,
            }
        }
    }

    fn build_individual(genome_length: usize) -> Self;

}

pub struct EmptyPopulationBuilder {
    // Phantom data to prevent direct instantiation by lib users.
    _empty: PhantomData<bool>,
}

impl EmptyPopulationBuilder {
    pub fn with_genome_length(self, genome_length: usize) -> PopulationWithGenomeLengthBuilder {
        PopulationWithGenomeLengthBuilder {
            genome_length: genome_length,
        }
    }
}

pub struct PopulationWithGenomeLengthBuilder {
    genome_length: usize,
}

impl PopulationWithGenomeLengthBuilder {
    pub fn of_size(self, population_size: usize) -> PopulationWithGenomeLengthAndSizeBuilder {
        PopulationWithGenomeLengthAndSizeBuilder {
            genome_length: self.genome_length,
            population_size: population_size,
        }
    }
}

pub struct PopulationWithGenomeLengthAndSizeBuilder {
    genome_length: usize,
    population_size: usize,
}

impl PopulationWithGenomeLengthAndSizeBuilder {
    pub fn uniform_at_random<G>(self) -> Population<G> where G: Genotype + PopulationBuilder {
        PopulationBuilder::build_population(self.population_size, self.genome_length)
    }
}

pub fn random_population() -> EmptyPopulationBuilder {
    EmptyPopulationBuilder {
        _empty: PhantomData
    }
}

impl PopulationBuilder for FixedBitSet {
    fn build_individual(genome_length: usize) -> FixedBitSet {
        let mut rng = thread_rng();
        let mut genome = FixedBitSet::with_capacity(genome_length);
        for bit in 0..genome_length {
            genome.set(bit, rng.gen());
        }
        genome
    }
}

impl<V> PopulationBuilder for Vec<V> where V: Clone + Debug + Send + Sync + Rand {
    fn build_individual(genome_length: usize) -> Vec<V> {
        let mut rng = thread_rng();
        let mut genome = Vec::with_capacity(genome_length);
        for _ in 0..genome_length {
            genome.push(rng.gen());
        }
        genome
    }
}
