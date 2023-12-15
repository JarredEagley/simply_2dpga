/// Primary definitions for the crate.  Multivectors and various grade of k-vector can be found in here,
/// along with an enum wrapper for k-vectors.
pub mod defs;
/// Geometric algebra traits are defined in this module.
pub mod traits;
/// Extra goodies!  Point2d wrapper, rotors, motors, and reflections.
pub mod extras;
/// A prelude, to be used as convenient.
/// Includes multivector, vector, bivector, trivector, traits, and equality comparisons.
pub mod prelude;

/// Equality comparisons for k-vectors and multivectors.
/// Included as its own module incase someone wants to tweak how its handled.
pub mod equality;

/// Unit tests.
mod tests;
