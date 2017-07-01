
use fixedbitset::FixedBitSet;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Phenotype<T> {
    fn as_() -> T;
}

pub trait Genotype: Clone + Debug + Send + Sync {
    type Dna;
}

impl Genotype for FixedBitSet {
    type Dna = bool;
}

impl<V> Genotype for Vec<V> where V: Clone + Debug + Send + Sync {
    type Dna = V;
}

pub trait Fitness {

}

pub trait FitnessFunction<G, F> {

    fn evaluate(genome: G) -> F;

}

pub struct EvaluatedGenotype<G, F>
    where G: Genotype, F: Fitness {
    genome: G,
    fitness: F,
}

pub trait SelectionOp<G, F>
    where G: Genotype, F: Fitness {

}

pub trait CrossoverOp<G>
    where G: Genotype {

}

pub trait MutationOp<G>
    where G: Genotype {

}

pub trait ReinsertionOp<G, F>
    where G: Genotype, F: Fitness {

}

pub trait Termination<G, F>
    where G: Genotype, F: Fitness {

}

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

pub fn genetic_algorithm<G, F, E, S, C, M, R, T>() -> FitnessFunctionBuilder<G, F>
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

    pub fn new() -> FitnessFunctionBuilder<G, F> {
        FitnessFunctionBuilder {
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

pub struct FitnessFunctionBuilder<G, F>
    where G: Genotype, F: Fitness {
    // phantom data to prevent direct initialization by the user of the lib
    _g: PhantomData<G>,
    _f: PhantomData<F>
}

impl<G, F> FitnessFunctionBuilder<G, F>
    where G: Genotype, F: Fitness {

    pub fn with_evaluation<E>(self, fitness_function: E) -> SelectionOpBuilder<G, F, E>
        where E: FitnessFunction<G, F> {
        SelectionOpBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: fitness_function,
        }
    }
}

pub struct SelectionOpBuilder<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
}

impl<G, F, E> SelectionOpBuilder<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F> {

    pub fn with_selection<S>(self, selection_op: S) -> CrossoverOpBuilder<G, F, E, S>
        where S: SelectionOp<G, F> {
        CrossoverOpBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: selection_op,
        }
    }
}

pub struct CrossoverOpBuilder<G, F, E, S>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
}

impl<G, F, E, S> CrossoverOpBuilder<G, F, E, S>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F> {

    pub fn with_crossover<C>(self, crossover_op: C) -> MutationOpBuilder<G, F, E, S, C>
        where C: CrossoverOp<G> {
        MutationOpBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: self.selector,
            breeder: crossover_op,
        }
    }
}

pub struct MutationOpBuilder<G, F, E, S, C>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
    breeder: C,
}

impl<G, F, E, S, C> MutationOpBuilder<G, F, E, S, C>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G> {

    pub fn with_mutation<M>(self, mutation_op: M) -> ReinsertionOpBuilder<G, F, E, S, C, M>
        where M: MutationOp<G> {
        ReinsertionOpBuilder {
            _g: self._g,
            _f: self._f,
            evaluator: self.evaluator,
            selector: self.selector,
            breeder: self.breeder,
            mutator: mutation_op,
        }
    }
}

pub struct ReinsertionOpBuilder<G, F, E, S, C, M>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G> {
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
    breeder: C,
    mutator: M,
}

impl<G, F, E, S, C, M> ReinsertionOpBuilder<G, F, E, S, C, M>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G> {

    pub fn with_reinsertion<R>(self, reinsertion_op: R) -> TerminationBuilder<G, F, E, S, C, M, R>
        where R: ReinsertionOp<G, F> {
        TerminationBuilder {
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

pub struct TerminationBuilder<G, F, E, S, C, M, R>
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

impl<G, F, E, S, C, M, R> TerminationBuilder<G, F, E, S, C, M, R>
    where G: Genotype, F: Fitness, E: FitnessFunction<G, F>, S: SelectionOp<G, F>,
          C: CrossoverOp<G>, M: MutationOp<G>, R: ReinsertionOp<G, F> {

    pub fn with_termination<T>(self, termination: T) -> GeneticAlgorithmBuilder<G, F, E, S, C, M, R, T>
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
