use super::*;

mod population_builder {

    use super::*;
    use crate::random::{get_rng, random_seed};
    #[cfg(feature = "fixedbitset")]
    use fixedbitset::FixedBitSet;
    use proptest::prelude::*;
    #[cfg(feature = "smallvec")]
    use smallvec::SmallVec;

    proptest! {

        #[test]
        fn builds_a_population_of_any_number_of_vec_of_bool_genomes(
            size in 0usize..9_999,
        ) {
            let rng = get_rng(random_seed());

            let population: Population<Vec<bool>> = PopulationBuilder::build_population(
                &BinaryEncodedGenomeBuilder::new(42),
                size,
                rng,
            );

            prop_assert_eq!(population.size(), size);
        }

        #[cfg(feature = "fixedbitset")]
        #[test]
        fn builds_a_population_of_any_number_of_fixedbitset_genomes(
            size in 0usize..9_999,
        ) {
            let rng = get_rng(random_seed());

            let population: Population<FixedBitSet> = PopulationBuilder::build_population(
                &BinaryEncodedGenomeBuilder::new(42),
                size,
                rng,
            );

            prop_assert_eq!(population.size(), size);
        }

        #[cfg(feature = "smallvec")]
        #[test]
        fn builds_a_population_of_any_number_of_smallvec_genomes(
            size in 0usize..9_999,
        ) {
            const GENOME_LEN: usize = 24;

            let rng = get_rng(random_seed());

            let population: Population<SmallVec<[bool; GENOME_LEN]>> = PopulationBuilder::build_population(
                &BinaryEncodedGenomeBuilder::new(GENOME_LEN),
                size,
                rng,
            );

            prop_assert_eq!(population.size(), size);
        }
    }
}
