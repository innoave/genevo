
use algorithm::*;
use fixedbitset::FixedBitSet;
use rand::{Rng, thread_rng};
use rayon;


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
}

pub trait PopulationBuilder<G> where G: Genotype, Self: Send + Sync {

    fn build_population(&self, population_size: usize, length_of_genome: usize) -> Population<G> {
        if population_size < 60 {
            Population {
                individuals: (0..population_size).map(|_|
                    self.build_individual(length_of_genome)
                ).collect(),
            }
        } else {
            let left_size = population_size / 2;
            let right_size = population_size - left_size;
            let (mut left_population, mut right_population) = rayon::join(
                || self.build_population(left_size, length_of_genome),
                || self.build_population(right_size, length_of_genome)
            );
            left_population.individuals.append(&mut right_population.individuals);
            Population {
                individuals: left_population.individuals,
            }
        }
    }

    fn build_individual(&self, genome_length: usize) -> G;

}

pub struct RandomPopulationBuilder {}

impl PopulationBuilder<FixedBitSet> for RandomPopulationBuilder {

    fn build_individual(&self, genome_length: usize) -> FixedBitSet {
        let mut rng = thread_rng();
        let mut bitset = FixedBitSet::with_capacity(genome_length);
        for bit in 0..genome_length {
            bitset.set(bit, rng.gen());
        }
        bitset
    }
}
