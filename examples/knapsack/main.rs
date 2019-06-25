//! The 0-1 knapsack problem example searches for the combination of items that
//! sums up to the greatest possible value while the total weight is still below
//! or equal the allowed weight of the knapsack.
//!
//! [knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem)

use genevo::{operator::prelude::*, population::*, prelude::*, types::fmt::Display};
use smallvec::SmallVec;

#[derive(Debug, Clone)]
struct Item {
    value: i32,
    weight: u32,
}

#[derive(Debug, Clone)]
struct GivenItems {
    list: Vec<Item>,
}

impl From<Vec<Item>> for GivenItems {
    fn from(items: Vec<Item>) -> Self {
        GivenItems { list: items }
    }
}

/// The phenotype
#[derive(Debug)]
struct Knapsack {
    items: Vec<Item>,
    value: i64,
    weight: u64,
}

/// The genotype
type Selection = SmallVec<[bool; 16]>;

/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn as_knapsack(&self, given_items: &GivenItems) -> Knapsack;
}

impl AsPhenotype for Selection {
    fn as_knapsack(&self, given_items: &GivenItems) -> Knapsack {
        let items: Vec<Item> = self
            .into_iter()
            .enumerate()
            .filter_map(|(index, selected)| {
                if *selected {
                    Some(given_items.list[index].clone())
                } else {
                    None
                }
            })
            .collect();

        let value = items.iter().map(|i| i.value as i64).sum::<i64>();
        let weight = items.iter().map(|i| i.weight as u64).sum::<u64>();

        Knapsack {
            items,
            value,
            weight,
        }
    }
}

/// The problem definition
#[derive(Debug, Clone)]
struct Problem {
    given_items: GivenItems,
    allowed_weight: u64,
    highest_possible_fitness: i64,
}

impl Problem {
    pub fn new(allowed_weight: u64, given_items: GivenItems) -> Self {
        let highest_possible_fitness = given_items
            .list
            .iter()
            .map(|item| i64::from(item.value))
            .sum::<i64>();
        Self {
            given_items,
            allowed_weight,
            highest_possible_fitness,
        }
    }
}

/// The fitness function for `Selection`
impl<'a> FitnessFunction<Selection, i64> for &'a Problem {
    fn fitness_of(&self, selection: &Selection) -> i64 {
        let (total_weight, total_value) = selection
            .iter()
            .enumerate()
            .filter_map(|(index, selected)| {
                if *selected {
                    let item = &self.given_items.list[index];
                    Some((u64::from(item.weight), i64::from(item.value)))
                } else {
                    None
                }
            })
            .fold((0, 0), |(acc_weight, acc_value), (weight, value)| {
                (acc_weight + weight, acc_value + value)
            });
        if total_weight > self.allowed_weight {
            0
        } else {
            total_value
        }
    }

    fn average(&self, values: &[i64]) -> i64 {
        (values.iter().sum::<i64>() as f32 / values.len() as f32 + 0.5).floor() as i64
    }

    fn highest_possible_fitness(&self) -> i64 {
        self.highest_possible_fitness
    }

    fn lowest_possible_fitness(&self) -> i64 {
        0
    }
}

fn main() {
    let given_items: GivenItems = vec![
        Item {
            value: 12,
            weight: 12,
        },
        Item {
            value: 15,
            weight: 16,
        },
        Item {
            value: 18,
            weight: 11,
        },
        Item {
            value: 21,
            weight: 15,
        },
        Item {
            value: 22,
            weight: 18,
        },
        Item {
            value: 25,
            weight: 21,
        },
        Item {
            value: 28,
            weight: 29,
        },
        Item {
            value: 31,
            weight: 41,
        },
        Item {
            value: 34,
            weight: 39,
        },
        Item {
            value: 37,
            weight: 35,
        },
        Item {
            value: 40,
            weight: 42,
        },
        Item {
            value: 43,
            weight: 44,
        },
        Item {
            value: 46,
            weight: 33,
        },
        Item {
            value: 49,
            weight: 64,
        },
        Item {
            value: 50,
            weight: 80,
        },
        Item {
            value: 53,
            weight: 54,
        },
    ]
    .into();

    let problem = Problem::new(550, given_items);

    let initial_population: Population<Selection> = build_population()
        .with_genome_builder(BinaryEncodedGenomeBuilder::new(
            problem.given_items.list.len(),
        ))
        .of_size(400)
        .uniform_at_random();

    let mut knapsack_sim = simulate(
        genetic_algorithm()
            .with_evaluation(&problem)
            .with_selection(MaximizeSelector::new(0.85, 12))
            .with_crossover(SinglePointCrossBreeder::new())
            .with_mutation(RandomValueMutator::new(0.2, false, true))
            .with_reinsertion(ElitistReinserter::new(&problem, false, 0.85))
            .with_initial_population(initial_population)
            .build(),
    )
    .until(GenerationLimit::new(20))
    .build();

    'sim: loop {
        let result = knapsack_sim.step();

        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!(
                    "step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt(),
                );
                let knapsack = best_solution
                    .solution
                    .genome
                    .as_knapsack(&problem.given_items);
                println!(
                    "      Knapsack: number of items: {}, total value: {}, total weight: {}",
                    knapsack.items.len(),
                    knapsack.value,
                    knapsack.weight
                );
            }
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
                    processing_time.fmt(),
                );
                let knapsack = best_solution
                    .solution
                    .genome
                    .as_knapsack(&problem.given_items);
                println!(
                    "      Knapsack: number of items: {}, total value: {}, total weight: {}",
                    knapsack.items.len(),
                    knapsack.value,
                    knapsack.weight
                );
                break 'sim;
            }
            Err(error) => {
                println!("{}", error);
                break 'sim;
            }
        }
    }
}
