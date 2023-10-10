//! The fault module contains types and traits related to the FaultDisputeGame.

extern crate alloy_primitives;
extern crate alloy_sol_types;
extern crate durin_primitives;

#[cfg(test)]
extern crate proptest;

pub mod types;
pub use types::*;

pub mod providers;

mod state;
pub use state::{ClaimData, FaultDisputeState};

pub mod traits;
pub use traits::*;

pub mod solver;
pub use solver::*;

pub mod solvers;
pub use solvers::*;
