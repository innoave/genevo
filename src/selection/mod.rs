//! The `selection` module provides implementations of the
//! `operator::SelectionOp` genetic operator.
//!
//! The provided `operator::SelectionOp`s are organized in sub-modules
//! named after the utilized selection method:
//! * `truncation`
//! * `linear_ranking`
//! * `exponential_ranking`
//! * `proportional`
//! * `tournament`

pub mod truncation;

pub mod linear_ranking;

pub mod exponential_ranking;

pub mod proportional;

pub mod tournament;
