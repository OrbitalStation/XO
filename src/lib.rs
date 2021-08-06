#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![feature(const_raw_ptr_deref)]

mod types;
pub mod field;
pub mod player;
pub mod ai;

pub use types::{Pos, Cell, FoundResult};
