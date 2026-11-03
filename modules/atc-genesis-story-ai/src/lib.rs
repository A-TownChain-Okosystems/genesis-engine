#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Deterministic narrative runtime for Genesis Engine.
//!
//! AI providers may create proposals, but only validated transactions mutate
//! narrative state. The checksum exposed by this crate is a non-cryptographic
//! deterministic fingerprint and MUST NOT be used for security decisions.

mod character;
mod consequence;
mod dialogue;
mod error;
mod event;
mod graph;
mod ids;
mod lore;
mod memory;
mod quest;
mod replay;
mod runtime;
mod state;
mod story;
mod timeline;
mod trigger;
mod validation;

pub mod ai;
pub mod integration;

pub use character::*;
pub use consequence::*;
pub use dialogue::*;
pub use error::*;
pub use event::*;
pub use graph::*;
pub use ids::*;
pub use lore::*;
pub use memory::*;
pub use quest::*;
pub use replay::*;
pub use runtime::*;
pub use state::*;
pub use story::*;
pub use timeline::*;
pub use trigger::*;
pub use validation::*;
