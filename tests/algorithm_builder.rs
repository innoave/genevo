#[cfg(test)] #[macro_use] extern crate hamcrest;

use hamcrest::prelude::*;

extern crate genevo;
extern crate fixedbitset;
extern crate rand;

use genevo::prelude::*;
use genevo::operator::prelude::*;
use genevo::population::ValueEncodedGenomeBuilder;


#[test]
fn create_new_genetic_algorithm_application() {
    type MyGenome = Vec<f64>;

    #[derive(Clone,Debug,PartialEq)]
    struct MyFitnessEvaluator;

    impl FitnessFunction<MyGenome, u32> for MyFitnessEvaluator {
        fn fitness_of(&self, individual: &MyGenome) -> u32 {
            (individual.iter().sum::<f64>() * 10000. + 0.5).floor() as u32
        }

        fn average(&self, fitness_values: &[u32]) -> u32 {
            (fitness_values.iter().sum::<u32>() as f64 / fitness_values.len() as f64 + 0.5).floor() as u32
        }

        fn highest_possible_fitness(&self) -> u32 {
            10000
        }

        fn lowest_possible_fitness(&self) -> u32 {
            0
        }
    }

    let initial_population: Population<Vec<f64>> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(4, -2., 2.))
        .of_size(200)
        .uniform_at_random();

    let algorithm = genetic_algorithm()
            .with_evaluation(MyFitnessEvaluator)
            .with_selection(RouletteWheelSelector::new(0.7, 2))
            .with_crossover(MultiPointCrossBreeder::new(3))
            .with_mutation(RandomValueMutator::new(0.015, -2.0, 2.0))
            .with_reinsertion(ElitistReinserter::new(MyFitnessEvaluator, false, 0.7))
            .with_initial_population(initial_population)
            .build();

    assert_that!(algorithm.selector().selection_ratio(), is(equal_to(0.7)));
    assert_that!(algorithm.selector().num_individuals_per_parents(), is(equal_to(2)));
    assert_that!(algorithm.breeder().num_cut_points(), is(equal_to(3)));
}
