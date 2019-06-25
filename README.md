# genevo

[![Crates.io][crb]][crl]
[![Docs.rs][dcb]][dcl]
[![Linux Build Status][tcb]][tcl]
[![Windows Build Status][avb]][avl]
[![codevoc.io][cvb]][cvl]
[![MIT/Apache][lib]][lil]
[![Join the chat][gcb]][gcl]

[crb]: https://img.shields.io/crates/v/genevo.svg
[dcb]: https://docs.rs/genevo/badge.svg
[tcb]: https://travis-ci.org/innoave/genevo.svg?branch=master
[avb]: https://ci.appveyor.com/api/projects/status/github/innoave/genevo?branch=master&svg=true
[cvb]: https://codecov.io/gh/innoave/genevo/branch/master/graph/badge.svg
[lib]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[l1b]: https://img.shields.io/badge/license-MIT-blue.svg
[l2b]: https://img.shields.io/badge/license-Apache-blue.svg
[gcb]: https://badges.gitter.im/innoave/general.svg

[crl]: https://crates.io/crates/genevo/
[dcl]: https://docs.rs/genevo
[tcl]: https://travis-ci.org/innoave/genevo/
[avl]: https://ci.appveyor.com/project/innoave/genevo
[cvl]: https://codecov.io/github/innoave/genevo?branch=master
[lil]: COPYRIGHT.txt
[l1l]: https://opensource.org/licenses/MIT
[l2l]: https://www.apache.org/licenses/LICENSE-2.0
[gcl]: https://gitter.im/innoave/genevo

_genevo_ provides building blocks to run simulations of optimization and search
problems using [genetic algorithms][GA] ([GA]).

The vision for _genevo_ is to be a flexible and greatly extensible framework
for implementing genetic algorithm applications.

_genevo_ is written in [Rust]. The library's API utilizes lots of traits and
types for modelling the domain of genetic algorithms.

[Documentation](https://docs.rs/genevo)

## Features

This crate provides a default implementation of the genetic algorithm to be used
to find solutions for a wide variety of search and optimization problems.

The implementation is split into building blocks which are all represented by
traits. This crate provides most common implementation for all building blocks.
So it can be used for many problems out of the box.

Anyway if one wants to use different implementations for one or the other
building block it can be extended by implementing any of the traits in a more
sophisticated and customized way.

The building blocks (defined as traits) are:

* Simulation
* Algorithm
* Termination
* Operator
* Population
* Phenotype and Genotype
* FitnessFunction

The simulation can run an algorithm that is executed in a loop. An algorithm
implements the steps to be done for each iteration of the loop. The provided
implementation of the genetic algorithm implements the `Algorithm` trait and
can therefore be executed by the `Simulator` which is the provided
implementation of the `Simulation` trait.

The `Simulator` holds state about the simulation and tracks statistics about
the execution of the algorithm, such as number of iterations and processing
time.

The simulation runs until the termination criteria are met. The termination
criteria can be a single one such as max number of iterations or a logical
combination of multiple termination criteria, e.g. max number of iterations
OR a minimum fitness value has been reached. Of coarse `Termination` is a 
trait as well and one can implement any termination criteria he/she can think
of.

The algorithm can make use of operators that perform different stages of the
algorithm. E.g. the basic genetic algorithm defines the stages: selection,
crossover, mutation and accepting. These stages are performed by the appropriate
operators: `SelectionOp`, `CrossoverOp`, `MutationOp`, `RecombinationOp` and
`ReinsertionOp`.

This crate provides multiple implementations for each one of those operators.
So one can experiment with combining the different implementations to compose
the best algorithm for a specific search or optimization problem. Now you may
have guessed that the defined operators are traits as well and you are free
to implement any of these operators in a way that suits best for your problem
and plug them into the provided implementation of the genetic algorithm.

The genetic algorithm needs a population that it evolves with each iteration.
A population contains a number of individuals. Each individual represents a
possible candidate solution for an optimization problem for which the best 
solution is searched for. This crate provides a `PopulationBuilder` to build 
population of genomes. To run the population builder it needs an implementation
of the `GenomeBuilder` trait. A `GenomeBuilder` defines how to create one 
individual (or genome) within the population.

Last but maybe most important are the traits `Phenotype`, `Genotype` and
`FitnessFunction`. These are the traits which define the domain of the
optimization problem. They must be implemented individually for each application
of the genetic algorithm.

Enough words about the building blocks. Show me some concrete examples. Have
a look at the examples in the examples folder to find out how to use this crate:

* [knapsack](./examples/knapsack/main.rs): tries to solve the
  [0-1 knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem)
* [monkeys](./examples/monkeys/main.rs): explores the idea of Shakespeare's monkeys, also known
  as the [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem)
* [queens](./examples/queens/main.rs): searches for solutions of the
  [N Queens Problem](https://en.wikipedia.org/wiki/Eight_queens_puzzle)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
genevo = "0.4"
```

If you are not using Rust 2018 edition add this to your crate root:

```rust
extern crate genevo;
```

## References

I started this project mainly to learn about genetic algorithms (GAs). During
this journey I searched a lot for information about GA. Here are the links to
sources of information about GA that I found most useful for me. 

[[JFGA]]: Jeremy Fisher: Genetic Algorithms

[JFGA]: https://www.youtube.com/watch?v=7J-DfS52bnI&t=302s

[[OBI98]]: Marek Obitko: Genetic Algorithms Tutorial

[OBI98]: http://www.obitko.com/tutorials/genetic-algorithms/

[[GEAT]]: GEATbx: Evolutionary Algorithms
 
[GEAT]: http://www.geatbx.com/docu/algindex.html

[[IGAYT]]: Noureddin Sadawi: A Practical Introduction to Genetic Algorithms
 
[IGAYT]: https://www.youtube.com/playlist?list=PLea0WJq13cnARQILcbHUPINYLy1lOSmjH 

[[CT9YT]]: The Coding Train: 9: Genetic Algorithms - The Nature of Code

[CT9YT]: https://www.youtube.com/playlist?list=PLRqwX-V7Uu6bJM3VgzjNV5YxVxUwzALHV

[[BT95]]: Tobias Blickle, Lothar Thiele, 1995: A Comparison of Selection Schemes used in Genetic Algorithms.

[BT95]: http://www.tik.ee.ethz.ch/file/6c0e384dceb283cd4301339a895b72b8/TIK-Report11.pdf

[[RRCGH]]: StefanoD: Rust_Random_Choice Rust library.

[RRCGH]: https://github.com/StefanoD/Rust_Random_Choice

[[TSP95]]: TSPLIB95: library of sample instances for the TSP (and related problems)

[TSP95]: http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/index.html

--------------------------------------------------------------------------------
[GA]: https://en.wikipedia.org/wiki/Genetic_algorithm
[Rust]: https://www.rust-lang.org/

Copyright &copy; 2017-2019, Innoave.com and contributors
