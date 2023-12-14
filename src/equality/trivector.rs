use num_traits::Float;

use crate::vectors::trivector::Trivector;


impl<N: Float + PartialEq> PartialEq for Trivector<N> {
    fn eq(&self, other: &Self) -> bool {
        self.e012 == other.e012
    }
}