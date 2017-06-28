
use operator::{GeneticOperator, MutationOp};
use simulation::SimError;
use random::random_index;
use rand::{Rng, thread_rng};
use std::fmt::Debug;


#[derive(Clone)]
pub struct ScalarVectorMutation<V>
    where V: Clone + Debug {
    mutation_rate: f64,
    mutation_range: V,
    mutation_precision: u8,
    max_value: V,
    min_value: V,
}

impl<V> ScalarVectorMutation<V>
    where V: Clone + Debug {
    pub fn new(mutation_rate: f64, mutation_range: V, mutation_precision: u8, min_value: V, max_value: V) -> Self {
        ScalarVectorMutation {
            mutation_rate: mutation_rate,
            mutation_range: mutation_range,
            mutation_precision: mutation_precision,
            max_value: max_value,
            min_value: min_value,
        }
    }
}

impl<V> GeneticOperator for ScalarVectorMutation<V>
    where V: Clone + Debug {
    fn name() -> String {
        "Scalar-Vector-Mutation".to_string()
    }
}

impl<V> MutationOp<Vec<V>> for ScalarVectorMutation<V>
    where V: Clone + Debug + PartialEq + PartialOrd + BreederMutation {
    fn mutate(&self, genome: Vec<V>) -> Result<Vec<V>, SimError> {
        let mut rng = thread_rng();
        Ok(BreederMutation::mutate_genome(genome,
                                          self.mutation_rate,
                                          self.mutation_range.clone(),
                                          self.mutation_precision,
                                          self.min_value.clone(),
                                          self.max_value.clone(),
                                          &mut rng))
    }
}

pub trait BreederMutation
    where Self: Clone + PartialEq + PartialOrd {

    fn breeder_mutated(value: Self, range: Self, adjustment: f64, sign: i8) -> Self;

    fn mutate_genome<R>(genome: Vec<Self>, mutation_rate: f64, range: Self, precision: u8,
                        min_value: Self, max_value: Self, rng: &mut R) -> Vec<Self>
        where R: Rng + Sized {
        let genome_length = genome.len();
        let num_mutations = ((genome_length as f64 * mutation_rate) + rng.next_f64()).floor() as usize;
        let mut mutated = genome;
        for _ in 0..num_mutations {
            let index = random_index(rng, genome_length);
            let sign = *rng.choose(&[-1, 1]).unwrap();
            let adjustment = if *rng.choose(&[true, false]).unwrap() {
                1. / (1i64 << precision) as f64
            } else {
                1.
            };
            let value_mut = Self::breeder_mutated(mutated[index].clone(), range.clone(), adjustment, sign);
            if value_mut < min_value {
                mutated[index] = min_value.clone();
            } else if value_mut > max_value {
                mutated[index] = max_value.clone();
            } else {
                mutated[index] = value_mut;
            }
        }
        mutated
    }
}

macro_rules! impl_breeder_mutation {
    ($($t:ty),*) => {
        $(
            impl BreederMutation for $t {
                #[inline]
                fn breeder_mutated(value: $t, range: $t, adjustment: f64, sign: i8) -> $t {
                    return (value as f64 + range as f64 * adjustment * sign as f64) as $t;
                }
            }
        )*
    }
}

impl_breeder_mutation!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
