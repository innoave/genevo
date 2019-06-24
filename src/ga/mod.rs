//! This module provides an `algorithm::Algorithm` which implements the genetic
//! algorithm (GA).
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

pub mod builder;

use self::builder::EmptyGeneticAlgorithmBuilder;
use crate::{
    algorithm::{Algorithm, BestSolution, EvaluatedPopulation},
    genetic::{Fitness, FitnessFunction, Genotype, Offspring, Parents},
    operator::{CrossoverOp, MutationOp, ReinsertionOp, SelectionOp},
    population::Population,
    random::{Prng, RngJump},
    statistic::{timed, ProcessingTime, TimedResult, TrackProcessingTime},
};
use chrono::Local;
use rayon;
use std::{marker::PhantomData, mem, rc::Rc};

/// The `State` struct holds the results of one pass of the genetic algorithm
/// loop, i.e. the processing of the evolution from one generation to the next
/// generation.
#[derive(Clone, Debug, PartialEq)]
pub struct State<G, F>
where
    G: Genotype,
    F: Fitness,
{
    /// The evaluated population of the current generation.
    pub evaluated_population: EvaluatedPopulation<G, F>,
    /// Best solution of this generation.
    pub best_solution: BestSolution<G, F>,
    /// Accumulated time spent by each thread in case of parallel processing.
    pub processing_time: ProcessingTime,
}

/// An error that can occur during execution of a `GeneticAlgorithm`.
#[derive(Clone, Debug, PartialEq)]
pub enum GeneticAlgorithmError {
    /// The algorithm is run with an empty population.
    EmptyPopulation(String),
    /// The algorithm is run with an population size that is smaller than the
    /// required minimum.
    PopulationTooSmall(String),
}

pub fn genetic_algorithm<G, F>() -> EmptyGeneticAlgorithmBuilder<G, F>
where
    G: Genotype,
    F: Fitness,
{
    EmptyGeneticAlgorithmBuilder::new()
}

/// A `GeneticAlgorithm` declares the building blocks that make up the actual
/// algorithm for a specific optimization problem.
#[derive(Clone, Debug, PartialEq)]
pub struct GeneticAlgorithm<G, F, E, S, C, M, R>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
    S: SelectionOp<G, F>,
    C: CrossoverOp<G>,
    M: MutationOp<G>,
    R: ReinsertionOp<G, F>,
{
    _f: PhantomData<F>,
    evaluator: E,
    selector: S,
    breeder: C,
    mutator: M,
    reinserter: R,
    min_population_size: usize,
    initial_population: Population<G>,
    population: Rc<Vec<G>>,
    processing_time: ProcessingTime,
}

impl<G, F, E, S, C, M, R> GeneticAlgorithm<G, F, E, S, C, M, R>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
    S: SelectionOp<G, F>,
    C: CrossoverOp<G>,
    M: MutationOp<G>,
    R: ReinsertionOp<G, F>,
{
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

    pub fn min_population_size(&self) -> usize {
        self.min_population_size
    }
}

impl<G, F, E, S, C, M, R> TrackProcessingTime for GeneticAlgorithm<G, F, E, S, C, M, R>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
    S: SelectionOp<G, F>,
    C: CrossoverOp<G>,
    M: MutationOp<G>,
    R: ReinsertionOp<G, F>,
{
    fn processing_time(&self) -> &ProcessingTime {
        &self.processing_time
    }
}

impl<G, F, E, S, C, M, R> Algorithm for GeneticAlgorithm<G, F, E, S, C, M, R>
where
    G: Genotype,
    F: Fitness + Send + Sync,
    E: FitnessFunction<G, F> + Sync,
    S: SelectionOp<G, F>,
    C: CrossoverOp<G> + Sync,
    M: MutationOp<G> + Sync,
    R: ReinsertionOp<G, F>,
{
    type Output = State<G, F>;
    type Error = GeneticAlgorithmError;

    fn reset(&mut self) -> Result<bool, Self::Error> {
        self.processing_time = ProcessingTime::zero();
        self.population = Rc::new(self.initial_population.individuals().to_vec());
        Ok(true)
    }

    fn next(&mut self, iteration: u64, rng: &mut Prng) -> Result<Self::Output, Self::Error> {
        if self.population.is_empty() {
            return Err(GeneticAlgorithmError::EmptyPopulation(format!(
                "Population of generation {} is empty. The required minimum size for \
                 populations is {}.",
                iteration, self.min_population_size
            )));
        }
        if self.population.len() < self.min_population_size {
            return Err(GeneticAlgorithmError::PopulationTooSmall(format!(
                "Population of generation {} has a size of {} which is smaller than the \
                 required minimum size of {}",
                iteration,
                self.population.len(),
                self.min_population_size
            )));
        }

        // Stage 2: The fitness check:
        let evaluation = evaluate_fitness(self.population.clone(), &self.evaluator);
        let best_solution = determine_best_solution(iteration, &evaluation.result);

        // Stage 3: The making of a new population:
        let selection = timed(|| self.selector.select_from(&evaluation.result, rng)).run();
        let mut breeding = par_breed_offspring(selection.result, &self.breeder, &self.mutator, rng);
        let reinsertion = timed(|| {
            self.reinserter
                .combine(&mut breeding.result, &evaluation.result, rng)
        })
        .run();

        // Stage 4: On to the next generation:
        let loop_time = evaluation.time
            + best_solution.time
            + selection.time
            + breeding.time
            + reinsertion.time;
        self.processing_time += loop_time;
        let next_generation = reinsertion.result;
        mem::replace(&mut self.population, Rc::new(next_generation));
        Ok(State {
            evaluated_population: evaluation.result,
            best_solution: best_solution.result,
            processing_time: self.processing_time.clone(),
        })
    }
}

fn evaluate_fitness<G, F, E>(
    population: Rc<Vec<G>>,
    evaluator: &E,
) -> TimedResult<EvaluatedPopulation<G, F>>
where
    G: Genotype + Sync,
    F: Fitness + Send + Sync,
    E: FitnessFunction<G, F> + Sync,
{
    let evaluation = par_evaluate_fitness(&population, evaluator);
    let average = timed(|| evaluator.average(&evaluation.result.0)).run();
    let evaluated = EvaluatedPopulation::new(
        population,
        evaluation.result.0,
        evaluation.result.1,
        evaluation.result.2,
        average.result,
    );
    TimedResult {
        result: evaluated,
        time: evaluation.time + average.time,
    }
}

/// Calculates the `genetic::Fitness` value of each `genetic::Genotype` and
/// records the highest and lowest values.
fn par_evaluate_fitness<G, F, E>(population: &[G], evaluator: &E) -> TimedResult<(Vec<F>, F, F)>
where
    G: Genotype + Sync,
    F: Fitness + Send + Sync,
    E: FitnessFunction<G, F> + Sync,
{
    if population.len() < 60 {
        timed(|| {
            let mut fitness = Vec::with_capacity(population.len());
            let mut highest = evaluator.lowest_possible_fitness();
            let mut lowest = evaluator.highest_possible_fitness();
            for genome in population.iter() {
                let score = evaluator.fitness_of(genome);
                if score > highest {
                    highest = score.clone();
                }
                if score < lowest {
                    lowest = score.clone();
                }
                fitness.push(score);
            }
            (fitness, highest, lowest)
        })
        .run()
    } else {
        let mid_point = population.len() / 2;
        let (l_slice, r_slice) = population.split_at(mid_point);
        let (mut left, mut right) = rayon::join(
            || par_evaluate_fitness(l_slice, evaluator),
            || par_evaluate_fitness(r_slice, evaluator),
        );
        let mut fitness = Vec::with_capacity(population.len());
        fitness.append(&mut left.result.0);
        fitness.append(&mut right.result.0);
        let highest = if left.result.1 >= right.result.1 {
            left.result.1
        } else {
            right.result.1
        };
        let lowest = if left.result.2 <= right.result.2 {
            left.result.2
        } else {
            right.result.2
        };
        TimedResult {
            result: (fitness, highest, lowest),
            time: left.time + right.time,
        }
    }
}

/// Determines the best solution of the current population
fn determine_best_solution<G, F>(
    generation: u64,
    score_board: &EvaluatedPopulation<G, F>,
) -> TimedResult<BestSolution<G, F>>
where
    G: Genotype,
    F: Fitness,
{
    timed(|| {
        let evaluated = score_board
            .evaluated_individual_with_fitness(&score_board.highest_fitness())
            .expect(&format!(
                "No fitness value of {:?} found in this EvaluatedPopulation",
                &score_board.highest_fitness()
            ));
        BestSolution {
            found_at: Local::now(),
            generation,
            solution: evaluated,
        }
    })
    .run()
}

/// Lets the parents breed their offspring and mutate its children. And
/// finally combines the offspring of all parents into one big offspring.
fn par_breed_offspring<G, C, M>(
    parents: Vec<Parents<G>>,
    breeder: &C,
    mutator: &M,
    rng: &mut Prng,
) -> TimedResult<Offspring<G>>
where
    G: Genotype + Send,
    C: CrossoverOp<G> + Sync,
    M: MutationOp<G> + Sync,
{
    if parents.len() < 60 {
        timed(|| {
            let mut offspring: Offspring<G> = Vec::with_capacity(parents.len() * parents[0].len());
            for parents in parents {
                let children = breeder.crossover(parents, rng);
                for child in children {
                    let mutated = mutator.mutate(child, rng);
                    offspring.push(mutated);
                }
            }
            offspring
        })
        .run()
    } else {
        let mut rng1 = rng.clone();
        rng1.jump(1);
        let mut rng2 = rng.clone();
        rng2.jump(2);
        let mid_point = parents.len() / 2;
        let mut offspring = Vec::with_capacity(parents.len() * 2);
        let mut parents = parents;
        let r_slice = parents.drain(mid_point..).collect();
        let l_slice = parents;
        let (mut left, mut right) = rayon::join(
            || par_breed_offspring(l_slice, breeder, mutator, &mut rng1),
            || par_breed_offspring(r_slice, breeder, mutator, &mut rng2),
        );
        offspring.append(&mut left.result);
        offspring.append(&mut right.result);
        TimedResult {
            result: offspring,
            time: left.time + right.time,
        }
    }
}
