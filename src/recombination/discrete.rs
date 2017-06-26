//! The `discrete` module provides `operator::CrossoverOp`s that recombine
//! `genetic::Genotype`s by exchanging variable values between the parent
//! individuals. Discrete recombination can be applied for binary encoded
//! genotypes and value encoded genotypes.
//!
//! The provided `operator::CrossoverOp`s are:
//! * `UniformCrossover` for `fixedbitset::FixedBitSet`
//! * `DiscreteCrossover` for `Vec` of any type.

use genetic::{Children, Parents};
use operator::{CrossoverOp, GeneticOperator};
use simulation::SimError;
use fixedbitset::FixedBitSet;
use rand::{Rng, thread_rng};
use std::fmt::Debug;


/// The `UniformCrossover` operator combines binary encoded `genetic::Genotype`s
/// by walking through the bits of the parents one by one and randomly selecting
/// the bit of one parent that is copied to the resulting child.
///
/// This crossover operator always creates as many child individuals as there
/// are individuals in the given `genetic::Parents` parameter.
#[derive(Clone)]
pub struct UniformCrossover {}

impl UniformCrossover {
    pub fn new() -> Self {
        UniformCrossover {}
    }
}

impl GeneticOperator for UniformCrossover {
    fn name() -> String {
        "Uniform-Crossover".to_string()
    }
}

impl CrossoverOp<FixedBitSet> for UniformCrossover {

    fn crossover(&self, parents: &Parents<FixedBitSet>) -> Result<Children<FixedBitSet>, SimError> {
        let mut rng = thread_rng();
        let genome_length = parents[0].len();
        let parents_size = parents.len();
        // breed one child for each parent in parents
        let mut offspring: Vec<FixedBitSet> = Vec::with_capacity(parents_size);
        while parents_size > offspring.len() {
            let mut genome = FixedBitSet::with_capacity(genome_length);
            // for each value in the genotype
            for locus in 0..genome_length {
                // pick the value of a randomly chosen parent
                let random = rng.gen_range(0, parents_size);
                let value = parents[random][locus];
                genome.set(locus, value);
            }
            offspring.push(genome);
        }
        Ok(offspring)
    }
}


/// The `DiscreteCrossover` operator combines value encoded 'genetic::Genotype`s
/// by looking at the values of the parents one by one and randomly selecting
/// a value of one parent that is copied to the resulting child.
///
/// This crossover operator always creates as many child individuals as there
/// are individuals in the given `genetic::Parents` parameter.
#[derive(Clone)]
pub struct DiscreteCrossover {}

impl DiscreteCrossover {
    pub fn new() -> Self {
        DiscreteCrossover {}
    }
}

impl GeneticOperator for DiscreteCrossover {
    fn name() -> String {
        "Discrete-Crossover".to_string()
    }
}

impl<V> CrossoverOp<Vec<V>> for DiscreteCrossover
    where V: Clone + Debug + PartialEq
{
    fn crossover(&self, parents: &Parents<Vec<V>>) -> Result<Children<Vec<V>>, SimError> {
        let mut rng = thread_rng();
        let genome_length = parents[0].len();
        let parents_size = parents.len();
        // breed one child for each parent in parents
        let mut offspring: Vec<Vec<V>> = Vec::with_capacity(parents_size);
        while parents_size > offspring.len() {
            let mut genome = Vec::with_capacity(genome_length);
            // for each value in the genotype
            for locus in 0..genome_length {
                // pick the value of a randomly chosen parent
                let random = rng.gen_range(0, parents_size);
                let value = parents[random][locus].clone();
                genome.push(value);
            }
            offspring.push(genome);
        }
        Ok(offspring)
    }
}
