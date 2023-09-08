//! Primitives for Durin, a library for building solvers for the OP Stack's
//! dispute protocol.

extern crate alloy_primitives;
extern crate anyhow;

mod dispute_game;
pub use dispute_game::{Claim, GameStatus, GameType};

mod traits;
pub use traits::{DisputeAgent, DisputeGame};

mod fault;
pub use fault::prelude::*;
