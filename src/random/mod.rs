//! The `random` module defines functions that are used to generate random
//! values for specific purposes.

#[cfg(test)]
mod tests;

pub use rand::{
    distributions::{uniform::SampleUniform, Open01},
    seq::SliceRandom,
    Rng, SeedableRng,
};

use crate::genetic::AsScalar;
use rand::thread_rng;
use rand_xoshiro::Xoshiro256Plus;

/// The `Prng` is the pseudo random number generator used through out this
/// library.
pub type Prng = Xoshiro256Plus;

/// The `Seed` as used through out this library to seed the `Prng`.
pub type Seed = [u8; 32];

/// Generates a random seed to initialize the `Prng`.
pub fn random_seed() -> Seed {
    let mut rng = thread_rng();
    rng.gen()
}

/// Returns a new `Prng` initialized with the given seed.
pub fn get_rng(seed: Seed) -> Prng {
    Prng::from_seed(seed)
}

/// Generates a random index into a slice of given length using the given
/// `Prng`.
pub fn random_index<R>(rng: &mut R, length: usize) -> usize
where
    R: Rng + Sized,
{
    random_index_from_range(rng, 0, length)
}

/// Generates a random index in the given range using the given `Prng`.
pub fn random_index_from_range<R>(rng: &mut R, min: usize, max: usize) -> usize
where
    R: Rng + Sized,
{
    rng.gen_range(min, max)
}

/// Generates two cut points for a slice of given length using the given `Prng`.
/// The first of the two returned cut points is always smaller than the second
/// one.
pub fn random_cut_points<R>(rng: &mut R, length: usize) -> (usize, usize)
where
    R: Rng + Sized,
{
    random_cut_points_from_range(rng, 0, length)
}

/// Generates two cut points within the given range using the given `Prng`. The
/// first of the two returned cut points is always smaller than the second one.
pub fn random_cut_points_from_range<R>(rng: &mut R, min: usize, max: usize) -> (usize, usize)
where
    R: Rng + Sized,
{
    assert!(max >= min + 4);
    let max_slice = max - min - 2;
    loop {
        let cutpoint1 = rng.gen_range(min, max);
        let cutpoint2 = rng.gen_range(min, max);
        if cutpoint1 < cutpoint2 {
            if cutpoint2 - cutpoint1 >= max_slice {
                continue;
            }
            return (cutpoint1, cutpoint2);
        } else if cutpoint2 < cutpoint1 {
            if cutpoint1 - cutpoint2 >= max_slice {
                continue;
            }
            return (cutpoint2, cutpoint1);
        }
    }
}

/// Generates `n` cut points for a slice of given length using the given `Prng`.
/// The returned cut points are ordered in ascending order.
pub fn random_n_cut_points<R>(rng: &mut R, n: usize, length: usize) -> Vec<usize>
where
    R: Rng + Sized,
{
    assert!(n > 0);
    assert!(length >= 2 * n);
    let mut cutpoints = Vec::with_capacity(n);
    match n {
        1 => {
            cutpoints.push(random_index(rng, length));
        },
        2 => {
            let (cp1, cp2) = random_cut_points(rng, length);
            cutpoints.push(cp1);
            cutpoints.push(cp2);
        },
        _ => {
            let slice_len = length / n;
            let mut start = 0;
            let mut end = slice_len;
            let mut count = 1;
            loop {
                let cutpoint = random_index_from_range(rng, start, end);
                if cutpoint == 0 || cutpoint == length {
                    continue;
                }
                cutpoints.push(cutpoint);
                count += 1;
                if count > n {
                    break;
                }
                start = cutpoint + 1;
                if count == n {
                    end = length;
                } else {
                    end += slice_len;
                }
            }
        },
    }
    cutpoints
}

/// Generates a random probability between 0 and 1 using the given `Prng`.
///
/// The generated probabilities are in the open range (0,1), excluding 0 and
/// excluding 1.
pub fn random_probability<R>(rng: &mut R) -> f64
where
    R: Rng + Sized,
{
    rng.sample(Open01)
}

/// The `WeightedDistribution` is used to select values proportional to their
/// weighted values.
///
/// The values in a `WeightedDistribution` must have a scalar representation.
/// Thus their types must implement the `genetic::AsScalar` trait. The weights
/// of the values are calculated from their scalar representation.
#[derive(Clone, Debug, PartialEq)]
pub struct WeightedDistribution<'a, T>
where
    T: 'a + AsScalar,
{
    values: &'a [T],
    sum: f64,
    weights: Vec<f64>,
}

impl<'a, T> WeightedDistribution<'a, T>
where
    T: 'a + AsScalar,
{
    /// Constructs a new instance of `WeightedDistribution` for the given slice
    /// of values.
    pub fn from_scalar_values(values: &'a [T]) -> Self {
        let (weights, weight_sum) = calc_weights_and_sum(values);
        WeightedDistribution {
            values,
            weights,
            sum: weight_sum,
        }
    }

    /// Selects a value proportional to its weight and returns its index.
    ///
    /// The pointer must be a float between 0 und the sum of the weights of all
    /// values. Usually the pointer is chosen uniformly at random.
    pub fn select(&self, pointer: f64) -> usize {
        assert!(pointer >= 0. && pointer <= self.sum);
        weighted_select(pointer, &self.weights)
    }

    /// Returns the sum of the weights of all values in this
    /// `WeightedDistribution` instance.
    ///
    /// The sum is calculated from the scalar values of the slice that was used
    /// to create this `WeightedDistribution` instance.
    pub fn sum(&self) -> f64 {
        self.sum
    }

    /// Returns a reference to the value at the given index.
    pub fn value(&self, index: usize) -> &T {
        &self.values[index]
    }
}

/// Calculates weights and the sum for the given values.
fn calc_weights_and_sum<'a, T>(values: &'a [T]) -> (Vec<f64>, f64)
where
    T: 'a + AsScalar,
{
    let mut weights = Vec::with_capacity(values.len());
    let mut weight_sum: f64 = 0.;
    for value in values.iter() {
        let scalar = value.as_scalar();
        weight_sum += scalar;
        weights.push(scalar);
    }
    (weights, weight_sum)
}

/// Selects one index proportional to their weights.
fn weighted_select(pointer: f64, weights: &[f64]) -> usize {
    let mut delta = pointer;
    for (i, weight) in weights.iter().enumerate() {
        delta -= *weight;
        if delta <= 0. {
            return i;
        }
    }
    // when rounding errors occur, return the last item's index
    weights.len() - 1
}
