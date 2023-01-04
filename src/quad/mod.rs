//! Tools to construct a quad tree of 2-dimensional quadrants
//! holding "newtonian" bodies and calculating their gravitational forces.
use super::*;

mod body;
pub use body::*;
mod quadnode;
pub use quadnode::*;
