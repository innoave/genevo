//! The `tournament` module.
//!
//! The provided `SelectionOp` implementations are:
//! * `TournamentSelector`

use genetic::{Breeding, Fitness, Genotype};
use operator::{GeneticOperator, SelectionOp, SingleObjective, MultiObjective};
use rand::{Rng, thread_rng};
use simulation::{EvaluatedPopulation, SimError};
use std::marker::PhantomData;


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
pub struct TournamentSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    /// The breeding used to create parents.
    breeding: B,
    /// The number of parents to select.
    num_parents_to_select: usize,
    /// The number of participants on each tournament.
    tournament_size: usize,
    /// The probability to pick candidates from one tournament.
    /// Values must be between 0 and 1.0 (inclusive).
    probability: f64,
    /// Remove chosen individuals from the list of candidates to avoid that
    /// they can be picked again.
    remove_selected_individuals: bool,
    // phantom types
    _g: PhantomData<G>,
}

impl<G, B> TournamentSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    /// Constructs a new instance of the `TournamentSelector`.
    pub fn new(breeding: B,
               num_parents_to_select: usize,
               tournament_size: usize,
               probability: f64,
               remove_selected_individuals: bool
    ) -> TournamentSelector<G, B> {
        TournamentSelector {
            breeding: breeding,
            num_parents_to_select: num_parents_to_select,
            tournament_size: tournament_size,
            probability: probability,
            remove_selected_individuals: remove_selected_individuals,
            _g: PhantomData,
        }
    }

    /// Returns the `Breeding` used by this `TournamentSelector`.
    pub fn breeding(&self) -> &B {
        &self.breeding
    }

    /// Returns the number of parents that are selected on every call of the
    /// `selection` function.
    pub fn num_parents_to_select(&self) -> usize {
        self.num_parents_to_select
    }

    /// Sets the number of parents that are selected on every call of the
    /// `selection` function to a new value.
    pub fn set_num_parents_to_select(&mut self, value: usize) {
        self.num_parents_to_select = value;
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
impl<G, B> SingleObjective for TournamentSelector<G, B> where G: Genotype, B: Breeding<G> {}
/// Can be used for multi-objective optimization
impl<G, B> MultiObjective for TournamentSelector<G, B> where G: Genotype, B: Breeding<G> {}

impl<G, B> GeneticOperator for TournamentSelector<G, B>
    where G: Genotype, B: Breeding<G>
{
    fn name() -> String {
        "Tournament-Selection".to_string()
    }
}

impl<G, F, B> SelectionOp<G, F, B> for TournamentSelector<G, B>
    where G: Genotype, F: Fitness, B: Breeding<G>
{
    fn selection(&self, evaluated: &EvaluatedPopulation<G, F>) -> Result<Vec<B::Parents>, SimError> {
        let mut rng = thread_rng();
        let individuals = evaluated.individuals();
        let fitness_values = evaluated.fitness_values();

        // mating pool holds indices to the individuals and fitness_values slices
        let mut mating_pool: Vec<usize> = (0..fitness_values.len()).collect();

        let parents_size = self.breeding.num_individuals_per_parents();
        let target_num_candidates = self.num_parents_to_select * parents_size;

        // select candidates for parents
        let mut picked_candidates = Vec::with_capacity(target_num_candidates);
        let mut count_candidates = 0;
        while count_candidates < target_num_candidates && !mating_pool.is_empty() {
            // fill up tournament with candidates
            let mut tournament = Vec::with_capacity(self.tournament_size);
            let mut count_participants = 0;
            while count_participants < self.tournament_size {
                let random = rng.gen_range(0, mating_pool.len());
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
                if rng.next_f64() <= prob {
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
        let mut selected: Vec<B::Parents> = Vec::with_capacity(self.num_parents_to_select);
        while !picked_candidates.is_empty() {
            let mut tuple = Vec::with_capacity(parents_size);
            for _ in 0..parents_size {
                // index into individuals slice
                let index_i = picked_candidates.remove(0);
                tuple.push(individuals[index_i].clone());
            }
            selected.push(self.breeding.mate_parents(tuple));
        }
        Ok(selected)
    }
}
