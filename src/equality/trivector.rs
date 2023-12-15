use num_traits::Float;

use crate::defs::trivector::Trivector;


impl<N: Float + PartialEq> PartialEq for Trivector<N> {
    fn eq(&self, other: &Self) -> bool {
        self.e012 == other.e012
    }
}