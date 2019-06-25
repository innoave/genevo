//! The `encoding` module provides basic scheme of encoding
//! `genetic::Genotype`s.
//!
//! Most important encoding schemes are:
//! * binary encoding
//! * value encoding
//! * permutation encoding
//! * tree encoding
//!
//! To express which encoding scheme is used for a specific `genetic::Genotype`
//! a set of marker traits are defined:
//! * `BinaryEncoded`
//! * `ValueEncoded`
//! * `PermutationEncoded`
//! * `TreeEncoded`
//!
//! These marker traits are important for providing default implementations
//! for the `operator::CrossoverOp` and the `operator::MutationOp`. In order
//! to use any of the default operator implementation the `genetic::Genotype`
//! used for a genetic algorithm application must be marked with the
//! appropriate encoding trait. If an application is defining its own crossover
//! and mutation operators then using these marker traits is optional.

use crate::genetic::Genotype;
use std::fmt::Debug;

/// Marker trait for declaring a `genetic::Genotype` as binary encoded.
pub trait BinaryEncoded {}

/// Marker trait for declaring a `genetic::Genotype` as value encoded.
pub trait ValueEncoded {}

/// Marker trait for declaring a permutation encoded `genetic::Genotype`.
pub trait PermutationEncoded {}

/// Marker trait for declaring a tree encoded `genetic::Genotype`.
pub trait TreeEncoded: Genotype {}

/// Implementation of a genotype using `Vec`.
impl<V> Genotype for Vec<V>
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    type Dna = V;
}

/// Implementation of binary encoded `genetic::Genotype`
/// using `Vec<bool>`.
impl BinaryEncoded for Vec<bool> {}

/// Implementation of a value encoded `genetic::Genotype`.
/// using `Vec`.
impl<V> ValueEncoded for Vec<V> {}

/// Implementation of a permutation encoded `genetic::Genotype`
/// using `Vec`.
impl<V> PermutationEncoded for Vec<V> {}

#[cfg(feature = "fixedbitset")]
mod fixedbitset_genotype {
    use super::{BinaryEncoded, Genotype};

    use fixedbitset::FixedBitSet;

    /// Implementation of genotype using `fixedbistset::FixedBitSet`.
    impl Genotype for FixedBitSet {
        type Dna = bool;
    }

    /// Implementation of binary encoded `genetic::Genotype`
    /// using `fixedbistset::FixedBitSet`.
    impl BinaryEncoded for FixedBitSet {}
}

#[cfg(feature = "smallvec")]
mod smallvec_genotype {
    use super::{BinaryEncoded, Genotype, PermutationEncoded, ValueEncoded};
    use smallvec::{Array, SmallVec};
    use std::fmt::Debug;

    /// Implementation of binary encoded `genetic::Genotype`
    /// using `smallvec::SmallVec`.
    impl<A, V> Genotype for SmallVec<A>
    where
        A: Array<Item = V> + Sync,
        V: Clone + Debug + PartialEq + Send + Sync,
    {
        type Dna = V;
    }

    /// Implementation of binary encoded `genetic::Genotype`
    /// using `smallvec::SmallVec<Item = bool>`.
    impl<A> BinaryEncoded for SmallVec<A> where A: Array<Item = bool> {}

    /// Implementation of a value encoded `genetic::Genotype`.
    /// using `smallvec::SmallVec`.
    impl<A> ValueEncoded for SmallVec<A> where A: Array {}

    /// Implementation of a permutation encoded `genetic::Genotype`
    /// using `smallvec::SmallVec`.
    impl<A> PermutationEncoded for SmallVec<A> where A: Array {}
}
