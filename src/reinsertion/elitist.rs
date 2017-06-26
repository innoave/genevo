//! The `elitist` module provides `operator::ReinsertionOp` that combine the
//! individuals from the offspring and the old population by choosing the best
//! individuals from both.

use genetic::{Fitness, FitnessEvaluation, Genotype, Offspring};
use operator::{GeneticOperator, MultiObjective, ReinsertionOp, SingleObjective};
use simulation::{EvaluatedPopulation, SimError};
use std::marker::PhantomData;


/// The `ElitistReinserter` combines the best individuals from the offspring.
/// and the old population. When there are more individuals in the offspring
/// than necessary either because the offspring is larger than the population
/// size or a replace ratio smaller then 1.0 is specified only those
/// individuals with the best fitness are taken over into the new population.
///
/// The reinserter can be configured by the `replace_ratio` field. The
/// replace ratio is the fraction of the population size that is replaced by
/// individuals from the offspring. The remaining spots are filled with
/// individuals from the old population.
///
/// A replace ratio of 1.0 means that the new population is fist filled with
/// individuals from the offspring. if the offspring does not contain enough
/// individuals then the new population is filled up with individuals from the
/// old population. If the offspring contains more individuals than the size of
/// the population then the individuals are chosen uniformly at random.
#[derive(Clone)]
pub struct ElitistReinserter<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessEvaluation<G, F>
{
    /// The `FitnessEvaluation` to be used to calculate fitness values of
    /// individuals of the offspring.
    fitness_evaluator: Box<E>,
    /// `offspring_has_precedence` defines whether individuals from offspring
    /// with lower fitness should possible replace better performing ones from
    /// the old population.
    offspring_has_precedence: bool,
    /// The `replace_ratio` defines the fraction of the population size that
    /// is going to be replaced by individuals from the offspring.
    replace_ratio: f64,
    // phantom types
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<G, F, E> ElitistReinserter<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessEvaluation<G, F>
{
    /// Constructs a new instance of the `ElitistReinserter`.
    pub fn new(fitness_evaluator: E, offspring_has_precedence: bool, replace_ratio: f64) -> Self {
        ElitistReinserter {
            fitness_evaluator: Box::new(fitness_evaluator),
            offspring_has_precedence: offspring_has_precedence,
            replace_ratio: replace_ratio,
            _g: PhantomData,
            _f: PhantomData,
        }
    }

    /// Returns true if the offspring should take precedence over better
    /// performing individuals from the old population.
    pub fn is_offspring_has_precedence(&self) -> bool {
        self.offspring_has_precedence
    }

    /// Sets whether the offspring should have precedence over better
    /// performing individuals from the old population.
    pub fn set_offspring_has_precedence(&mut self, value: bool) {
        self.offspring_has_precedence = value;
    }

    /// Returns the `replace_ratio` of this `ElitistReinserter`.
    pub fn replace_ratio(&self) -> f64 {
        self.replace_ratio
    }

    /// Set the `replace_ratio` of this `ElitistReinserter` to the given
    /// value. The value must be between 0 and 1.0 (inclusive).
    pub fn set_replace_ratio(&mut self, value: f64) {
        self.replace_ratio = value;
    }
}

impl<G, F, E> GeneticOperator for ElitistReinserter<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessEvaluation<G, F>
{
    fn name() -> String {
        "Uniform-Reinserter".to_string()
    }
}

/// Can be used for single-objective optimization
impl<G, F, E> SingleObjective for ElitistReinserter<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessEvaluation<G, F> {}
/// Can be used for multi-objective optimization
impl<G, F, E> MultiObjective for ElitistReinserter<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessEvaluation<G, F> {}

impl<G, F, E> ReinsertionOp<G, F> for ElitistReinserter<G, F, E>
    where G: Genotype, F: Fitness, E: FitnessEvaluation<G, F>
{
    fn combine(&self, offspring: &mut Offspring<G>, evaluated: &EvaluatedPopulation<G, F>)
               -> Result<Vec<G>, SimError> {
        let old_individuals = evaluated.individuals();
        let old_fitness_values = evaluated.fitness_values();
        // holds indices to the individuals and fitness_values slices
        let mut old_population_indices: Vec<usize> = (0..old_fitness_values.len()).collect();
        // sort fitness indices from best performing to worst performing index
        old_population_indices.sort_by(|x, y| old_fitness_values[*y].cmp(&old_fitness_values[*x]));

        let population_size = old_individuals.len();
        let mut new_population: Vec<G> = Vec::with_capacity(population_size);

        // How many individuals should we take from the offspring?
        let num_offspring = (population_size as f64 / self.replace_ratio).ceil() as usize;

        if self.offspring_has_precedence {
            // first pick individuals from offspring
            if num_offspring < offspring.len() {
                // evaluate fitness of the offspring individuals
                let offspring_fitness: Vec<F> = offspring.iter().map(|child|
                    self.fitness_evaluator.fitness_of(child)).collect();
                // holds indices to the individuals and fitness values of offspring
                let mut offspring_indices: Vec<usize> = (0..offspring_fitness.len()).collect();
                // sort offspring indices from best performing to worst performing
                offspring_indices.sort_by(|x, y| offspring_fitness[*y].cmp(&offspring_fitness[*x]));
                // pick only the best individuals from the offspring
                while num_offspring > new_population.len() {
                    new_population.push(offspring.remove(0));
                }
            } else {
                // insert all individuals from offspring
                for i in 0..offspring.len() {
                    new_population.push(offspring.remove(i));
                }
            }
            //
            // finally fill up new population with individuals from old population
            //
            let num_old_population = population_size - new_population.len();
            for i in 0..num_old_population {
                // pick only the best individuals from old population
                let index_old = old_population_indices[i];
                new_population.push(old_individuals[index_old].clone());
            }
        } else {
            // evaluate fitness of the offspring individuals
            let offspring_fitness: Vec<F> = offspring.iter().map(|child|
                self.fitness_evaluator.fitness_of(child)).collect();
            // holds indices to the individuals and fitness values of offspring
            let mut offspring_indices: Vec<usize> = (0..offspring_fitness.len()).collect();
            // sort offspring indices from best performing to worst performing
            offspring_indices.sort_by(|x, y| offspring_fitness[*y].cmp(&offspring_fitness[*x]));
            for _ in 0..population_size {
                // compare fitness of best offspring with best fitness of old population
                let index_old = old_population_indices[0];
                if !offspring_indices.is_empty()
                    && offspring_fitness[offspring_indices[0]] > old_fitness_values[index_old] {
                    // insert best from offspring
                    new_population.push(offspring.remove(old_population_indices[0]));
                    offspring_indices.remove(0);
                } else {
                    // insert best from old population
                    new_population.push(old_individuals[index_old].clone());
                    old_population_indices.remove(0);
                }
            }
        }
        Ok(new_population)
    }
}
