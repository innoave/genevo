//! The `tournament` module.
//!
//! The provided `SelectionOp` implementations are:
//! * `TournamentSelector`

use genetic::{Fitness, Genotype, Parents};
use operator::{GeneticOperator, SelectionOp, SingleObjective, MultiObjective};
use random::{random_index, random_probability};
use rand::Rng;
use simulation::{EvaluatedPopulation, SimError};


/// The `TournamentSelector` implements the tournament selection method.
/// It runs tournaments with a small size of participants and pick the best
/// performing individuals from each tournament.
///
/// The number of participants in each tournament is configurable by the
/// 'tournament_size` field. A tournament size of 1 is called 1-way tournament
/// and is equivalent to random selection.
///
/// The final selection is picked from the best performing participants in each
/// tournament but with a probability. The probability gives also chances to
/// the second best, third best and so on. The probability is configurable by
/// the `probability` field. A probability of 1.0 means the tournament is
/// deterministic. The best and only the best individual of each tournament is
/// selected.
///
/// To avoid that candidates chosen once are selected again they are removed
/// from the list of candidates. Though this can be configured as well. The
/// field `remove_selected_individuals` controls whether selected candidates
/// are removed or not.
///
/// This `TournamentSelector` can be used for single-objective fitness values
/// as well as multi-objective fitness values.
#[derive(Clone)]
pub struct TournamentSelector {
    /// The fraction of number of parents to select in relation to the
    /// number of individuals in the population.
    selection_ratio: f64,
    /// The number of individuals per parents.
    num_individuals_per_parents: usize,
    /// The number of participants on each tournament.
    tournament_size: usize,
    /// The probability to pick candidates from one tournament.
    /// Values must be between 0 and 1.0 (inclusive).
    probability: f64,
    /// Remove chosen individuals from the list of candidates to avoid that
    /// they can be picked again.
    remove_selected_individuals: bool,
}

impl TournamentSelector {
    /// Constructs a new instance of the `TournamentSelector`.
    pub fn new(selection_ratio: f64,
               num_individuals_per_parents: usize,
               tournament_size: usize,
               probability: f64,
               remove_selected_individuals: bool
    ) -> Self {
        TournamentSelector {
            selection_ratio: selection_ratio,
            num_individuals_per_parents: num_individuals_per_parents,
            tournament_size: tournament_size,
            probability: probability,
            remove_selected_individuals: remove_selected_individuals,
        }
    }

    /// Returns the selection ratio.
    ///
    /// The selection ratio is the fraction of number of parents that are
    /// selected on every call of the `select_from` function and the number
    /// of individuals in the population.
    pub fn selection_ratio(&self) -> f64 {
        self.selection_ratio
    }

    /// Sets the selection ratio to a new value.
    ///
    /// The selection ratio is the fraction of number of parents that are
    /// selected on every call of the `select_from` function and the number
    /// of individuals in the population.
    pub fn set_selection_ratio(&mut self, value: f64) {
        self.selection_ratio = value;
    }

    /// Returns the number of individuals per parents use by this selector.
    pub fn num_individuals_per_parents(&self) -> usize {
        self.num_individuals_per_parents
    }

    /// Sets the number of individuals per parents to the given value.
    pub fn set_num_individuals_per_parents(&mut self, value: usize) {
        self.num_individuals_per_parents = value;
    }

    /// Returns the size of one tournament.
    pub fn tournament_size(&self) -> usize {
        self.tournament_size
    }

    /// Sets the size of one tournament to a given value. The value must be
    /// a positive integer greater 0.
    ///
    /// A tournament size of 1 is called 1-way tournament and is
    /// equivalent to random selection.
    pub fn set_tournament_size(&mut self, value: usize) {
        self.tournament_size = value;
    }

    /// Returns the probability to pick candidates from one tournament.
    pub fn probability(&self) -> f64 {
        self.probability
    }

    /// Set the probability to pick candidates from one tournament to the given
    /// value. The value must be between 0 and 1.0 (inclusive).
    ///
    /// A probability of 1.0 means the tournament is deterministic. The best
    /// and only the best individual of each tournament is selected.
    pub fn set_probability(&mut self, value: f64) {
        self.probability = value;
    }

    /// Returns whether individuals are removed from the list of candidates
    /// after they have been picked once.
    pub fn is_remove_selected_individuals(&self) -> bool {
        self.remove_selected_individuals
    }

    /// Sets whether individuals shall be removed from the list of candidates
    /// after they have been picked once.
    pub fn set_remove_selected_individuals(&mut self, value: bool) {
        self.remove_selected_individuals = value;
    }
}

/// Can be used for single-objective optimization
impl SingleObjective for TournamentSelector {}
/// Can be used for multi-objective optimization
impl MultiObjective for TournamentSelector {}

impl GeneticOperator for TournamentSelector {
    fn name() -> String {
        "Tournament-Selection".to_string()
    }
}

impl<G, F> SelectionOp<G, F> for TournamentSelector
    where G: Genotype, F: Fitness
{
    fn select_from<R>(&self, evaluated: &EvaluatedPopulation<G, F>, rng: &mut R)
        -> Result<Vec<Parents<G>>, SimError>
        where R: Rng + Sized {
        let individuals = evaluated.individuals();
        let fitness_values = evaluated.fitness_values();

        // mating pool holds indices to the individuals and fitness_values slices
        let mut mating_pool: Vec<usize> = (0..fitness_values.len()).collect();

        let num_parents_to_select = (individuals.len() as f64 * self.selection_ratio + 0.5).floor() as usize;
        let target_num_candidates = num_parents_to_select * self.num_individuals_per_parents;

        // select candidates for parents
        let mut picked_candidates = Vec::with_capacity(target_num_candidates);
        let mut count_candidates = 0;
        while count_candidates < target_num_candidates && !mating_pool.is_empty() {
            // fill up tournament with candidates
            let mut tournament = Vec::with_capacity(self.tournament_size);
            let mut count_participants = 0;
            while count_participants < self.tournament_size {
                let random = random_index(rng, mating_pool.len());
                let participant = mating_pool[random];
                tournament.push(participant);
                count_participants += 1;
            }
            if tournament.is_empty() {
               break;
            }
            // sort tournament from best performing to worst performing index
            tournament.sort_by(|x, y| fitness_values[*y].cmp(&fitness_values[*x]));
            // pick candidates with probability
            let mut prob = self.probability; let mut prob_redux = 1.;
            while prob > 0. {
                if random_probability(rng) <= prob {
                    let picked = tournament.remove(0);
                    if self.remove_selected_individuals {
                        match mating_pool.iter().position(|x| *x == picked) {
                            Some(position) => {
                                mating_pool.remove(position);
                            },
                            _ => (),
                        }
                    }
                    picked_candidates.push(picked);
                    count_candidates += 1;
                }
                prob_redux *= 1. - prob;
                prob *= prob_redux;
            }
        }
        // convert selected candidate indices to parents of individuals
        let mut selected: Vec<Parents<G>> = Vec::with_capacity(num_parents_to_select);
        while !picked_candidates.is_empty() {
            let mut tuple = Vec::with_capacity(self.num_individuals_per_parents);
            for _ in 0..self.num_individuals_per_parents {
                // index into individuals slice
                let index_i = picked_candidates.remove(0);
                tuple.push(individuals[index_i].clone());
            }
            selected.push(tuple);
        }
        Ok(selected)
    }
}
