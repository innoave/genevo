//! This module provides the `Simulator` which implements the genetic algorithm
//! (GA) and the related `SimulatorBuilder`.
//!
//! The stages of the basic genetic algorithm are:
//!
//! 1. **Initialize**: Generate random population of n genotypes (or chromosomes)
//! 2. **Fitness**: Evaluate the fitness of each genotype in the population
//! 3. **New Population**: Create a new population by repeating following steps
//!    until the new population is complete:
//! 3.1. **Selection**: Select a tuple of parent genotypes from a population
//!      according to their fitness and the selection strategy of the
//!      configured `operator::SelectionOp`
//! 3.2. **Crossover**: With a crossover probability cross over the parents to
//!      form a new offspring (child) by means of the configured
//!      `operator::CrossoverOp`.
//! 3.3. **Mutation**: With a mutation probability mutate new offspring at each
//!      locus (position in genotype) by means of the configured
//!      `operator::MutationOp`.
//! 3.4. **Accepting**: Place new offspring in the new population.
//! 4. **Replace**: Use new generated population for a further run of the
//!    algorithm.
//! 5. **Termination**: If the end condition is satisfied, stop, and return the
//!    best solution in current population.
//! 6. **Loop**: Go to step 2
//!
//! The `Simulator` implements the `simulation::Simulation` trait. The
//! `SimulatorBuilder` implements the `simulation::SimulationBuilder` trait.

use chrono::{DateTime, Duration, Local};
use genetic::{Breeding, Fitness, FitnessEvaluation, Genotype, Population};
use operator::{CrossoverOp, MutationOp, ReinsertionOp, SelectionOp};
use simulation::{BestSolution, Evaluated, EvaluatedPopulation, SimError, SimResult, Simulation,
                 SimulationBuilder, State};
use termination::{StopFlag, Termination};
use std::marker::PhantomData;
use std::mem;
use std::rc::Rc;

//TODO make MIN_POPULATION_SIZE a parameter of the Simulator
const MIN_POPULATION_SIZE: usize = 7;

/// The `RunMode` identifies whether the simulation is running and how it has
/// been started.
enum RunMode {
    /// The simulation is running in loop mode. i.e. it was started by calling
    /// the `run` function.
    Loop,
    /// The simulation is running in step mode. i.e. it was started by calling
    /// the `step` function.
    Step,
    /// The simulation is not running.
    NotRunning,
}

/// The `SimulationBuilder` implements the 'initialization' stage (step 1) of
/// the genetic algorithm.
pub struct SimulatorBuilder<G, F, E, S, B, C, M, R, Q>
    where G: Genotype, F: Fitness, B: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, B>, Q: Termination<G, F>,
          C: CrossoverOp<B, G>, M: MutationOp<G>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    reinserter: Box<R>,
    termination: Box<Q>,
    _g: PhantomData<G>,
    _f: PhantomData<F>,
    _p: PhantomData<B>,
}

impl<G, F, E, S, B, C, M, R, Q> SimulationBuilder<Simulator<G, F, E, S, B, C, M, R, Q>, G, F, E, S, B, C, M, R, Q>
    for SimulatorBuilder<G, F, E, S, B, C, M, R, Q>
    where G: Genotype + Send + Sync, F: Fitness + Send + Sync, B: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, B>, Q: Termination<G, F>,
          C: CrossoverOp<B, G>, M: MutationOp<G>, R: ReinsertionOp<G, F>
{
    fn initialize(&mut self, population: Population<G>) -> Simulator<G, F, E, S, B, C, M, R, Q> {
        Simulator {
            evaluator: self.evaluator.clone(),
            selector: self.selector.clone(),
            breeder: self.breeder.clone(),
            mutator: self.mutator.clone(),
            reinserter: self.reinserter.clone(),
            termination: self.termination.clone(),
            run_mode: RunMode::NotRunning,
            started_at: Local::now(),
            generation: 1,
            population: Rc::new(population.individuals().to_vec()),
            processing_time: Duration::zero(),
            finished: false,
            initial_population: population,
            _f: PhantomData,
            _p: PhantomData,
        }
    }
}

pub struct Simulator<G, F, E, S, B, C, M, R, Q>
    where G: Genotype, F: Fitness, B: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, B>, Q: Termination<G, F>,
          C: CrossoverOp<B, G>, M: MutationOp<G>, R: ReinsertionOp<G, F>
{
    evaluator: Box<E>,
    selector: Box<S>,
    breeder: Box<C>,
    mutator: Box<M>,
    reinserter: Box<R>,
    termination: Box<Q>,
    initial_population: Population<G>,
    run_mode: RunMode,
    started_at: DateTime<Local>,
    generation: u64,
    population: Rc<Vec<G>>,
    processing_time: Duration,
    finished: bool,
    _p: PhantomData<B>,
    _f: PhantomData<F>,
}

impl<G, F, E, S, B, C, M, R, Q> Simulation<G, F, E, S, B, C, M, R, Q>
    for Simulator<G, F, E, S, B, C, M, R, Q>
    where G: Genotype + Send + Sync, F: Fitness + Send + Sync, B: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, B>, Q: Termination<G, F>,
          C: CrossoverOp<B, G>, M: MutationOp<G>, R: ReinsertionOp<G, F>
{
    type Builder = SimulatorBuilder<G, F, E, S, B, C, M, R, Q>;

    fn builder(evaluator: E, selector: S, breeder: C, mutator: M, reinserter: R, termination: Q)
        -> Self::Builder {
        SimulatorBuilder {
            evaluator: Box::new(evaluator),
            selector: Box::new(selector),
            breeder: Box::new(breeder),
            mutator: Box::new(mutator),
            reinserter: Box::new(reinserter),
            termination: Box::new(termination),
            _g: PhantomData,
            _f: PhantomData,
            _p: PhantomData,
        }
    }

    fn run(&mut self) -> Result<SimResult<G, F>, SimError> {
        match self.run_mode {
            RunMode::Loop =>
                return Err(SimError::SimulationAlreadyRunning(
                    format!("Simulation already running in loop since {}", &self.started_at))),
            RunMode::Step =>
                return Err(SimError::SimulationAlreadyRunning(
                    format!("Simulation already running in step mode since {}", &self.started_at))),
            RunMode::NotRunning => {
                self.run_mode = RunMode::Loop;
                self.started_at = Local::now();
            },
        }
        let mut result = Err(SimError::UnexpectedError("Unexpected error! \
                             No loop of the simulation has ever been processed!".to_string()));
        self.finished = false;
        while !self.finished {
            // Stages 2-4: Look at one generation
            result = self.process_one_generation().and_then(|state| {
                // Stage 5: Be aware of the termination:
                Ok(match self.termination.evaluate(&state) {
                    StopFlag::Continue => {
                        SimResult::Intermediate(state)
                    },
                    StopFlag::StopNow(reason) => {
                        self.finished = true;
                        let duration = Local::now().signed_duration_since(self.started_at);
                        SimResult::Final(state, duration, reason)
                    },
                })
            }).or_else(|error| {
                self.finished = true;
                Err(error)
            });
        }
        self.run_mode = RunMode::NotRunning;
        result
    }

    fn step(&mut self) -> Result<SimResult<G, F>, SimError> {
        match self.run_mode {
            RunMode::Loop =>
                return Err(SimError::SimulationAlreadyRunning(
                    format!("Simulation already running in loop since {}", &self.started_at))),
            RunMode::Step => (),
            RunMode::NotRunning => {
                    self.run_mode = RunMode::Step;
                    self.started_at = Local::now();
                },
        }
        if self.population.len() < MIN_POPULATION_SIZE {
            return Err(SimError::PopulationTooSmall(
                format!("Population of generation {} has a size of {} which is smaller than the \
                        required minimum size of {}",
                        self.generation, self.initial_population.size(), MIN_POPULATION_SIZE)))
        }

        // Stages 2-4: Look at one generation
        self.process_one_generation().and_then(|state|

            // Stage 5: Be aware of the termination:
            Ok(match self.termination.evaluate(&state) {
                StopFlag::Continue => {
                    SimResult::Intermediate(state)
                },
                StopFlag::StopNow(reason) => {
                    let duration = Local::now().signed_duration_since(self.started_at);
                    self.run_mode = RunMode::NotRunning;
                    SimResult::Final(state, duration, reason)
                },
            })
        )
    }

    fn stop(&mut self) -> Result<bool, SimError> {
        match self.run_mode {
            RunMode::Loop => {
                self.finished = true;
                Ok(true)
            },
            RunMode::Step => {
                self.finished = true;
                Ok(true)
            },
            RunMode::NotRunning =>
                Ok(false)
        }
    }

    fn reset(&mut self) -> Result<bool, SimError> {
        match self.run_mode {
            RunMode::Loop =>
                return Err(SimError::SimulationAlreadyRunning(
                    format!("Simulation still running in loop mode since {}. Wait for the \
                             simulation to finish or stop it before resetting it.",
                            &self.started_at))),
            RunMode::Step =>
                return Err(SimError::SimulationAlreadyRunning(
                    format!("Simulation still running in step mode since {}. Wait for the \
                             simulation to finish or stop it before resetting it.",
                             &self.started_at))),
            RunMode::NotRunning => (),
        }
        self.run_mode = RunMode::NotRunning;
        self.processing_time = Duration::zero();
        self.generation = 1;
        self.population = Rc::new(self.initial_population.individuals().to_vec());
        Ok(true)
    }
}

impl<G, F, E, S, B, C, M, R, Q> Simulator<G, F, E, S, B, C, M, R, Q>
    where G: Genotype, F: Fitness, B: Breeding<G>,
          E: FitnessEvaluation<G, F>, S: SelectionOp<G, F, B>, Q: Termination<G, F>,
          C: CrossoverOp<B, G>, M: MutationOp<G>, R: ReinsertionOp<G, F>
{
    /// Processes stages 2-4 of the genetic algorithm
    fn process_one_generation(&mut self) -> Result<State<G, F>, SimError> {
        let loop_started_at = Local::now();

        // Stage 2: The fitness check:
        let (score_board, eval_proc_time1) = self.evaluate_fitness(self.population.clone());
        let (best_solution, eval_proc_time2) = self.determine_best_solution(&score_board);

        // Stage 3: The making of a new population:
        let (next_generation, new_pop_proc_time) = self.create_new_population(&score_board);
        next_generation.and_then(|next_generation| {

            // Stage 4: On to the next generation:
            let loop_processing_time = eval_proc_time1 + eval_proc_time2 + new_pop_proc_time;
            self.processing_time = self.processing_time + loop_processing_time;
            let loop_duration = Local::now().signed_duration_since(loop_started_at);
            Ok(self.replace_generation(loop_duration, loop_processing_time, score_board, best_solution,
                                    next_generation))
        })
    }

    /// Calculates the `Fitness` value of each `Genotype` and records the
    /// highest and lowest values.
    fn evaluate_fitness<'a>(&self, population: Rc<Vec<G>>) -> (EvaluatedPopulation<G, F>, Duration) {
        let started_at = Local::now();
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
        let average = self.evaluator.average(&fitness);
        (EvaluatedPopulation {
            individuals: population,
            fitness_values: fitness,
            highest_fitness: highest,
            lowest_fitness: lowest,
            average_fitness: average,
        },
        Local::now().signed_duration_since(started_at))
    }

    /// Determines the best solution of the current population
    fn determine_best_solution(&self, score_board: &EvaluatedPopulation<G, F>)
        -> (BestSolution<G, F>, Duration) {
        let started_at = Local::now();
        let index_of_best = score_board.index_of_fitness(&score_board.highest_fitness)
            .expect(&format!("No fitness value of {:?} found in this EvaluatedPopulation",
                             &score_board.highest_fitness));
        let evaluated = Evaluated {
            genome: self.population[index_of_best].clone(),
            fitness: score_board.fitness_values[index_of_best].clone(),
        };
        (BestSolution {
            found_at: Local::now(),
            generation: self.generation,
            solution: evaluated,
        },
        Local::now().signed_duration_since(started_at))
    }

    /// Creates a new population which is derived from the current population
    /// applying 'Selection', 'Crossover' and 'Mutation'.
    fn create_new_population(&self, evaluated_population: &EvaluatedPopulation<G, F>)
        -> (Result<Vec<G>, SimError>, Duration) {
        let started_at = Local::now();
        let new_population = self.selector.selection(evaluated_population)
            .and_then(|selection| selection.iter()
                .map(|parents| self.breeder.crossover(&parents)
                    .and_then(|offspring| self.mutator.mutate(&offspring))
            ).collect())
            .and_then(|mut offspring| self.reinserter.combine(&mut offspring, evaluated_population));
        (new_population, Local::now().signed_duration_since(started_at))
    }

    /// Generates a `State` object about the last processed evolution, replaces the
    /// current generation with the next generation and increases the
    /// generation counter.
    fn replace_generation(&mut self,
                          loop_time: Duration,
                          processing_time: Duration,
                          score_board: EvaluatedPopulation<G, F>,
                          best_solution: BestSolution<G, F>,
                          next_population: Vec<G>,
                         ) -> State<G, F> {
        let curr_generation = self.generation;
        let curr_p = mem::replace(&mut self.population, Rc::new(next_population));
        let curr_p = Rc::try_unwrap(curr_p).expect("Can not unwrap Rc(Vec<G>)");
        self.generation += 1;
        State {
            started_at: self.started_at.clone(),
            generation: curr_generation,
            population: curr_p,
            fitness_values: score_board.fitness_values,
            duration: loop_time,
            processing_time: processing_time,
            average_fitness: score_board.average_fitness,
            highest_fitness: score_board.highest_fitness,
            lowest_fitness: score_board.lowest_fitness,
            best_solution: best_solution,
        }
    }
}
