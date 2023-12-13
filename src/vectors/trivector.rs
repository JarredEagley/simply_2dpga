use std::{fmt::Display, ops::Mul};

use num_traits::Float;

use crate::traits::{GeometricProduct, OuterProduct};

use super::{multivector::Multivector, bivector::{Bivector}, vector::Vector, k_vector::KVector};

/// The trivector in this vector space is the pseudoscalar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trivector<N>
where N: Float {
    pub e012: N
}

// Constructors
impl<N: Float> Trivector<N> {
    pub fn zero() -> Self {
        Trivector { e012: N::zero() }
    }

    pub fn new(e012: N) -> Self {
        Trivector { e012 }
    }
}

// Conversions

impl<N: Float> Trivector<N> {
    /// Convert to a multivector.
    pub fn to_multivector(&self) -> Multivector<N> {
        Multivector { 
            trivector: self.clone(),
            bivector: Bivector::zero(),
            vector: Vector::zero(), 
            scalar: N::zero()
        }
    }

    /// Get a generalized k-vector wrapping this trivector.
    pub fn to_k_vector(&self) -> KVector<N> {
        KVector::Trivector(self.clone())
    }
}

impl<N: Float+Display> Display for Trivector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}e012 }}", self.e012 )
    }
}


// Operators //

// Geometric product
impl<N> GeometricProduct<Trivector<N>, N> for Trivector<N> 
where N: Float {
    /// WARNING! Currently inefficient implementation!
    fn geo(&self, other: &Trivector<N>) -> Multivector<N> {
        let self_multivector = self.to_multivector();
        self_multivector.geo(&other.to_multivector())
    }
}
impl<N> GeometricProduct<Vector<N>, N> for Trivector<N> 
where N: Float {
    /// WARNING! Currently inefficient implementation!
    fn geo(&self, other: &Vector<N>) -> Multivector<N> {
        let self_multivector = self.to_multivector();
        self_multivector.geo(&other.to_multivector())
    }
}
impl<N> GeometricProduct<Bivector<N>, N> for Trivector<N> 
where N: Float {
    /// WARNING! Currently inefficient implementation!
    fn geo(&self, other: &Bivector<N>) -> Multivector<N> {
        let self_multivector = self.to_multivector();
        self_multivector.geo(&other.to_multivector())
    }
}

// Scalar multiplication
impl<N> Mul<N> for Trivector<N>
where N: Float {
    type Output = Self;
    fn mul(self, rhs: N) -> Self::Output {
        Trivector {
            e012: self.e012 * rhs,
        }
    }
}
impl<N> Mul<N> for &Trivector<N>
where N: Float {
    type Output = Trivector<N>;
    fn mul(self, rhs: N) -> Self::Output {
        Trivector {
            e012: self.e012 * rhs,
        }
    }
}

// Wedge product.. Just returns zero. But we need an implementation anyways.
impl<N: Float> OuterProduct<Trivector<N>, N> for Trivector<N> {
    type Output = N;
    
    /// The wedge product between two trivectors is meaningless, and will always return zero.
    fn wedge(&self, _other: &Trivector<N>) -> Self::Output {
        N::zero()
    }
}
