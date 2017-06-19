
use genetic::{Fitness, Genotype, Phenotype};
use simulation::State;
use termination::{StopFlag, Termination};
use std::marker::PhantomData;
use std::sync::Arc;

//TODO add doc comments
pub fn and<E1, E2, T, G, F>(condition1: E1, condition2: E2) -> And<E1, E2, T, G, F>
    where E1: Termination<T, G, F>, E2: Termination<T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    And::new(condition1, condition2)
}

//TODO add doc comments
#[derive(Clone)]
pub struct And<E1, E2, T, G, F>
    where E1: Termination<T, G, F>, E2: Termination<T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    condition1: E1,
    condition2: E2,
    _t: PhantomData<T>,
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<E1, E2, T, G, F> And<E1, E2, T, G, F>
    where E1: Termination<T, G, F>, E2: Termination<T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    pub fn new(condition1: E1, condition2: E2) -> Self {
        And {
            condition1: condition1,
            condition2: condition2,
            _t: PhantomData,
            _g: PhantomData,
            _f: PhantomData,
        }
    }
}

impl<E1, E2, T, G, F> Termination<T, G, F> for And<E1, E2, T, G, F>
    where E1: Termination<T, G, F>, E2: Termination<T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: Arc<State<T, G, F>>) -> StopFlag {
        let mut reasons = Vec::with_capacity(2);
        match self.condition1.evaluate(state.clone()) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match self.condition2.evaluate(state) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match reasons.len() {
            0 => StopFlag::Continue,
            1 => StopFlag::Continue,
            _ => StopFlag::StopNow(reasons.join(" and ")) //TODO how combine the two `StopReason`s preserving combinator semantics?
        }
    }
}

//TODO add doc comments
pub fn or<E1, E2, T, G, F>(condition1: E1, condition2: E2) -> Or<E1, E2, T, G, F>
    where E1: Termination<T, G, F>, E2: Termination<T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    Or::new(condition1, condition2)
}

//TODO add doc comments
#[derive(Clone)]
pub struct Or<E1, E2, T, G, F>
    where E1: Termination<T, G, F>, E2: Termination<T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    condition1: E1,
    condition2: E2,
    _t: PhantomData<T>,
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<E1, E2, T, G, F> Or<E1, E2, T, G, F>
    where E1: Termination<T, G, F>, E2: Termination<T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    pub fn new(condition1: E1, condition2: E2) -> Self {
        Or {
            condition1: condition1,
            condition2: condition2,
            _t: PhantomData,
            _g: PhantomData,
            _f: PhantomData,
        }
    }
}

impl<E1, E2, T, G, F> Termination<T, G, F> for Or<E1, E2, T, G, F>
    where E1: Termination<T, G, F>, E2: Termination<T, G, F>,
          T: Phenotype<G>, G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: Arc<State<T, G, F>>) -> StopFlag {
        let mut reasons = Vec::with_capacity(2);
        match self.condition1.evaluate(state.clone()) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match self.condition2.evaluate(state) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match reasons.len() {
            0 => StopFlag::Continue,
            1 => StopFlag::StopNow(reasons[0].clone()),
            _ => StopFlag::StopNow(reasons.join(" and ")) //TODO how combine the two `StopReason`s preserving combinator semantics?
        }
    }
}
