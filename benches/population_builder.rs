#[macro_use]
extern crate criterion;

use criterion::Criterion;
use genevo::population::{build_population, BinaryEncodedGenomeBuilder};

fn bench_build_population(c: &mut Criterion) {
    let seed = [42; 32];

    c.bench_function_over_inputs(
        "build population",
        move |b, size| {
            b.iter(|| {
                build_population()
                    .with_genome_builder::<_, Vec<bool>>(BinaryEncodedGenomeBuilder::new(24))
                    .of_size(*size)
                    .using_seed(seed)
            })
        },
        vec![20, 200, 2000, 20_000],
    );
}

criterion_group!(benches, bench_build_population);
criterion_main!(benches);
