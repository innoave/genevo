
use genetic::{Fitness, Genotype, Phenotype};
use simulation::State;
use termination::Termination;
use std::marker::PhantomData;


pub fn and<'a, E1, E2, T, G, F>(condition1: E1, condition2: E2) -> And<'a, E1, E2, T, G, F>
    where E1: Termination<'a, T, G, F>, E2: Termination<'a, T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    And::new(condition1, condition2)
}

pub struct And<'a, E1, E2, T, G, F>
    where E1: Termination<'a, T, G, F>, E2: Termination<'a, T, G, F>,
          T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    condition1: E1,
    condition2: E2,
    phantom_t: &'a PhantomData<T>,
    phantom_g: PhantomData<G>,
    phantom_f: PhantomData<F>,
}

impl<'a, E1, E2, T, G, F> And<'a, E1, E2, T, G, F>
    where E1: Termination<'a, T, G, F>, E2: Termination<'a, T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    pub fn new(condition1: E1, condition2: E2) -> Self {
        And {
            condition1: condition1,
            condition2: condition2,
            phantom_t: PhantomData,
            phantom_g: PhantomData,
            phantom_f: PhantomData,
        }
    }
}

impl<'a, E1, E2, T, G, F> Termination<'a, T, G, F> for And<'a, E1, E2, T, G, F>
    where E1: Termination<'a, T, G, F>, E2: Termination<'a, T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: State<'a, T, G, F>) -> bool {
        self.condition1.evaluate(state) && self.condition2.evaluate(state)
    }
}

pub fn or<'a, E1, E2, T, G, F>(condition1: E1, condition2: E2) -> Or<'a, E1, E2, T, G, F>
    where E1: Termination<'a, T, G, F>, E2: Termination<'a, T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    Or::new(condition1, condition2)
}

pub struct Or<'a, E1, E2, T, G, F>
    where E1: Termination<'a, T, G, F>, E2: Termination<'a, T, G, F>,
          T: 'a + Phenotype<G>, G: Genotype, F: Fitness
{
    condition1: E1,
    condition2: E2,
    phantom_t: &'a PhantomData<T>,
    phantom_g: PhantomData<G>,
    phantom_f: PhantomData<F>,
}

impl<'a, E1, E2, T, G, F> Or<'a, E1, E2, T, G, F>
    where E1: Termination<'a, T, G, F>, E2: Termination<'a, T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    pub fn new(condition1: E1, condition2: E2) -> Self {
        And {
            condition1: condition1,
            condition2: condition2,
            phantom_t: PhantomData,
            phantom_g: PhantomData,
            phantom_f: PhantomData,
        }
    }
}

impl<'a, E1, E2, T, G, F> Termination<'a, T, G, F> for Or<'a, E1, E2, T, G, F>
    where E1: Termination<'a, T, G, F>, E2: Termination<'a, T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: State<'a, T, G, F>) -> bool {
        self.condition1.evaluate(state) || self.condition2.evaluate(state)
    }
}
