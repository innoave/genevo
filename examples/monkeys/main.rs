extern crate genevo;
extern crate rand;

use genevo::genetic::{Fitness, FitnessEvaluation, Genotype, PopulationGenerator};
use genevo::recombination::discrete::DiscreteCrossover;
use genevo::reinsertion::random::UniformReinserter;
use genevo::selection::proportional::RouletteWheelSelector;
use genevo::simulation::{Simulation, SimulationBuilder, SimResult};
use genevo::simulation::ga;
use genevo::termination::limiter::GenerationLimit;
use genevo::types::Display;
use rand::{Rng, thread_rng};


const TARGET_TEXT: &str = "See how a genius creates a legend";
const POPULATION_SIZE: usize = 200;
const NUM_PARENTS_TO_SELECT: usize = POPULATION_SIZE / 2;
const NUM_INDIVIDUALS_PER_PARENTS: usize = 2;
const REINSERTION_RATIO: f64 = 0.9;
const GENERATION_LIMIT: u64 = 2000;


struct Text {
    text: String,
}

type TextGenome = Vec<char>;

type MonkeyBreeding = ([char], [char]);

/// Calculator for the fitness value of `TextGenome`s.
#[derive(Clone)]
struct FitnessCalc {}

impl FitnessEvaluation<TextGenome, usize> for FitnessCalc {

    fn fitness_of(&self, genome: &TextGenome) -> usize {
        genome.iter().zip(TARGET_TEXT.chars())
            .fold(0, |sum, (c, t)|
                if *c == t {
                    sum + 1
                } else {
                    sum
                }
            )
    }

    fn average(&self, fitness_values: &[usize]) -> usize {
        fitness_values.iter().sum() / fitness_values.len()
    }

    fn highest_possible_fitness(&self) -> usize {
        TARGET_TEXT.len()
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

/// This is the random population generator
struct Monkey {}

impl PopulationGenerator<TextGenome> for Monkey {
    fn generate_genotype(&self) -> TextGenome {
        let mut rng = thread_rng;
        (0..TARGET_TEXT.len()).map(|_|
            rng.gen_range(32u8, 126u8) as char
        ).collect()
    }
}

fn main() {

    let initial_population = Monkey{}.generate_population(POPULATION_SIZE);

    let monkeys_sim = ga::Simulator::builder(
        FitnessCalc {},
        RouletteWheelSelector::new(NUM_PARENTS_TO_SELECT, NUM_INDIVIDUALS_PER_PARENTS),
        DiscreteCrossover::new(),
        ValueEncodedMutator,
        UniformReinserter::new(REINSERTION_RATIO),
        GenerationLimit::new(GENERATION_LIMIT)
    ).initialize(initial_population);

    loop {
        let result = monkeys_sim.step();
        match result {
            Ok(SimResult::Intermediate(result)) => {
                println!("Step: {:?}", result);
            },
            Ok(SimResult::Final(result, duration, stop_reason)) => {
                println!("Final result after {}: {:?}, {:?}", duration.fmt(), result, stop_reason);
                break;
            },
            Err(error) => {
                println!("{:?}", error);
                break;
            },
        }
    }
}
