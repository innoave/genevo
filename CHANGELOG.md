# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## 0.4.0 : <unreleased>

### Breaking changes: 

* remove method `step_with_seed` from `Simulation` trait.
* remove field `seed` from `State` struct of the simulation.
* the `SimulationBuilder` trait requires an additional method `build_with_seed`.
* remove variant `Unexpected` from `SimError` enum.
* method `processing_time` on `TrackProcessingTime` trait now returns owned 
  `ProcessingTime` instead of a reference to `ProcessingTime`. 

### Fixed issues:

* accumulate processing time for final simulation result

## 0.3.0 : 2019-06-25

### Breaking changes:

* make support for `FixedBitSet` an optional crate feature
* replace `DiscreteCrossBreeder` by integrating it into `UniformCrossBreeder`

### New features:

* add support for `SmallVec` as optional crate feature
* implement std `Error` trait for `SimError` and `GeneticAlgorithmError`.
  This implicitly provides support for the `failure` crate.
* minor internal changes to ease development

### Fixed issues: 

* make support for `Vec<bool>` consistent through all building blocks
* tracking of accumulated processing time not correct 

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
