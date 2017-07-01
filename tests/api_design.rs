#[cfg(test)] #[macro_use] extern crate hamcrest;
#[cfg(test)] #[macro_use] extern crate quickcheck;

extern crate genevo;
extern crate rand;

use genevo::prelude::*;
use genevo::algorithm::*;
use genevo::population::*;
use rand::StdRng;


#[test]
fn create_initial_population_uniform_at_random() {
    let initial_population = random_population()
        .with_genome_length(20)
        .of_size(200)
        .uniform_at_random();
}

#[test]
fn create_new_genetic_algorithm_application() {
    let algorithm = genetic_algorithm()
        .with_evaluation(my_fitness_function)
        .with_selection(roulette_wheel_selector)
        .with_crossover(multi_point_cross_breeder)
        .with_mutation(random_value_mutator)
        .with_reinsertion(elitist_resinserter)
        .with_termination(or(generations(2000), minimum_fitness(10000)))
        .build();

    let &mut rng = StdRng::with_seed([42]);
    iterate(algorithm).with_rng(&mut rng);

    loop {
        result = simulation.step();

        match result {
            ResultFlag::Intermediate(result) => {
                //print progress
            },
            ResultFlag::Final(result) => {
                //print final results
                break;
            },
        }
    }
}
