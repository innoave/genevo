#[macro_use]
extern crate criterion;

use criterion::Criterion;
use genevo::random::{get_rng, random_seed};
use rand::{
    distributions::{Bernoulli, Distribution},
    Rng,
};

fn generate_vec_of_random_bool_the_functional_way(c: &mut Criterion) {
    let mut rng = get_rng(random_seed());

    c.bench_function_over_inputs(
        "generate vec of bool the functional way",
        move |b, length| {
            b.iter(|| {
                let _genome: Vec<bool> = (0..*length).map(|_| rng.gen()).collect();
            })
        },
        vec![12, 24, 48, 96],
    );
}

fn generate_vec_of_random_bool_using_for_loop(c: &mut Criterion) {
    let mut rng = get_rng(random_seed());

    c.bench_function_over_inputs(
        "generate vec of bool using for loop",
        move |b, length| {
            b.iter(|| {
                let length = *length;
                let mut genome: Vec<bool> = Vec::with_capacity(length);
                for _ in 0..length {
                    genome.push(rng.gen());
                }
            })
        },
        vec![12, 24, 48, 96],
    );
}

fn generate_vec_of_random_bool_the_functional_way_with_initialized_distribution(c: &mut Criterion) {
    let bernoulli = Bernoulli::new(0.5).expect("0.5 is a valid probability");
    let mut rng = get_rng(random_seed());

    c.bench_function_over_inputs(
        "generate vec of bool functional init distribution",
        move |b, length| {
            b.iter(|| {
                let _genome: Vec<bool> = (0..*length).map(|_| bernoulli.sample(&mut rng)).collect();
            })
        },
        vec![12, 24, 48, 96],
    );
}

fn generate_vec_of_random_bool_using_for_loop_with_initialized_distribution(c: &mut Criterion) {
    let bernoulli = Bernoulli::new(0.5).expect("0.5 is a valid probability");
    let mut rng = get_rng(random_seed());

    c.bench_function_over_inputs(
        "generate vec of bool loop init distribution",
        move |b, length| {
            b.iter(|| {
                let length = *length;
                let mut genome: Vec<bool> = Vec::with_capacity(length);
                for _ in 0..length {
                    genome.push(bernoulli.sample(&mut rng));
                }
            })
        },
        vec![12, 24, 48, 96],
    );
}

criterion_group!(
    benches,
    generate_vec_of_random_bool_the_functional_way,
    generate_vec_of_random_bool_using_for_loop,
    generate_vec_of_random_bool_the_functional_way_with_initialized_distribution,
    generate_vec_of_random_bool_using_for_loop_with_initialized_distribution,
);
criterion_main!(benches);
