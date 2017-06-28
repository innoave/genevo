//! The `math` module provides functions that are useful for implementing
//! genetic algorithms.

use genetic::{AsScalar};


/// The `WeightedDistribution` is used to select values proportional to their
/// weighted values.
pub struct WeightedDistribution<'a, T> where T: 'a + AsScalar {
    values: &'a [T],
    sum: f64,
    weights: Vec<f64>,
}

impl<'a, T> WeightedDistribution<'a, T> where T: 'a + AsScalar {

    pub fn from_scalar_values(values: &'a [T]) -> Self {
        let (weights, weight_sum) = calc_weights_and_sum(values);
        WeightedDistribution {
            values: values,
            weights: weights,
            sum: weight_sum,
        }
    }

    pub fn sum(&self) -> &f64 {
        &self.sum
    }

    pub fn select(&self, pointer: f64) -> usize {
        weighted_select(pointer, &self.weights)
    }

    pub fn value(&self, index: usize) -> &T {
        &self.values[index]
    }

}

/// Calculates weights and the sum for the given values.
fn calc_weights_and_sum<'a, T>(values: &'a [T]) -> (Vec<f64>, f64)
    where T: 'a + AsScalar {
    let mut weights = Vec::with_capacity(values.len());
    let mut weight_sum: f64 = 0.;
    for i in 0..values.len() {
        weights.push(values[i].as_scalar());
        weight_sum += values[i].as_scalar();
    }
    (weights, weight_sum)
}

/// Selects one index proportional to their weights.
fn weighted_select(pointer: f64, weights: &[f64]) -> usize {
    let mut delta = pointer;
    for i in 0..weights.len() {
        delta -= weights[i];
        if delta <= 0. {
            return i;
        }
    }
    // when rounding errors occur, we return the last item's index
    return weights.len() - 1;
}


#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
    use rand::{Rng, SeedableRng, StdRng};

    #[test]
    fn weighted_distribution_select() {
        let mut rng = StdRng::from_seed(&[42usize]);

        let weights = vec![200, 150, 600, 50];
        let n_sum = 1_000.;

        let weighted_distribution = WeightedDistribution::from_scalar_values(&weights);

        let mut counter = vec![0, 0, 0, 0];
        for _ in 0..n_sum as usize {
            let random = rng.next_f64() * weighted_distribution.sum();
            let index = weighted_distribution.select(random);
            counter[index] += 1;
        }

        assert_that!(counter[0], is(equal_to(204)));
        assert_that!(counter[1], is(equal_to(152)));
        assert_that!(counter[2], is(equal_to(600)));
        assert_that!(counter[3], is(equal_to(44)));
    }
}
