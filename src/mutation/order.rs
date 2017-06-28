//! The `order` module provides `operator::MutationOp`s for permutation encoded
//! `genetic::Genotype`s.

use operator::{GeneticOperator, MutationOp};
use simulation::SimError;
use random::random_cut_points;
use rand::{Rng, thread_rng};
use std::fmt::Debug;


#[derive(Clone)]
pub struct OrderInsertMutation {
    mutation_rate: f64,
}

impl OrderInsertMutation {
    pub fn new(mutation_rate: f64) -> Self {
        OrderInsertMutation {
            mutation_rate: mutation_rate,
        }
    }

    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    pub fn set_mutation_rate(&mut self, value: f64) {
        self.mutation_rate = value;
    }
}

impl GeneticOperator for OrderInsertMutation {
    fn name() -> String {
        "Order-Insert-Mutation".to_string()
    }
}

impl<V> MutationOp<Vec<V>> for OrderInsertMutation
    where V: Clone + Debug + PartialEq {

    fn mutate(&self, genome: &Vec<V>) -> Result<Vec<V>, SimError> {
        let mut rng = thread_rng();
        let genome_length = genome.len();
        let num_mutations = ((genome_length as f64 * self.mutation_rate) + rng.next_f64()).floor() as usize;
        let mut mutated = genome.to_vec();
        for _ in 0..num_mutations {
            let (locus1, locus2) = random_cut_points(&mut rng, genome_length);
            let value2 = mutated.remove(locus2);
            mutated.insert(locus1 + 1, value2);
        }
        Ok(mutated)
    }
}

#[derive(Clone)]
pub struct OrderSwapMutation {
    mutation_rate: f64,
}

impl OrderSwapMutation {
    pub fn new(mutation_rate: f64) -> Self {
        OrderSwapMutation {
            mutation_rate: mutation_rate,
        }
    }

    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    pub fn set_mutation_rate(&mut self, value: f64) {
        self.mutation_rate = value;
    }
}

impl GeneticOperator for OrderSwapMutation {
    fn name() -> String {
        "Order-Swap-Mutation".to_string()
    }
}

impl<V> MutationOp<Vec<V>> for OrderSwapMutation
    where V: Clone + Debug + PartialEq {

    fn mutate(&self, genome: &Vec<V>) -> Result<Vec<V>, SimError> {
        let mut rng = thread_rng();
        let genome_length = genome.len();
        let num_mutations = ((genome_length as f64 * self.mutation_rate) + rng.next_f64()).floor() as usize;
        let mut mutated = genome.to_vec();
        for _ in 0..num_mutations {
            let (locus1, locus2) = random_cut_points(&mut rng, genome_length);
            mutated.swap(locus1, locus2);
        }
        Ok(mutated)
    }
}
