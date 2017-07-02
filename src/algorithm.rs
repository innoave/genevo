use genetic::{Fitness, FitnessFunction, Genotype};
use operator::{CrossoverOp, MutationOp, ReinsertionOp, SelectionOp};
use termination::Termination;
use std::marker::PhantomData;


pub trait Algorithm {

}

/// A `GeneticAlgorithm` declares the building blocks that make up the actual
/// algorithm for a specific optimization problem.
pub struct GeneticAlgorithm<G, F, E, S, C, M, R, T> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
    breeder: C,
    mutator: M,
    reinserter: R,
    termination: T,
}

impl<G, F, E, S, C, M, R, T> Algorithm for GeneticAlgorithm<G, F, E, S, C, M, R, T> {}

impl<G, F, E, S, C, M, R, T> GeneticAlgorithm<G, F, E, S, C, M, R, T>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>,
          S: SelectionOp<G, F>, C: CrossoverOp<G>, M: MutationOp<G>,
          R: ReinsertionOp<G, F>, T: Termination<G, F> {

    pub fn evaluator(&self) -> &E {
        &self.evaluator
    }

    pub fn selector(&self) -> &S {
        &self.selector
    }

    pub fn breeder(&self) -> &C {
        &self.breeder
    }

    pub fn mutator(&self) -> &M {
        &self.mutator
    }

    pub fn reinserter(&self) -> &R {
        &self.reinserter
    }

    pub fn termination(&self) -> &T {
        &self.termination
    }
}

pub fn genetic_algorithm<G, F, E, S, C, M, R, T>() -> EmptyGeneticAlgorithmBuilder<G, F>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G>, R: ReinsertionOp<G, F>, T: Termination<G, F> {
    GeneticAlgorithmBuilder::<G, F, E, S, C, M, R, T>::new()
}

pub struct GeneticAlgorithmBuilder<G, F, E, S, C, M, R, T>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G>, R: ReinsertionOp<G, F>, T: Termination<G, F> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
    breeder: C,
    mutator: M,
    reinserter: R,
    termination: T,
}

impl<G, F, E, S, C, M, R, T> GeneticAlgorithmBuilder<G, F, E, S, C, M, R, T>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G>, R: ReinsertionOp<G, F>, T: Termination<G, F> {

    pub fn new() -> EmptyGeneticAlgorithmBuilder<G, F> {
        EmptyGeneticAlgorithmBuilder {
            _g: PhantomData,
            _f: PhantomData,
        }
    }

    pub fn build(self) -> GeneticAlgorithm<G, F, E, S, C, M, R, T> {
        GeneticAlgorithm {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: self.selector,
            breeder: self.breeder,
            mutator: self.mutator,
            reinserter: self.reinserter,
            termination: self.termination,
        }
    }
}

pub struct EmptyGeneticAlgorithmBuilder<G, F>
    where G: Genotype, F: Fitness {
    // phantom data to prevent direct initialization by the user of the lib
    _g: PhantomData<G>,
    _f: PhantomData<F>
}

impl<G, F> EmptyGeneticAlgorithmBuilder<G, F>
    where G: Genotype, F: Fitness {

    pub fn with_evaluation<E>(self, fitness_function: E)
        -> GeneticAlgorithmWithEvalBuilder<G, F, E>
        where E: FitnessFunction<G, F> {
        GeneticAlgorithmWithEvalBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: fitness_function,
        }
    }
}

pub struct GeneticAlgorithmWithEvalBuilder<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
}

impl<G, F, E> GeneticAlgorithmWithEvalBuilder<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F> {

    pub fn with_selection<S>(self, selection_op: S)
        -> GeneticAlgorithmWithEvalAndSeleBuilder<G, F, E, S>
        where S: SelectionOp<G, F> {
        GeneticAlgorithmWithEvalAndSeleBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: selection_op,
        }
    }
}

pub struct GeneticAlgorithmWithEvalAndSeleBuilder<G, F, E, S>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
}

impl<G, F, E, S> GeneticAlgorithmWithEvalAndSeleBuilder<G, F, E, S>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F> {

    pub fn with_crossover<C>(self, crossover_op: C)
        -> GeneticAlgorithmWithEvalSeleAndBreeBuilder<G, F, E, S, C>
        where C: CrossoverOp<G> {
        GeneticAlgorithmWithEvalSeleAndBreeBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: self.selector,
            breeder: crossover_op,
        }
    }
}

pub struct GeneticAlgorithmWithEvalSeleAndBreeBuilder<G, F, E, S, C>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
    breeder: C,
}

impl<G, F, E, S, C> GeneticAlgorithmWithEvalSeleAndBreeBuilder<G, F, E, S, C>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G> {

    pub fn with_mutation<M>(self, mutation_op: M)
        -> GeneticAlgorithmWithEvalSeleBreeAndMutaBuilder<G, F, E, S, C, M>
        where M: MutationOp<G> {
        GeneticAlgorithmWithEvalSeleBreeAndMutaBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: self.selector,
            breeder: self.breeder,
            mutator: mutation_op,
        }
    }
}

pub struct GeneticAlgorithmWithEvalSeleBreeAndMutaBuilder<G, F, E, S, C, M>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
    breeder: C,
    mutator: M,
}

impl<G, F, E, S, C, M> GeneticAlgorithmWithEvalSeleBreeAndMutaBuilder<G, F, E, S, C, M>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G> {

    pub fn with_reinsertion<R>(self, reinsertion_op: R)
        -> GeneticAlgorithmWithEvalSeleBreeMutaAndReinBuilder<G, F, E, S, C, M, R>
        where R: ReinsertionOp<G, F> {
        GeneticAlgorithmWithEvalSeleBreeMutaAndReinBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: self.selector,
            breeder: self.breeder,
            mutator: self.mutator,
            reinserter: reinsertion_op,
        }
    }
}

pub struct GeneticAlgorithmWithEvalSeleBreeMutaAndReinBuilder<G, F, E, S, C, M, R>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G>, R: ReinsertionOp<G, F> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
    breeder: C,
    mutator: M,
    reinserter: R,
}

impl<G, F, E, S, C, M, R> GeneticAlgorithmWithEvalSeleBreeMutaAndReinBuilder<G, F, E, S, C, M, R>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G>, R: ReinsertionOp<G, F> {

    pub fn with_termination<T>(self, termination: T)
        -> GeneticAlgorithmBuilder<G, F, E, S, C, M, R, T>
        where T: Termination<G, F> {
        GeneticAlgorithmBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: self.selector,
            breeder: self.breeder,
            mutator: self.mutator,
            reinserter: self.reinserter,
            termination: termination,
        }
    }
}

pub trait Simulation<A>
    where A: Algorithm
{

}


pub fn iterate<A, P>(algorithm: A) -> P
    where A: Algorithm, P: Simulation<A>
{
    unimplemented!()
}
