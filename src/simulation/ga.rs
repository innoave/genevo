
use futures::{Future, Stream};
use genetic::{Breeding, Fitness, FitnessEvaluation, Genotype, Phenotype, Population};
use operator::{CrossoverOp, MutationOp, SelectionOp};
use simulation::{BestSolution, Error, Result, Simulation, SimulationBuilder};
use termination::Termination;


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
}

impl<'a, T, G, F, E, S, Q, C, M, P> Simulation<'a, T, G, F, E, S, Q, C, M, P>
    for Simulator<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    fn builder<B>(evaluator: E, selector: S, breeder: C, mutator: M, termination: Q) -> B
        where B: SimulationBuilder<'a, Self, T, G, F, E, S, Q, C, M, P>, Self: Sized
    {
        SimulatorBuilder {
            evaluator: Box::new(evaluator),
            selector: Box::new(selector),
            breeder: Box::new(breeder),
            mutator: Box::new(mutator),
            termination: Box::new(termination),
        }
    }

    fn run(&mut self) -> Future<Item=Result<'a, T, G, F>, Error=Error> {
        unimplemented!()
    }

    fn step(&mut self) -> Future<Item=Result<'a, T, G, F>, Error=Error> {
        unimplemented!()
    }

    fn stream(&mut self) -> Stream<Item=Result<'a, T, G, F>, Error=Error> {
        unimplemented!()
    }

    fn reset(&mut self) {
        unimplemented!()
    }
}

pub struct SimulatorBuilder<'a, Sim, T, G, F, E, S, Q, C, M, P>
    where Sim: Simulation<'a, T, G, F, E, S, Q, C, M, P>,
          T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    termination: Box<Q>,
}

impl<'a, Sim, T, G, F, E, S, Q, C, M, P> SimulationBuilder<'a, Sim, T, G, F, E, S, Q, C, M, P>
    for SimulatorBuilder<'a, Sim, T, G, F, E, S, Q, C, M, P>
    where Sim: Simulation<'a, T, G, F, E, S, Q, C, M, P>,
          T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    fn initialize(&self, population: Population<T, G>) -> Sim {
        Simulator {
            evaluator: self.fitnessEvaluation,
            selector: self.selector,
            breeder: self.breeder,
            mutator: self.mutator,
            termination: self.termination,
            population: extract_genes(population.individuals),
            best_solution: population.individuals[0].genes(),
            initial_population: population,
            generation: 1,
        }
    }
}

fn extract_genes<T, G>(population: &Population<T, G>) -> Vec<G>
    where T: Phenotype<G>, G: Genotype
{
    population.individuals.iter.map(|pheno|
        pheno.genes()
    ).collect::<Vec<G>>()
}
