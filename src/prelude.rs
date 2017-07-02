
//
// Genetic
//
pub use genetic::{Fitness, FitnessFunction, Genotype, Phenotype};

//
// Algorithm
//
pub use algorithm::genetic_algorithm;

//
// Population
//
pub use population::build_population;
pub use population::GenomeBuilder;
pub use population::Population;

//
// Simulation
//
pub use simulation::{SimResult, Simulation, SimulationBuilder};

//
// Termination
//
pub use termination::combinator::{and, or};
pub use termination::limiter::*;
