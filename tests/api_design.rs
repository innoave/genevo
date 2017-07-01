#[cfg(test)] #[macro_use] extern crate hamcrest;
#[cfg(test)] #[macro_use] extern crate quickcheck;

use hamcrest::prelude::*;

extern crate genevo;
extern crate fixedbitset;
extern crate rand;

use genevo::algorithm::*;
use genevo::population::*;
use fixedbitset::FixedBitSet;
use rand::{SeedableRng, StdRng};
use std::marker::PhantomData;


#[test]
fn create_initial_population_of_fixedbitset_uniform_at_random() {
    let initial_population: Population<FixedBitSet> =
        random_population()
        .with_genome_length(20)
        .of_size(200)
        .uniform_at_random();

    assert_that!(initial_population.size(), is(equal_to(200)));
}

#[test]
fn create_initial_population_of_vec_of_f64_uniform_at_random() {
    let initial_population: Population<Vec<f64>> =
        random_population()
            .with_genome_length(20)
            .of_size(200)
            .uniform_at_random();

    assert_that!(initial_population.size(), is(equal_to(200)));
}
#[test]
fn create_new_genetic_algorithm_application() {
//    let algorithm = genetic_algorithm()
//        .with_evaluation(my_fitness_function)
//        .with_selection(roulette_wheel_selector)
//        .with_crossover(multi_point_cross_breeder)
//        .with_mutation(random_value_mutator)
//        .with_reinsertion(elitist_resinserter)
//        .with_termination(or(generations(2000), minimum_fitness(10000)))
//        .build();
//
//    let &mut rng = StdRng::with_seed([42]);
//    iterate(algorithm).with_rng(&mut rng);
//
//    loop {
//        let result = simulation.step();
//
//        match result {
//            ResultFlag::Intermediate(result) => {
//                //print progress
//            },
//            ResultFlag::Final(result) => {
//                //print final results
//                break;
//            },
//        }
//    }
}
