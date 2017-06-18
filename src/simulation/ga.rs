/// This module implements a `Simulator` that executes the genetic algorithm
/// (GA)
///
/// The steps of the genetic algorithm are:
///
/// 1. **Initialize**: Generate random population of n genotypes (or chromosomes)
/// 2. **Fitness**: Evaluate the fitness of each genotype in the population
/// 3. **New Population**: Create a new population by repeating following steps
///    until the new population is complete:
/// 3.1. **Selection**: Select a tuple of parent genotypes from a population
///      according to their fitness (the better fitness, the bigger chance to
///      be selected)
/// 3.2. **Crossover**: With a crossover probability cross over the parents to
///      form a new offspring (children). If no crossover was performed,
///      offspring is an exact copy of parents.
/// 3.3. **Mutation**: With a mutation probability mutate new offspring at each
///      locus (position in genotype)
/// 3.4. **Accepting**: Place new offspring in the new population.
/// 4. **Replace**: Use new generated population for a further run of the
///    algorithm.
/// 5. **Termination**: If the end condition is satisfied, stop, and return the
///    best solution in current population.
/// 6. **Loop**: Go to step 2

use futures::{Future, Stream};
use genetic::{Breeding, Fitness, FitnessEvaluation, Genotype, Phenotype, Population};
use operator::{CrossoverOp, MutationOp, SelectionOp};
use simulation::{BestSolution, SimError, SimResult, Simulation, SimulationBuilder};
use termination::Termination;
use std::marker::PhantomData;


pub struct Simulator<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    termination: Box<Q>,
    initial_population: Vec<T>,
    population: Vec<G>,
    best_solution: G,
    generation: u64,
    _t: PhantomData<&'a T>,
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

impl<'a, T, G, F, E, S, Q, C, M, P> Simulation<'a, T, G, F, E, S, Q, C, M, P>
    for Simulator<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    type Builder = SimulatorBuilder<'a, T, G, F, E, S, Q, C, M, P>;

    fn builder(evaluator: E, selector: S, breeder: C, mutator: M, termination: Q) -> Self::Builder {
        SimulatorBuilder {
            evaluator: Box::new(evaluator),
            selector: Box::new(selector),
            breeder: Box::new(breeder),
            mutator: Box::new(mutator),
            termination: Box::new(termination),
            _t: PhantomData,
            _g: PhantomData,
            _f: PhantomData,
            _p: PhantomData,
        }
    }

    fn run(&mut self) -> Box<Future<Item=SimResult<'a, T, G, F>, Error=SimError>> {
        unimplemented!()
    }

    fn step(&mut self) -> Box<Future<Item=SimResult<'a, T, G, F>, Error=SimError>> {
//        self.evaluate_fitness();
//        self.create_new_population();
//        self.replace_population();
//        self.check_termination();
        unimplemented!()
    }

    fn stream(&mut self) -> Box<Stream<Item=SimResult<'a, T, G, F>, Error=SimError>> {
        unimplemented!()
    }

    fn reset(&mut self) {
        unimplemented!()
    }
}

/// The `SimulationBuilder` implements the 'initialization' stage (step 1) of
/// the genetic algorithm.
pub struct SimulatorBuilder<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    termination: Box<Q>,
    _t: PhantomData<&'a T>,
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

impl<'a, T, G, F, E, S, Q, C, M, P> SimulationBuilder<'a, Simulator<'a, T, G, F, E, S, Q, C, M, P>, T, G, F, E, S, Q, C, M, P>
    for SimulatorBuilder<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    fn initialize(&self, population: Population<T, G>) -> Simulator<'a, T, G, F, E, S, Q, C, M, P> {
        Simulator {
            evaluator: self.evaluator.clone(),
            selector: self.selector.clone(),
            breeder: self.breeder.clone(),
            mutator: self.mutator.clone(),
            termination: self.termination.clone(),
            population: extract_genes(&population),
            best_solution: population.individuals[0].genes(),
            initial_population: population.individuals,
            generation: 1,
            _t: PhantomData,
            _f: PhantomData,
            _p: PhantomData,
        }
    }
}

fn extract_genes<T, G>(population: &Population<T, G>) -> Vec<G>
    where T: Phenotype<G>, G: Genotype
{
    population.individuals.iter().map(|pheno|
        pheno.genes()
    ).collect::<Vec<G>>()
}

impl<'a, T, G, F, E, S, Q, C, M, P> Simulator<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    fn evaluate_fitness(&mut self) {
        unimplemented!()
    }

    fn replace_population(&mut self, new_population: Vec<G>) {
        unimplemented!()
    }

    fn check_termination(&mut self) -> bool {
        unimplemented!()
    }
}

fn create_new_population<G>(current_population: &Vec<G>) -> Vec<G>
    where G: Genotype
{
    unimplemented!()
}
