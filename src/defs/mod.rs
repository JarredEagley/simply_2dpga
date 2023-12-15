/// Module which contains the definition for the multivector.
pub mod multivector;

/// Module which contains the definition for a grade-3 trivector.
pub mod trivector;

/// Module which contains the definition for a grade-2 bivector.
pub mod bivector;

/// Module which contains the definition for a grade-1 vector.
pub mod vector;

/// Enum wrapper around vector, bivector, and trivector.
/// Meant to make some chaining geometric operations more streamlined, but ended up being 
/// more or less unnecessary.
/// Might still be helpful in some use cases.
pub mod k_vector;

// Exporting these was a bit of an afterthought, but should make things a little less verbose.
pub use multivector::Multivector;
pub use trivector::Trivector;
pub use bivector::Bivector;
pub use vector::Vector;