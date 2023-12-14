use num_traits::Float;

use crate::vectors::vector::Vector;

impl<N: Float + PartialEq> PartialEq for Vector<N> {
    fn eq(&self, other: &Self) -> bool {
        self.e0 == other.e0 
        && self.e1 == other.e1 
        && self.e2 == other.e2
    }
}