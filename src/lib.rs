pub mod defs;
pub mod traits;
pub mod extras;
pub mod prelude;

/// Equality comparisons for k-vectors and multivectors.
/// Included as its own module incase someone wants to change the floating point epsilons.
pub mod equality;

mod tests;
