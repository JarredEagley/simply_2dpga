use num_traits::Float;

use crate::vectors::multivector::Multivector;


impl<N: Float + PartialEq> PartialEq for Multivector<N> {
    fn eq(&self, other: &Self) -> bool {
        self.trivector == other.trivector 
        && self.bivector == other.bivector 
        && self.vector == other.vector 
        && self.scalar == other.scalar
    }
}
