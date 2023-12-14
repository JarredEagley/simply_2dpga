// Equality comparisons implemented separately, just incase someone wants to fiddle about with
// epsilon values, or otherwise customize.
// Note: Since these are floating point comparisons, I'll be implementing PartialEq but not Eq.

pub mod multivector;
pub mod vector;
pub mod bivector;
pub mod trivector;