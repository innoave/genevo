#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion};
use genevo::population::{build_population, BinaryEncodedGenomeBuilder};

fn bench_build_population(c: &mut Criterion) {
    let seed = [42; 32];

    let mut group = c.benchmark_group("build population");
    for population_size in [20, 200, 2000, 20_000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(population_size),
            &population_size,
            |b, size| {
                b.iter(|| {
                    build_population()
                        .with_genome_builder::<_, Vec<bool>>(BinaryEncodedGenomeBuilder::new(24))
                        .of_size(*size)
                        .using_seed(seed)
                })
            },
        );
    }
}

criterion_group!(benches, bench_build_population);
criterion_main!(benches);
