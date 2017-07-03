
use genetic::{Fitness, Genotype};
use simulation::State;
use termination::{StopFlag, Termination};
use std::marker::PhantomData;

//TODO add doc comments
pub fn and<T1, T2, G, F>(condition1: T1, condition2: T2) -> And<T1, T2, G, F>
    where T1: Termination<G, F>, T2: Termination<G, F>,
          G: Genotype, F: Fitness
{
    And::new(condition1, condition2)
}

//TODO add doc comments
#[derive(Clone)]
pub struct And<T1, T2, G, F>
    where T1: Termination<G, F>, T2: Termination<G, F>,
          G: Genotype, F: Fitness
{
    condition1: T1,
    condition2: T2,
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<T1, T2, G, F> And<T1, T2, G, F>
    where T1: Termination<G, F>, T2: Termination<G, F>,
          G: Genotype, F: Fitness
{
    pub fn new(condition1: T1, condition2: T2) -> Self {
        And {
            condition1: condition1,
            condition2: condition2,
            _g: PhantomData,
            _f: PhantomData,
        }
    }

    pub fn condition1(&self) -> &T1 {
        &self.condition1
    }

    pub fn condition2(&self) -> &T2 {
        &self.condition2
    }
}

impl<T1, T2, G, F> Termination<G, F> for And<T1, T2, G, F>
    where T1: Termination<G, F>, T2: Termination<G, F>,
          G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: &State<G, F>) -> StopFlag {
        let mut reasons = Vec::with_capacity(2);
        match self.condition1.evaluate(state) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match self.condition2.evaluate(state) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match reasons.len() {
            0 | 1 => StopFlag::Continue,
            _ => StopFlag::StopNow(reasons.join(" and ")) //TODO how combine the two `StopReason`s preserving combinator semantics?
        }
    }
}

//TODO add doc comments
pub fn or<T1, T2, G, F>(condition1: T1, condition2: T2) -> Or<T1, T2, G, F>
    where T1: Termination<G, F>, T2: Termination<G, F>,
          G: Genotype, F: Fitness
{
    Or::new(condition1, condition2)
}

//TODO add doc comments
#[derive(Clone)]
pub struct Or<T1, T2, G, F>
    where T1: Termination<G, F>, T2: Termination<G, F>,
          G: Genotype, F: Fitness
{
    condition1: T1,
    condition2: T2,
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<T1, T2, G, F> Or<T1, T2, G, F>
    where T1: Termination<G, F>, T2: Termination<G, F>,
          G: Genotype, F: Fitness
{
    pub fn new(condition1: T1, condition2: T2) -> Self {
        Or {
            condition1: condition1,
            condition2: condition2,
            _g: PhantomData,
            _f: PhantomData,
        }
    }

    pub fn condition1(&self) -> &T1 {
        &self.condition1
    }

    pub fn condition2(&self) -> &T2 {
        &self.condition2
    }
}

impl<T1, T2, G, F> Termination<G, F> for Or<T1, T2, G, F>
    where T1: Termination<G, F>, T2: Termination<G, F>,
          G: Genotype, F: Fitness
{
    fn evaluate(&mut self, state: &State<G, F>) -> StopFlag {
        let mut reasons = Vec::with_capacity(2);
        match self.condition1.evaluate(state) {
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
