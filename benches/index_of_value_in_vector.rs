#![feature(test)]

extern crate test;

use test::Bencher;


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
        data: (0..member).map(|i| Gene { order: (i..i + size).collect() }).collect()
    }
}

#[bench]
fn index_of_gene_in_vector_the_functional_way_200_member(b: &mut Bencher) {
    let fixture = fixture(200, 11);
    b.iter(|| {
        for i in 0..fixture.data.len() {
            let gene = &fixture.data[i];
            fixture.index_of_gene_functional(gene);
        }
    })
}

#[bench]
fn index_of_gene_in_vector_for_loop_over_index_200_member(b: &mut Bencher) {
    let fixture = fixture(200, 11);
    b.iter(|| {
        for i in 0..fixture.data.len() {
            let gene = &fixture.data[i];
            fixture.index_of_gene_for_loop_over_index(gene);
        }
    })
}

#[bench]
fn index_of_gene_in_vector_the_functional_way_20_member(b: &mut Bencher) {
    let fixture = fixture(20, 200);
    b.iter(|| {
        for i in 0..fixture.data.len() {
            let gene = &fixture.data[i];
            fixture.index_of_gene_functional(gene);
        }
    })
}

#[bench]
fn index_of_gene_in_vector_for_loop_over_index_20_member(b: &mut Bencher) {
    let fixture = fixture(20, 200);
    b.iter(|| {
        for i in 0..fixture.data.len() {
            let gene = &fixture.data[i];
            fixture.index_of_gene_for_loop_over_index(gene);
        }
    })
}

#[bench]
fn index_of_gene_in_vector_the_functional_way_2000_member(b: &mut Bencher) {
    let fixture = fixture(2000, 11);
    b.iter(|| {
        for i in 0..fixture.data.len() {
            let gene = &fixture.data[i];
            fixture.index_of_gene_functional(gene);
        }
    })
}

#[bench]
fn index_of_gene_in_vector_for_loop_over_index_2000_member(b: &mut Bencher) {
    let fixture = fixture(2000, 11);
    b.iter(|| {
        for i in 0..fixture.data.len() {
            let gene = &fixture.data[i];
            fixture.index_of_gene_for_loop_over_index(gene);
        }
    })
}
