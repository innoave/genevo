
//
// Genetic
//
pub use genetic::{Fitness, FitnessFunction, Genotype, Phenotype};

//
// Algorithm
//
pub use algorithm::{GeneticAlgorithm, genetic_algorithm};

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
pub use simulation::ga::simulate;

//
// Termination
//
pub use termination::combinator::{and, or, Or, And};
pub use termination::limit::*;
