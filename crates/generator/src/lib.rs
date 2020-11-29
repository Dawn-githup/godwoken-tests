//! Generator handle layer2 transactions and blocks,
//! and generate new status that can be committed to layer1

pub mod dummy_state;
mod error;
pub mod generator;
pub mod syscalls;
#[cfg(test)]
mod tests;
pub mod traits;

// re-exports
pub use error::Error;
pub use generator::Generator;
pub(crate) use gw_types::bytes;
pub use syscalls::RunResult;
