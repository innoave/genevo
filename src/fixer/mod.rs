use crate::operator::{FixerOp, GeneticOperator};
use crate::genetic::Genotype;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NoOpFixer;

impl GeneticOperator for NoOpFixer {
    fn name() -> String {
        String::from("No-Op-Fixer")
    }
}

impl<T> FixerOp<T> for NoOpFixer
    where
        T: Genotype {
    fn fix(&self, _: &mut T) {}
}