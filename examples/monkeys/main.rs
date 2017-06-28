extern crate genevo;
extern crate rand;

use genevo::genetic::{FitnessEvaluation, PopulationGenerator};
use genevo::mutation::value::RandomValueMutator;
use genevo::recombination::discrete::MultiPointCrossover;
use genevo::reinsertion::elitist::ElitistReinserter;
use genevo::selection::truncation::MaximizeSelector;
use genevo::simulation::{Simulation, SimulationBuilder, SimResult};
use genevo::simulation::ga;
use genevo::termination::or;
use genevo::termination::limiter::{FitnessLimit, GenerationLimit};
use genevo::types::Display;
use rand::{Rng, thread_rng};
//use std::fmt::{Debug, Display, Formatter, Result};

const TARGET_TEXT: &str = "See how a genius creates a legend";
const POPULATION_SIZE: usize = 200;
const GENERATION_LIMIT: u64 = 2000;
const NUM_INDIVIDUALS_PER_PARENTS: usize = 2;
const SELECTION_RATIO: f64 = 1.0;
const NUM_CROSSOVER_POINTS: usize = 6;
const MUTATION_RATE: f64 = 0.05;
const REINSERTION_RATIO: f64 = 0.7;


/// The phenotype
type Text = String;

/// The genotype
type TextGenome = Vec<u8>;

trait AsText {
    fn as_text(&self) -> Text;
}

impl AsText for TextGenome {
    fn as_text(&self) -> Text {
        format!("{}", self.iter().fold(String::new(), |s, c| s + &(*c as char).to_string()))
    }
}

/// The fitness function for `TextGenome`s.
#[derive(Clone)]
struct FitnessCalc {}

impl FitnessEvaluation<TextGenome, usize> for FitnessCalc {

    fn fitness_of(&self, genome: &TextGenome) -> usize {
        let mut score = 0;
        for (c, t) in genome.iter().zip(TARGET_TEXT.chars()) {
            let c = *c as char;
            if c == t {
                score += 1;
            }
        }
        let fraction = score as f32 / TARGET_TEXT.len() as f32;
        (fraction * fraction * 100. + 0.5).floor() as usize
    }

    fn average(&self, fitness_values: &[usize]) -> usize {
        fitness_values.iter().sum::<usize>() / fitness_values.len()
    }

    fn highest_possible_fitness(&self) -> usize {
        100
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

/// The random population generator
struct Monkey {}

impl PopulationGenerator<TextGenome> for Monkey {
    fn generate_genotype(&self) -> TextGenome {
        let mut rng = thread_rng();
        (0..TARGET_TEXT.len()).map(|_|
            rng.gen_range(32u8, 126u8)
        ).collect()
    }
}

fn main() {

    let initial_population = Monkey{}.generate_population(POPULATION_SIZE);

    let mut monkeys_sim = ga::Simulator::builder(
        FitnessCalc {},
        MaximizeSelector::new(SELECTION_RATIO, NUM_INDIVIDUALS_PER_PARENTS),
        MultiPointCrossover::new(NUM_CROSSOVER_POINTS),
        RandomValueMutator::new(MUTATION_RATE, 32u8, 126u8),
        ElitistReinserter::new(FitnessCalc{}, true, REINSERTION_RATIO),
        or(FitnessLimit::new(FitnessCalc{}.highest_possible_fitness()),
           GenerationLimit::new(GENERATION_LIMIT))
    ).initialize(initial_population);

    loop {
        let result = monkeys_sim.step();
        match result {
            Ok(SimResult::Intermediate(result)) => {
                println!("Step: generation: {}, average_fitness: {}, \
                         best_solution: [{}], fitness: {}, processing_time: {}",
                         result.generation, result.average_fitness,
                         result.best_solution.solution.genome.as_text(),
                         result.best_solution.solution.fitness,
                         result.processing_time.fmt());
//                println!("| population: [{}]", result.population.iter().map(|g| g.as_text())
//                    .collect::<Vec<String>>().join("], ["));
            },
            Ok(SimResult::Final(result, duration, stop_reason)) => {
                println!("{}", stop_reason);
                println!("Final result after {}: generation: {}, \
                         best_solution: [{}] with fitness {} found in generation {}, processing_time: {}",
                         duration.fmt(), result.generation,
                         result.best_solution.solution.genome.as_text(),
                         result.best_solution.solution.fitness,
                         result.best_solution.generation,
                         result.processing_time.fmt());
                break;
            },
            Err(error) => {
                println!("{:?}", error);
                break;
            },
        }
    }
}
