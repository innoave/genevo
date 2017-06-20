/// This module provides the `Simulator` which implements the genetic algorithm
/// (GA) and the related `SimulatorBuilder`.
///
/// The stages of the basic genetic algorithm are:
///
/// 1. **Initialize**: Generate random population of n genotypes (or chromosomes)
/// 2. **Fitness**: Evaluate the fitness of each genotype in the population
/// 3. **New Population**: Create a new population by repeating following steps
///    until the new population is complete:
/// 3.1. **Selection**: Select a tuple of parent genotypes from a population
///      according to their fitness and the selection strategy of the
///      configured `operator::SelectionOp`
/// 3.2. **Crossover**: With a crossover probability cross over the parents to
///      form a new offspring (children) by means of the configured
///      `operator::CrossoverOp`.
/// 3.3. **Mutation**: With a mutation probability mutate new offspring at each
///      locus (position in genotype) by means of the configured
///      `operator::MutationOp`.
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
use genetic::{Breeding, Fitness, FitnessEvaluation, Genotype, Population};
use operator::{CrossoverOp, MutationOp, SelectionOp};
use simulation::{BestSolution, Evaluated, EvaluatedPopulation, SimError, SimResult, Simulation,
                 SimulationBuilder, State};
use termination::{StopFlag, Termination};
use std::marker::PhantomData;
use std::mem;
use std::rc::Rc;

/// The `SimulationBuilder` implements the 'initialization' stage (step 1) of
/// the genetic algorithm.
pub struct SimulatorBuilder<G, F, E, S, Q, C, M, P>
    where G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, P>, Q: Termination<G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    termination: Box<Q>,
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

impl<G, F, E, S, Q, C, M, P> SimulationBuilder<Simulator<G, F, E, S, Q, C, M, P>, G, F, E, S, Q, C, M, P>
for SimulatorBuilder<G, F, E, S, Q, C, M, P>
    where G: 'static + Genotype + Send + Sync, F: 'static + Fitness + Send + Sync, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, P>, Q: Termination<G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    fn initialize(&mut self, population: Population<G>) -> Simulator<G, F, E, S, Q, C, M, P> {
        let generation = 1;
        let p_size = population.size();
        Simulator {
            evaluator: self.evaluator.clone(),
            selector: self.selector.clone(),
            breeder: self.breeder.clone(),
            mutator: self.mutator.clone(),
            termination: self.termination.clone(),
            started: false,
            started_at: Local::now(),
            generation: generation,
            population: Rc::new(population.individuals().to_vec()),
            processing_time: Duration::zero(),
            next_population: Vec::with_capacity(p_size),
            initial_population: population,
            _f: PhantomData,
            _p: PhantomData,
        }
    }
}

pub struct Simulator<G, F, E, S, Q, C, M, P>
    where G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, P>, Q: Termination<G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    termination: Box<Q>,
    initial_population: Population<G>,
    started: bool,
    started_at: DateTime<Local>,
    generation: u64,
    population: Rc<Vec<G>>,
    processing_time: Duration,
    next_population: Vec<G>,
    _p: PhantomData<P>,
    _f: PhantomData<F>,
}

impl<G, F, E, S, Q, C, M, P> Simulation<G, F, E, S, Q, C, M, P>
    for Simulator<G, F, E, S, Q, C, M, P>
    where G: 'static + Genotype + Send + Sync, F: 'static + Fitness + Send + Sync, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, P>, Q: Termination<G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    type Builder = SimulatorBuilder<G, F, E, S, Q, C, M, P>;

    fn builder(evaluator: E, selector: S, breeder: C, mutator: M, termination: Q) -> Self::Builder {
        SimulatorBuilder {
            evaluator: Box::new(evaluator),
            selector: Box::new(selector),
            breeder: Box::new(breeder),
            mutator: Box::new(mutator),
            termination: Box::new(termination),
            _g: PhantomData,
            _f: PhantomData,
            _p: PhantomData,
        }
    }

    fn run(&mut self) -> Result<SimResult<G, F>, SimError> {
        if self.started {
            return Err(SimError::SimulationAlreadyRunning(
                    format!("Simulation already running since {}", &self.started_at)));
        } else {
            self.started = true;
            self.started_at = Local::now();
        }
        unimplemented!()
    }

    fn step(&mut self) -> Result<SimResult<G, F>, SimError> {
        if self.started {
            return Err(SimError::SimulationAlreadyRunning(
                format!("Simulation already running since {}", &self.started_at)));
        } else {
            self.started = true;
            self.started_at = Local::now();
        }

        // Stages 2-4: Look at one generation
        let state = self.process_one_generation();

        // Stage 5: Be aware of the termination:
        Ok(match self.termination.evaluate(&state) {
            StopFlag::StopNow(reason) => {
                let duration = Local::now().signed_duration_since(self.started_at);
                SimResult::Final(state, duration, reason)
            },
            StopFlag::Continue => {
                SimResult::Intermediate(state)
            },
        })
    }

    fn reset(&mut self) {
        if self.started {
            return; //TODO we should not silently ignore this command -> need some error handling!
//                    Err(SimError::SimulationAlreadyRunning(
//                    format!("Simulation still running since {}. Wait for the simulation to finish \
//                        or abort it before resetting it.", time)))));
        }
        unimplemented!()
    }
}

impl<G, F, E, S, Q, C, M, P> Simulator<G, F, E, S, Q, C, M, P>
    where G: Genotype, F: Fitness, P: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, P>, Q: Termination<G, F>,
          C: CrossoverOp<P, G>, M: MutationOp<G>
{
    /// Processes stages 2-4 of the genetic algorithm
    fn process_one_generation(&mut self) -> State<G, F> {
        let loop_started_at = Local::now();
        let mut processing_time = Duration::zero();

        // Stage 2: The fitness check:
        let score_board = self.evaluate_fitness(self.population.clone());
        let best_solution = self.determine_best_solution(&score_board);

        // Stage 3: The making of a new population:
        let next_generation = self.create_new_population(&score_board);

        // Stage 4: On to the next generation:
        let loop_time = Local::now().signed_duration_since(loop_started_at);
        self.replace_generation(loop_time, processing_time, score_board, best_solution)
    }

    /// Calculates the `Fitness` value of each `Genotype` and records the
    /// highest and lowest values.
    fn evaluate_fitness(&self, population: Rc<Vec<G>>) -> EvaluatedPopulation<G, F> {
        let mut fitness = Vec::new();
        let mut highest = self.evaluator.lowest_possible_fitness();
        let mut lowest = self.evaluator.highest_possible_fitness();
        for genome in population.iter() {
            let score = self.evaluator.fitness_of(&genome);
            if score > highest {
                highest = score.clone();
            }
            if score < lowest {
                lowest = score.clone();
            }
            fitness.push(score);
        }
        let normalized = self.evaluator.normalize(&fitness);
        let average = self.evaluator.average(&fitness);
        EvaluatedPopulation {
            individuals: population,
            fitness_values: fitness,
            normalized_fitness: normalized,
            highest_fitness: highest,
            lowest_fitness: lowest,
            average_fitness: average,
        }
    }

    /// Determines the best solution of the current population
    fn determine_best_solution(&self, score_board: &EvaluatedPopulation<G, F>) -> BestSolution<G, F> {
        let index_of_best = score_board.index_of_fitness(&score_board.highest_fitness)
            .expect(&format!("No fitness value of {:?} found in this EvaluatedPopulation",
                             &score_board.highest_fitness));
        let evaluated = Evaluated {
            genome: self.population[index_of_best].clone(),
            fitness: score_board.fitness_values[index_of_best].clone(),
            normalized_fitness: score_board.normalized_fitness[index_of_best].clone(),
        };
        BestSolution {
            found_at: Local::now(),
            generation: self.generation,
            solution: evaluated,
        }
    }

    fn create_new_population(&self, evaluated_population: &EvaluatedPopulation<G, F>) -> Vec<G> {
        self.selector.selection(evaluated_population);
        unimplemented!()
    }

    /// Generates a `State` object about the last processed loop, replaces the
    /// current generation with the next generation and increases the
    /// generation counter.
    fn replace_generation(&mut self,
                          loop_time: Duration,
                          processing_time: Duration,
                          score_board: EvaluatedPopulation<G, F>,
                          best_solution: BestSolution<G, F>
                         ) -> State<G, F> {
        let curr_generation = self.generation;
        let p_size = self.next_population.len();
        let next_p = mem::replace(&mut self.next_population, Vec::with_capacity(p_size));
        let curr_p = mem::replace(&mut self.population, Rc::new(next_p));
        self.generation += 1;
        State {
            started_at: self.started_at.clone(),
            generation: curr_generation,
            population: Rc::try_unwrap(curr_p).expect("Can not unwrap Rc(Vec<G>)"),
            fitness_values: score_board.fitness_values,
            normalized_fitness: score_board.normalized_fitness,
            duration: loop_time,
            processing_time: processing_time,
            average_fitness: score_board.average_fitness,
            highest_fitness: score_board.highest_fitness,
            lowest_fitness: score_board.lowest_fitness,
            best_solution: best_solution,
        }
    }

}
