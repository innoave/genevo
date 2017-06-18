
use futures::{Future, Stream};
use genetic::{Breeding, CrossoverOp, Fitness, FitnessEvaluation, GeneticOperator, Genotype,
                      MutationOp, Phenotype, Population, SelectionOp};
use simulation::{BestSolution, SimError, SimResult, Simulation, SimulationBuilder,
                         Termination};


pub struct GeneticSimulator<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<T, G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    initial_population: Vec<T>,
    population: Vec<G>,
    best_solution: G,
    generation: u64,
}

impl<'a, T, G, F, E, S, Q, C, M, P> Simulation<'a, T, G, F, E, S, Q, C, M, P>
    for GeneticSimulator<'a, T, G, F, E, S, Q, C, M, P>
    where T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<T, G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    fn builder<B>(evaluator: E, selector: S, breeder: C, mutator: M, termination: Vec<Q>) -> B
        where B: SimulationBuilder<'a, Self, T, G, F, E, S, Q, C, M, P>, Self: Sized
    {
        GeneticSimulatorBuilder {
            evaluator: Box::new(evaluator),
            selector: Box::new(selector),
            breeder: Box::new(breeder),
            mutator: Box::new(mutator),
        }
    }

    fn run(&mut self) -> Future<Item=SimResult<'a, T, G, F>, Error=SimError> {
        unimplemented!()
    }

    fn step(&mut self) -> Future<Item=SimResult<'a, T, G, F>, Error=SimError> {
        unimplemented!()
    }

    fn stream(&mut self) -> Stream<Item=SimResult<'a, T, G, F>, Error=SimError> {
        unimplemented!()
    }

    fn reset(&mut self) {
        unimplemented!()
    }
}

pub struct GeneticSimulatorBuilder<'a, Sim, T, G, F, E, S, Q, C, M, P>
    where Sim: Simulation<'a, T, G, F, E, S, Q, C, M, P>,
          T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<T, G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
}

impl<'a, Sim, T, G, F, E, S, Q, C, M, P> SimulationBuilder<'a, Sim, T, G, F, E, S, Q, C, M, P>
    for GeneticSimulatorBuilder<'a, Sim, T, G, F, E, S, Q, C, M, P>
    where Sim: Simulation<'a, T, G, F, E, S, Q, C, M, P>,
          T: 'a + Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<T, G, P>, Q: Termination<'a, T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    fn initialize(&self, population: Population<T, G>) -> Sim {
        GeneticSimulator {
            evaluator: self.fitnessEvaluation,
            selector: self.selector,
            breeder: self.breeder,
            mutator: self.mutator,
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
