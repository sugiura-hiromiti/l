//! utility module to extending existing crates and to re-export better names.
//! thus, this module has 2 aspects. one is as utility library, another is convenient
//! `preruldes::*`,

pub mod container;
pub mod itr;
pub use anyhow::Result as Rslt;
