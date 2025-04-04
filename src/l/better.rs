//! utility module to extending existing crates and to re-export better names.
//! thus, this module has 2 aspects. one is as utility library, another is convenient
//! `preruldes::*`,

pub use anyhow::Result as Rslt;

pub mod container;
pub mod integer;
pub mod itr;
