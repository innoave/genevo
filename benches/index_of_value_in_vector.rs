#[macro_use]
extern crate criterion;

use criterion::Criterion;

#[derive(Debug, PartialEq)]
struct Gene {
    order: Vec<usize>,
}

struct Fixture {
    data: Vec<Gene>,
}

impl Fixture {
    fn index_of_gene_functional(&self, gene: &Gene) -> Option<usize> {
        self.data.iter().position(|g| *g == *gene)
    }

    fn index_of_gene_for_loop_over_index(&self, gene: &Gene) -> Option<usize> {
        let mut index_of_gene = None;
        for i in 0..self.data.len() {
            if *gene == self.data[i] {
                index_of_gene = Some(i);
                break;
            }
        }
        index_of_gene
    }
}

fn fixture(member: usize, size: usize) -> Fixture {
    Fixture {
        data: (0..member)
            .map(|i| Gene {
                order: (i..i + size).collect(),
            })
            .collect(),
    }
}

fn index_of_gene_in_vector_the_functional_way_200_member(c: &mut Criterion) {
    let fixture = fixture(200, 11);
    c.bench_function(
        "index of gene in vector of 200 members the functional way",
        move |b| {
            b.iter(|| {
                for i in 0..fixture.data.len() {
                    let gene = &fixture.data[i];
                    fixture.index_of_gene_functional(gene);
                }
            })
        },
    );
}

fn index_of_gene_in_vector_for_loop_over_index_200_member(c: &mut Criterion) {
    let fixture = fixture(200, 11);
    c.bench_function(
        "index of gene in vector of 200 members using for loop",
        move |b| {
            b.iter(|| {
                for i in 0..fixture.data.len() {
                    let gene = &fixture.data[i];
                    fixture.index_of_gene_for_loop_over_index(gene);
                }
            })
        },
    );
}

fn index_of_gene_in_vector_the_functional_way_20_member(c: &mut Criterion) {
    let fixture = fixture(20, 200);
    c.bench_function(
        "index of gene in vector of 20 members the functional way",
        move |b| {
            b.iter(|| {
                for i in 0..fixture.data.len() {
                    let gene = &fixture.data[i];
                    fixture.index_of_gene_functional(gene);
                }
            })
        },
    );
}

fn index_of_gene_in_vector_for_loop_over_index_20_member(c: &mut Criterion) {
    let fixture = fixture(20, 200);
    c.bench_function(
        "index of gene in vector of 20 members using for loop",
        move |b| {
            b.iter(|| {
                for i in 0..fixture.data.len() {
                    let gene = &fixture.data[i];
                    fixture.index_of_gene_for_loop_over_index(gene);
                }
            })
        },
    );
}

fn index_of_gene_in_vector_the_functional_way_2000_member(c: &mut Criterion) {
    let fixture = fixture(2000, 11);
    c.bench_function(
        "index of gene in vector of 2000 members the functional way",
        move |b| {
            b.iter(|| {
                for i in 0..fixture.data.len() {
                    let gene = &fixture.data[i];
                    fixture.index_of_gene_functional(gene);
                }
            })
        },
    );
}

fn index_of_gene_in_vector_for_loop_over_index_2000_member(c: &mut Criterion) {
    let fixture = fixture(2000, 11);
    c.bench_function(
        "index of gene in vector of 2000 members using for loop",
        move |b| {
            b.iter(|| {
                for i in 0..fixture.data.len() {
                    let gene = &fixture.data[i];
                    fixture.index_of_gene_for_loop_over_index(gene);
                }
            })
        },
    );
}

criterion_group!(
    benches,
    index_of_gene_in_vector_the_functional_way_200_member,
    index_of_gene_in_vector_for_loop_over_index_200_member,
    index_of_gene_in_vector_the_functional_way_20_member,
    index_of_gene_in_vector_for_loop_over_index_20_member,
    index_of_gene_in_vector_the_functional_way_2000_member,
    index_of_gene_in_vector_for_loop_over_index_2000_member,
);
criterion_main!(benches);
