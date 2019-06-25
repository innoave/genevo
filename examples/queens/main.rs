//! The `queens` example searches for solutions of the
//! [N Queens Problem](https://en.wikipedia.org/wiki/Eight_queens_puzzle)

use genevo::{operator::prelude::*, prelude::*, random::Rng, types::fmt::Display};

const NUMBER_OF_QUEENS: i16 = 16;
const NUM_ROWS: i16 = NUMBER_OF_QUEENS;
const NUM_COLS: i16 = NUMBER_OF_QUEENS;
const POPULATION_SIZE: usize = 200;
const GENERATION_LIMIT: u64 = 2000;
const NUM_INDIVIDUALS_PER_PARENTS: usize = 3;
const SELECTION_RATIO: f64 = 0.7;
const MUTATION_RATE: f64 = 0.05;
const REINSERTION_RATIO: f64 = 0.7;

/// The phenotype
type Queen = String;
type Board = Vec<Vec<Queen>>;

/// The genotype
#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Pos {
    x: i16,
    y: i16,
}
type Positions = Vec<Pos>;

/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn as_board(&self) -> Board;
}

impl AsPhenotype for Positions {
    fn as_board(&self) -> Board {
        (0..NUM_ROWS)
            .map(|row| {
                (0..NUM_COLS)
                    .map(|col| self.iter().any(|&Pos { x, y }| x == row && y == col))
                    .map(|q| if q { "Q" } else { " " }.to_string())
                    .collect::<Vec<Queen>>()
            })
            .collect::<Vec<Vec<Queen>>>()
    }
}

fn count_collisions(positions: &Positions) -> i16 {
    let mut count = 0;
    for (i, i_pos) in positions.iter().enumerate() {
        for (j, j_pos) in positions.iter().enumerate() {
            if i != j
                && (i_pos.x == j_pos.x
                    || i_pos.y == j_pos.y
                    || i_pos.x + i_pos.y == j_pos.x + j_pos.y
                    || i_pos.x - i_pos.y == j_pos.x - j_pos.y)
            {
                count += 1;
            }
        }
    }
    count
}

/// The fitness function for `Positions`.
#[derive(Clone, Debug)]
struct FitnessCalc;

impl FitnessFunction<Positions, usize> for FitnessCalc {
    fn fitness_of(&self, positions: &Positions) -> usize {
        let collisions = count_collisions(positions);
        let max_collisions = (NUMBER_OF_QUEENS - 1) * (NUMBER_OF_QUEENS - 1);
        let score = (max_collisions - collisions) as f32 / (max_collisions + collisions) as f32;
        (score * score * 100. + 0.5).floor() as usize
    }

    fn average(&self, values: &[usize]) -> usize {
        (values.iter().sum::<usize>() as f32 / values.len() as f32 + 0.5).floor() as usize
    }

    fn highest_possible_fitness(&self) -> usize {
        100
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

impl BreederValueMutation for Pos {
    fn breeder_mutated(value: Self, range: &Pos, adjustment: f64, sign: i8) -> Self {
        Pos {
            x: value.x,
            y: value.y + (range.y as f64 * adjustment * sign as f64) as i16,
        }
    }
}

impl RandomValueMutation for Pos {
    fn random_mutated<R>(value: Self, min_value: &Pos, max_value: &Pos, rng: &mut R) -> Self
    where
        R: Rng + Sized,
    {
        Pos {
            x: value.x,
            y: rng.gen_range(min_value.y, max_value.y),
        }
    }
}

/// Generate some random boards
struct QueensPositions;

impl GenomeBuilder<Positions> for QueensPositions {
    fn build_genome<R>(&self, _: usize, rng: &mut R) -> Positions
    where
        R: Rng + Sized,
    {
        (0..NUM_ROWS)
            .map(|row| Pos {
                x: row,
                y: rng.gen_range(0, NUM_COLS),
            })
            .collect()
    }
}

fn main() {
    let initial_population: Population<Positions> = build_population()
        .with_genome_builder(QueensPositions)
        .of_size(POPULATION_SIZE)
        .uniform_at_random();

    let mut queens_sim = simulate(
        genetic_algorithm()
            .with_evaluation(FitnessCalc)
            .with_selection(RouletteWheelSelector::new(
                SELECTION_RATIO,
                NUM_INDIVIDUALS_PER_PARENTS,
            ))
            .with_crossover(UniformCrossBreeder::new())
            .with_mutation(BreederValueMutator::new(
                MUTATION_RATE,
                Pos { x: 0, y: 1 },
                3,
                Pos { x: 0, y: 0 },
                Pos {
                    x: NUM_ROWS,
                    y: NUM_COLS,
                },
            ))
            .with_reinsertion(ElitistReinserter::new(
                FitnessCalc,
                false,
                REINSERTION_RATIO,
            ))
            .with_initial_population(initial_population)
            .build(),
    )
    .until(or(
        FitnessLimit::new(FitnessCalc.highest_possible_fitness()),
        GenerationLimit::new(GENERATION_LIMIT),
    ))
    .build();

    loop {
        let result = queens_sim.step();
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!(
                    "Step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt()
                );
                for row in best_solution.solution.genome.as_board() {
                    println!("      {:?}", row);
                }
            },
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!(
                    "Final result after {}: generation: {}, \
                     best solution with fitness {} found in generation {}, processing_time: {}",
                    duration.fmt(),
                    step.iteration,
                    best_solution.solution.fitness,
                    best_solution.generation,
                    processing_time.fmt()
                );
                for row in best_solution.solution.genome.as_board() {
                    println!("      {:?}", row);
                }
                break;
            },
            Err(error) => {
                println!("{}", error);
                break;
            },
        }
    }
}
