#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion};
use genevo::random::{get_rng, random_seed};
use rand::{
    distributions::{Bernoulli, Distribution},
    Rng,
};

fn generate_vec_of_random_bool_the_functional_way(c: &mut Criterion) {
    let mut rng = get_rng(random_seed());

    let mut group = c.benchmark_group("generate vec of bool the functional way");
    for length in [12, 24, 48, 96] {
        group.bench_with_input(BenchmarkId::from_parameter(length), &length, |b, length| {
            b.iter(|| {
                let _genome: Vec<bool> = (0..*length).map(|_| rng.gen()).collect();
            })
        });
    }
}

fn generate_vec_of_random_bool_using_for_loop(c: &mut Criterion) {
    let mut rng = get_rng(random_seed());

    let mut group = c.benchmark_group("generate vec of bool using for loop");
    for length in [12, 24, 48, 96] {
        group.bench_with_input(BenchmarkId::from_parameter(length), &length, |b, length| {
            b.iter(|| {
                let length = *length;
                let mut genome: Vec<bool> = Vec::with_capacity(length);
                for _ in 0..length {
                    genome.push(rng.gen());
                }
            })
        });
    }
}

fn generate_vec_of_random_bool_the_functional_way_with_initialized_distribution(c: &mut Criterion) {
    let bernoulli = Bernoulli::new(0.5).expect("0.5 is a valid probability");
    let mut rng = get_rng(random_seed());

    let mut group = c.benchmark_group("generate vec of bool functional init distribution");
    for length in [12, 24, 48, 96] {
        group.bench_with_input(BenchmarkId::from_parameter(length), &length, |b, length| {
            b.iter(|| {
                let _genome: Vec<bool> = (0..*length).map(|_| bernoulli.sample(&mut rng)).collect();
            })
        });
    }
}

fn generate_vec_of_random_bool_using_for_loop_with_initialized_distribution(c: &mut Criterion) {
    let bernoulli = Bernoulli::new(0.5).expect("0.5 is a valid probability");
    let mut rng = get_rng(random_seed());

    let mut group = c.benchmark_group("generate vec of bool loop init distribution");
    for length in [12, 24, 48, 96] {
        group.bench_with_input(BenchmarkId::from_parameter(length), &length, |b, length| {
            b.iter(|| {
                let length = *length;
                let mut genome: Vec<bool> = Vec::with_capacity(length);
                for _ in 0..length {
                    genome.push(bernoulli.sample(&mut rng));
                }
            })
        });
    }
}

criterion_group!(
    benches,
    generate_vec_of_random_bool_the_functional_way,
    generate_vec_of_random_bool_using_for_loop,
    generate_vec_of_random_bool_the_functional_way_with_initialized_distribution,
    generate_vec_of_random_bool_using_for_loop_with_initialized_distribution,
);
criterion_main!(benches);
