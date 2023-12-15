use num_traits::Float;

use crate::defs::bivector::Bivector;


impl<N: Float + PartialEq> PartialEq for Bivector<N> {
    fn eq(&self, other: &Self) -> bool {
        self.e01 == other.e01 
        && self.e20 == other.e20 
        && self.e12 == other.e12
    }
}