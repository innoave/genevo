
#[cfg(test)] mod tests;

use genetic::AsScalar;

pub use rand::{Rng, SeedableRng, StdRng, thread_rng};
pub use rand::distributions::range::SampleRange;

pub type RngProvider<R: Rng + Sized> = Fn() -> R + Send + Sync;

pub fn random_index<R>(rng: &mut R, length: usize) -> usize
    where R: Rng + Sized {
    random_index_from_range(rng, 0, length)
}

pub fn random_index_from_range<R>(rng: &mut R, min: usize, max: usize) -> usize
    where R: Rng + Sized {
    rng.gen_range(min, max)
}

pub fn random_cut_points<R>(rng: &mut R, length: usize) -> (usize, usize)
    where R: Rng + Sized {
    random_cut_points_from_range(rng, 0, length)
}

pub fn random_cut_points_from_range<R>(rng: &mut R, min: usize, max: usize) -> (usize, usize)
    where R: Rng + Sized {
    assert!(max >= min + 4);
    let max_slice = max - min - 2;
    loop {
        let cutpoint1 = rng.gen_range(min, max);
        let cutpoint2 = rng.gen_range(min, max);
        if cutpoint1 < cutpoint2 {
            if cutpoint2 - cutpoint1 >= max_slice {
                continue;
            }
            return (cutpoint1, cutpoint2)
        } else if cutpoint2 < cutpoint1 {
            if cutpoint1 - cutpoint2 >= max_slice {
                continue;
            }
            return (cutpoint2, cutpoint1)
        }
    }
}

pub fn random_n_cut_points<R>(rng: &mut R, n: usize, length: usize) -> Vec<usize>
    where R: Rng + Sized {
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

pub fn random_probability<R>(rng: &mut R) -> f64
    where R: Rng + Sized {
    rng.next_f64()
}

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
    // when rounding errors occur, we return the last item's index
    weights.len() - 1
}
