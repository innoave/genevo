# genevo

[![Crates.io][crb]][crl]
[![Docs.rs][dcb]][dcl]
[![Linux Build Status][tcb]][tcl]
[![Windows Build Status][avb]][avl]
[![codevoc.io][cvb]][cvl]
[![MIT/Apache][lib]][lil]
[![Join the chat][gcb]][gcl]

[crb]: https://img.shields.io/crates/v/genevo.svg?style=flat-square
[dcb]: https://docs.rs/genevo/badge.svg?style=flat-square
[tcb]: https://img.shields.io/travis/innoave/genevo/master.svg?style=flat-square
[avb]: https://img.shields.io/appveyor/ci/innoave/genevo.svg
[cvb]: https://img.shields.io/codecov/c/github/innoave/genevo/master.svg?style=flat-square
[lib]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=flat-square
[l1b]: https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square
[l2b]: https://img.shields.io/badge/license-Apache-blue.svg?style=flat-square
[gcb]: https://badges.gitter.im/innoave/general.svg?style=flat-square

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

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
genevo = "0.1"
```

And add this to your crate:

```rust
extern crate genevo;
```

Have a look at the examples to see how to use this crate:
* [monkeys](./examples/monkeys/main.rs): explores the idea of Shakespeare's monkeys, also known
  as the [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem)
* [queens](./examples/queens/main.rs): searches for solutions of the
  [N Queens Problem](https://en.wikipedia.org/wiki/Eight_queens_puzzle)

## Research

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

Copyright &copy; 2017, Innoave.com and contributors
