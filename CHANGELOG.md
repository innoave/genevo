# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## 0.3.0 : 2019-06-25

* add support for `SmallVec` as optional crate feature
* make support for `FixedBitSet` an optional crate feature
* replace `DiscreteCrossBreeder` by integrating it into `UniformCrossBreeder`
* make support for `Vec<bool>` consistent through all building blocks
* implement std `Error` trait for `SimError` and `GeneticAlgorithmError`.
  This implicitly provides support for the `failure` crate.
* minor internal changes to ease development 

## 0.2.0 : 2019-06-24

* implement `RandomValueMutation` for `bool`
* use `rand_xoshiro` crate for pseudo random number generation
* migrate `rand` crate to version 0.6.x
* do not use references to primitive types in function parameters or return types 
* migrate to Rust 2018 edition
* use `criterion` for benchmarking on stable Rust

## 0.1.2 : 2017-11-07

* fix some mistakes in the documentation

## 0.1.1 : 2017-11-06 : First words

* Describe the basic building blocks (traits) defined in this crate.<br/>
  (documentation only, no code changes)

## 0.1.0 : 2017-10-26 : Newborn
First release
