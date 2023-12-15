// Users can manually include k-vectors if they want to use them.
pub use crate::defs::{multivector::*, vector::*, bivector::*, trivector::*};

// We want to include all geometric algebra operators.
pub use crate::traits::*;

// Probably for the best that equality comparison is included by default.
pub use crate::equality::*;