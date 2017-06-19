/// This module provides the `Simulator` which implements the genetic algorithm
/// (GA) and the related `SimulatorBuilder`.
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
///
/// The `Simulator` implements the `simulation::Simulation` trait. The
/// `SimulatorBuilder` implements the `simulation::SimulationBuilder` trait.

use chrono::{DateTime, Duration, Local};
use futures::future;
use futures::future::{BoxFuture, Future};
use futures::stream;
use futures::stream::{BoxStream, Stream};
use genetic::{Breeding, Fitness, FitnessEvaluation, Genotype, Phenotype, Population};
use operator::{CrossoverOp, MutationOp, SelectionOp};
use simulation::{BestSolution, Evaluated, SimError, SimResult, Simulation, SimulationBuilder,
                 State};
use termination::{StopFlag, Termination};
use std::marker::PhantomData;
use std::mem;
use std::sync::Arc;


pub struct Simulator<T, G, F, E, S, Q, C, M, P>
    where T: Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    termination: Box<Q>,
    initial_population: Population<T, G>,
    started: bool,
    started_at: DateTime<Local>,
    generation: u64,
    curr_population: Vec<G>,
    fitness_values: Vec<F>,
    highest_fitness: F,
    lowest_fitness: F,
    average_fitness: F,
    normalized_fitness: Vec<F>,
    processing_time: Duration,
    best_solution: BestSolution<T, G, F>,
    next_population: Vec<G>,
    _p: PhantomData<P>,
}

impl<T, G, F, E, S, Q, C, M, P> Simulation<T, G, F, E, S, Q, C, M, P>
    for Simulator<T, G, F, E, S, Q, C, M, P>
    where T: Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    type Builder = SimulatorBuilder<T, G, F, E, S, Q, C, M, P>;

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

    fn run(&mut self) -> BoxFuture<SimResult<T, G, F>, SimError> {
        if self.started {
            return Future::boxed(future::err(SimError::SimulationAlreadyRunning(
                    format!("Simulation already running since {}", &self.started_at))));
        } else {
            self.started = true;
            self.started_at = Local::now();
        }
        unimplemented!()
    }

    fn step(&mut self) -> BoxFuture<SimResult<T, G, F>, SimError> {
        if self.started {
            return Future::boxed(future::err(SimError::SimulationAlreadyRunning(
                format!("Simulation already running since {}", &self.started_at))));
        } else {
            self.started = true;
            self.started_at = Local::now();
        }
        let loop_started_at = Local::now();
        // Stage 2: The fitness check:
        self.evaluate_fitness();
        self.normalized_fitness = self.evaluator.normalize(&self.fitness_values);
        self.average_fitness = self.evaluator.average(&self.fitness_values);

        // Stage 3: The making of a new population:

        self.create_new_population();

        // Stage 4: Replace current generation with next generation:
        let loop_time = Local::now().signed_duration_since(loop_started_at);
        let state = Arc::new(self.replace_generation(loop_time));
        // Stage 5: Be aware of the termination:
        match self.termination.evaluate(state) {
            StopFlag::StopNow(reason) => {
                unimplemented!()
            },
            StopFlag::Continue => (),
        }
        unimplemented!()
    }

    fn stream(&mut self) -> BoxStream<SimResult<T, G, F>, SimError> {
        if self.started {
            return stream::Once::boxed(stream::once(Err(SimError::SimulationAlreadyRunning(
                    format!("Simulation already running since {}", &self.started_at.clone())))));
        } else {
            self.started = true;
            self.started_at = Local::now();
        }
        unimplemented!()
    }

    fn reset(&mut self) {
        if self.started {
            return; //TODO we should not silently ignore this command -> need some error handling!
//                    Box::new(future::result(Err(SimError::SimulationAlreadyRunning(
//                    format!("Simulation still running since {}. Wait for the simulation to finish \
//                        or abort it before resetting it.", time)))));
        }
        unimplemented!()
    }
}

/// The `SimulationBuilder` implements the 'initialization' stage (step 1) of
/// the genetic algorithm.
pub struct SimulatorBuilder<T, G, F, E, S, Q, C, M, P>
    where T: Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    termination: Box<Q>,
    _t: PhantomData<T>,
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

impl<T, G, F, E, S, Q, C, M, P> SimulationBuilder<Simulator<T, G, F, E, S, Q, C, M, P>, T, G, F, E, S, Q, C, M, P>
    for SimulatorBuilder<T, G, F, E, S, Q, C, M, P>
    where T: Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    fn initialize(&mut self, population: Population<T, G>) -> Simulator<T, G, F, E, S, Q, C, M, P> {
        let generation = 1;
        let p_size = population.size();
        let best_s = BestSolution {
            found_at: Local::now(),
            generation: generation,
            solution: Arc::new(Evaluated::new(Arc::new(population.individuals()[0].clone()),
                                              Fitness::zero(), Fitness::zero())),
        };
        Simulator {
            evaluator: self.evaluator.clone(),
            selector: self.selector.clone(),
            breeder: self.breeder.clone(),
            mutator: self.mutator.clone(),
            termination: self.termination.clone(),
            initial_population: population,
            started: false,
            started_at: Local::now(),
            generation: generation,
            curr_population: extract_genes(&population),
            fitness_values: Vec::with_capacity(p_size),
            highest_fitness: self.evaluator.worst_possible_fitness(),
            lowest_fitness: self.evaluator.best_possible_fitness(),
            average_fitness: <F>::zero(),
            normalized_fitness: Vec::with_capacity(p_size),
            processing_time: Duration::zero(),
            best_solution: best_s,
            next_population: Vec::with_capacity(p_size),
            _p: PhantomData,
        }
    }
}

fn extract_genes<T, G>(population: &Population<T, G>) -> Vec<G>
    where T: Phenotype<G>, G: Genotype
{
    population.individuals().iter().map(|pheno|
        pheno.genes()
    ).collect::<Vec<G>>()
}

impl<T, G, F, E, S, Q, C, M, P> Simulator<T, G, F, E, S, Q, C, M, P>
    where T: Phenotype<G>, G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, P>, Q: Termination<T, G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    /// Calculates the `Fitness` value of each `Phenotype` and records the
    /// highest and lowest values.
    fn evaluate_fitness(&mut self) {
        for genome in self.curr_population {
            let score = self.evaluator.fitness_of(&genome);
            if score > self.highest_fitness {
                self.highest_fitness = score.clone();
            }
            if score < self.lowest_fitness {
                self.lowest_fitness = score.clone();
            }
            self.fitness_values.push(score);
        }
    }

    /// Generates a `State` object about the last processed loop, replaces the
    /// current generation with the next generation and increases the
    /// generation counter.
    fn replace_generation(&mut self, loop_time: Duration) -> State<T, G, F> {
        let curr_generation = self.generation;
        let p_size = self.next_population.len();
        let next_p = mem::replace(&mut self.next_population, Vec::with_capacity(p_size));
        let curr_p = mem::replace(&mut self.curr_population, next_p);
        self.generation += 1;
        State {
            started_at: self.started_at.clone(),
            generation: curr_generation,
            population: Arc::new(curr_p),
            fitness_values: Arc::new(Vec::with_capacity(p_size)),
            normalized_fitness: Arc::new(Vec::with_capacity(p_size)),
            time: loop_time,
            average_fitness: self.average_fitness.clone(),
            highest_fitness: self.highest_fitness.clone(),
            lowest_fitness: self.lowest_fitness.clone(),
            best_solution: Arc::new(self.best_solution.clone()),
        }
    }

}

fn create_new_population<G>(current_population: &Vec<G>) -> Vec<G>
    where G: Genotype
{
    unimplemented!()
}
