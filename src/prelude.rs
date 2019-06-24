// Genetic
//
pub use crate::genetic::{Fitness, FitnessFunction, Genotype, Phenotype};

// Algorithm
//
pub use crate::{
    algorithm::Algorithm,
    ga::{genetic_algorithm, GeneticAlgorithm},
    random::{Prng, Rng, Seed},
};

// Population
//
pub use crate::population::{build_population, GenomeBuilder, Population};

// Simulation
//
pub use crate::simulation::{simulator::simulate, SimResult, Simulation, SimulationBuilder};

// Termination
//
pub use crate::termination::{
    combinator::{and, or, And, Or},
    limit::*,
};
