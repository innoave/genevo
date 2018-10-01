//! The `order` module provides implementations of `operator::CrossoverOp` for
//! permutation encoded `genetic::Genotype`s. Crossover of permutation encoded
//! values must assure that the permutation does not invalidate the order
//! values.
//!
//! The provided `operator::CrossoverOp`s for permutation encoded values are:
//! * `OrderOneCrossover` (OX1)
//! * `PartiallyMappedCrossover` (PMX)

use genetic::{Children, Parents};
use operator::{CrossoverOp, GeneticOperator};
use random::{random_cut_points, Rng};
use std::collections::HashMap;

/// The `OrderOneCrossover` operator combines permutation encoded
/// `genetic::Genotype`s according the order one crossover scheme (OX1).
///
/// This crossover operator always creates as many child individuals as there
/// are individuals in the given `genetic::Parents` parameter.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct OrderOneCrossover {}

impl OrderOneCrossover {
    pub fn new() -> Self {
        OrderOneCrossover {}
    }
}

impl GeneticOperator for OrderOneCrossover {
    fn name() -> String {
        "Order-One-Crossover".to_string()
    }
}

impl CrossoverOp<Vec<usize>> for OrderOneCrossover {
    fn crossover<R>(&self, parents: Parents<Vec<usize>>, rng: &mut R) -> Children<Vec<usize>>
    where
        R: Rng + Sized,
    {
        multi_parents_cyclic_crossover(&parents, order_one_crossover, rng)
    }
}

/// The `PartiallyMappedCrossover` operator combines permutation encoded
/// `genetic::Genotype`s according the partially mapped crossover scheme (PMX).
///
/// This crossover operator always creates as many child individuals as there
/// are individuals in the given `genetic::Parents` parameter.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct PartiallyMappedCrossover {}

impl PartiallyMappedCrossover {
    pub fn new() -> Self {
        PartiallyMappedCrossover {}
    }
}

impl GeneticOperator for PartiallyMappedCrossover {
    fn name() -> String {
        "Partially-Mapped-Crossover".to_string()
    }
}

impl CrossoverOp<Vec<usize>> for PartiallyMappedCrossover {
    fn crossover<R>(&self, parents: Parents<Vec<usize>>, rng: &mut R) -> Children<Vec<usize>>
    where
        R: Rng + Sized,
    {
        multi_parents_cyclic_crossover(&parents, partial_mapped_crossover, rng)
    }
}

fn multi_parents_cyclic_crossover<'a, FN, R>(
    parents: &'a Parents<Vec<usize>>,
    crossover: FN,
    rng: &mut R,
) -> Children<Vec<usize>>
where
    FN: Fn(&'a [usize], &'a [usize], usize, usize) -> Vec<usize>,
    R: Rng + Sized,
{
    let parents_size = parents.len();
    let genome_length = parents[0].len();
    // breed one child for each partner in parents
    let mut offspring: Vec<Vec<usize>> = Vec::with_capacity(parents_size);
    let mut p1_index = 0;
    let mut p2_index = 1;
    while p1_index < parents_size {
        let (cutpoint1, cutpoint2) = random_cut_points(rng, genome_length);
        let genome = crossover(&parents[p1_index], &parents[p2_index], cutpoint1, cutpoint2);
        offspring.push(genome);
        p1_index += 1;
        p2_index += 1;
        if p2_index >= genome_length {
            p2_index = 0;
        }
    }
    offspring
}

fn order_one_crossover(
    parent1: &[usize],
    parent2: &[usize],
    cutpoint1: usize,
    cutpoint2: usize,
) -> Vec<usize> {
    let genome_length = parent1.len();
    let mut genome: Vec<usize> = Vec::with_capacity(genome_length);
    // collect genes of parent1 located at cutpoint1 to cutpoint2
    let mut p1_slice: Vec<&usize> = if cutpoint1 == 0 {
        parent1.iter().take(cutpoint2 + 1).collect()
    } else {
        parent1
            .iter()
            .skip(cutpoint1)
            .take(cutpoint2 - cutpoint1 + 1)
            .collect()
    };
    // collect genes from parent2 which are not in cut slice
    let mut p2_slice: Vec<&usize> = Vec::with_capacity(genome_length);
    let mut p2_index = (cutpoint2 + 1) % genome_length;
    for _ in 0..genome_length {
        let p2_genome = &parent2[p2_index];
        if p1_slice.iter().all(|g| p2_genome != *g) {
            p2_slice.push(p2_genome);
        }
        p2_index += 1;
        if p2_index >= genome_length {
            p2_index = 0;
        }
    }
    //    println!("{}-{} : {:?} <-> {:?}", cutpoint1, cutpoint2, p1_slice, p2_slice);
    // insert genes into child genome at correct position
    let right_offset = genome_length - cutpoint2 - 1;
    for locus in 0..genome_length {
        if locus < cutpoint1 {
            genome.push(*p2_slice.remove(right_offset));
        } else if locus > cutpoint2 {
            genome.push(*p2_slice.remove(0));
        } else {
            genome.push(*p1_slice.remove(0));
        }
    }
    genome
}

fn partial_mapped_crossover(
    parent1: &[usize],
    parent2: &[usize],
    cutpoint1: usize,
    cutpoint2: usize,
) -> Vec<usize> {
    let genome_length = parent1.len();
    let mut genome: Vec<usize> = Vec::with_capacity(genome_length);
    // using HashMap as indexed array of variable length
    let mut result: HashMap<usize, usize> = HashMap::with_capacity(genome_length);
    // mapping of value to index
    let mut inverse: HashMap<usize, usize> = HashMap::with_capacity(genome_length);
    for (i, v2) in parent2.iter().enumerate() {
        result.insert(i, *v2);
        inverse.insert(*v2, i);
    }
    for (j, v1) in parent1
        .iter()
        .enumerate()
        .take(cutpoint2 + 1)
        .skip(cutpoint1)
    {
        let orig = result[&j];
        result.insert(j, *v1);
        let k = inverse[v1];
        result.insert(k, orig);
        inverse.insert(orig, k);
    }
    for i in 0..genome_length {
        genome.push(result[&i])
    }
    genome
}

#[cfg(test)]
mod tests {
    use super::*;
    use galvanic_assert::matchers::*;

    #[test]
    fn order_one_crossover_cutpoints_3_6() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = order_one_crossover(&p1, &p2, 3, 6);
        expect_that!(&children, eq(vec![3, 8, 2, 4, 5, 6, 7, 1, 9]));

        let children = order_one_crossover(&p2, &p1, 3, 6);
        expect_that!(&children, eq(vec![3, 4, 7, 8, 2, 6, 5, 9, 1]));
    }

    #[test]
    fn order_one_crossover_cutpoints_0_0() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = order_one_crossover(&p1, &p2, 0, 0);
        expect_that!(&children, eq(vec![1, 3, 7, 8, 2, 6, 5, 4, 9]));

        let children = order_one_crossover(&p2, &p1, 0, 0);
        expect_that!(&children, eq(vec![9, 2, 3, 4, 5, 6, 7, 8, 1]));
    }

    #[test]
    fn order_one_crossover_cutpoints_0_8() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = order_one_crossover(&p1, &p2, 0, 8);
        expect_that!(&children, eq(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let children = order_one_crossover(&p2, &p1, 0, 8);
        expect_that!(&children, eq(vec![9, 3, 7, 8, 2, 6, 5, 1, 4]));
    }

    #[test]
    fn order_one_crossover_cutpoints_1_8() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = order_one_crossover(&p1, &p2, 1, 8);
        expect_that!(&children, eq(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let children = order_one_crossover(&p2, &p1, 1, 8);
        expect_that!(&children, eq(vec![9, 3, 7, 8, 2, 6, 5, 1, 4]));
    }

    #[test]
    fn order_one_crossover_cutpoints_0_7() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = order_one_crossover(&p1, &p2, 0, 7);
        expect_that!(&children, eq(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let children = order_one_crossover(&p2, &p1, 0, 7);
        expect_that!(&children, eq(vec![9, 3, 7, 8, 2, 6, 5, 1, 4]));
    }

    #[test]
    fn order_one_crossover_cutpoints_1_7() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![1, 3, 7, 8, 2, 6, 5, 9, 4];

        let children = order_one_crossover(&p1, &p2, 1, 7);
        expect_that!(&children, eq(vec![9, 2, 3, 4, 5, 6, 7, 8, 1]));

        let children = order_one_crossover(&p2, &p1, 1, 7);
        expect_that!(&children, eq(vec![4, 3, 7, 8, 2, 6, 5, 9, 1]));
    }

    #[test]
    fn partial_mapped_crossover_cutpoints_3_6() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = partial_mapped_crossover(&p1, &p2, 3, 6);
        expect_that!(&children, eq(vec![9, 3, 2, 4, 5, 6, 7, 1, 8]));

        let children = partial_mapped_crossover(&p2, &p1, 3, 6);
        expect_that!(&children, eq(vec![1, 7, 3, 8, 2, 6, 5, 4, 9]));
    }

    #[test]
    fn partial_mapped_crossover_cutpoints_0_0() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = partial_mapped_crossover(&p1, &p2, 0, 0);
        expect_that!(&children, eq(vec![1, 3, 7, 8, 2, 6, 5, 9, 4]));

        let children = partial_mapped_crossover(&p2, &p1, 0, 0);
        expect_that!(&children, eq(vec![9, 2, 3, 4, 5, 6, 7, 8, 1]));
    }

    #[test]
    fn partial_mapped_crossover_cutpoints_0_8() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = partial_mapped_crossover(&p1, &p2, 0, 8);
        expect_that!(&children, eq(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let children = partial_mapped_crossover(&p2, &p1, 0, 8);
        expect_that!(&children, eq(vec![9, 3, 7, 8, 2, 6, 5, 1, 4]));
    }

    #[test]
    fn partial_mapped_crossover_cutpoints_1_8() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = partial_mapped_crossover(&p1, &p2, 1, 8);
        expect_that!(&children, eq(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let children = partial_mapped_crossover(&p2, &p1, 1, 8);
        expect_that!(&children, eq(vec![9, 3, 7, 8, 2, 6, 5, 1, 4]));
    }

    #[test]
    fn partial_mapped_crossover_cutpoints_0_7() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![9, 3, 7, 8, 2, 6, 5, 1, 4];

        let children = partial_mapped_crossover(&p1, &p2, 0, 7);
        expect_that!(&children, eq(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let children = partial_mapped_crossover(&p2, &p1, 0, 7);
        expect_that!(&children, eq(vec![9, 3, 7, 8, 2, 6, 5, 1, 4]));
    }

    #[test]
    fn partial_mapped_crossover_cutpoints_1_7() {
        let p1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let p2 = vec![1, 3, 7, 8, 2, 6, 5, 9, 4];

        let children = partial_mapped_crossover(&p1, &p2, 1, 7);
        expect_that!(&children, eq(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

        let children = partial_mapped_crossover(&p2, &p1, 1, 7);
        expect_that!(&children, eq(vec![1, 3, 7, 8, 2, 6, 5, 9, 4]));
    }
}
