#[cfg(test)] #[macro_use] extern crate hamcrest;

use hamcrest::prelude::*;

extern crate genevo;
extern crate fixedbitset;
extern crate rand;

use genevo::prelude::*;
use genevo::operator::prelude::*;


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

    let algorithm = genetic_algorithm()
            .with_evaluation(MyFitnessEvaluator)
            .with_selection(RouletteWheelSelector::new(0.7, 2))
            .with_crossover(MultiPointCrossBreeder::new(3))
            .with_mutation(RandomValueMutator::new(0.015, -2.0, 2.0))
            .with_reinsertion(ElitistReinserter::new(MyFitnessEvaluator, false, 0.7))
            .with_termination(or(GenerationLimit::new(2000), FitnessLimit::new(10000)))
            .build();

    assert_that!(*algorithm.termination().condition1().max_generations(), is(equal_to(2000)));
    assert_that!(*algorithm.termination().condition2().fitness_target(), is(equal_to(10000)));

//    let &mut rng = StdRng::with_seed(&[42]);
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
