use std::{fmt::Display, ops::Mul};

use num_traits::Float;

use crate::traits::{GeometricProduct, OuterProduct, GradeProjection, RegressiveProduct};

use super::{multivector::Multivector, trivector::Trivector, vector::Vector, k_vector::KVector};


/// In 2d PGA, a bivector represents a point in space.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bivector<N>
where N: Float {
    pub e01: N,
    pub e20: N, // This bivector is always written backwards for some reason...
    pub e12: N,
}

// Constructors
impl<N: Float> Bivector<N> {
    pub fn zero() -> Self {
        Bivector { 
            e01: N::zero(),
            e20: N::zero(), 
            e12: N::zero()
        }
    }

    pub fn new(e01: N, e20: N, e12: N) -> Self {
        Bivector { e01, e20, e12 }
    }
}

// Conversions
impl<N: Float> Bivector<N> {
    /// Get this bivector as a multivector struct.
    pub fn to_multivector(&self) -> Multivector<N> {
        Multivector { 
            trivector: Trivector::zero(), 
            bivector: self.clone(),
            vector: Vector::zero(),
            scalar: N::zero()
        }
    }

    /// Get a generalized k-vector wrapping this bivector.
    pub fn to_k_vector(&self) -> KVector<N> {
        KVector::Bivector(self.clone())
    }
}

impl<N: Float+Display> Display for Bivector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}e01, {}e20, {}e12 }}", self.e01, self.e20, self.e12)
    }
}

// Operators //

// Geometric product
impl<N: Float> GeometricProduct<Multivector<N>, N> for Bivector<N> {
    fn geo(&self, other: &Multivector<N>) -> Multivector<N> {
        self.to_multivector()
            .geo(&other)
    }
}
impl<N: Float> GeometricProduct<Bivector<N>, N> for Bivector<N> {
    /// WARNING! INEFFICIENT IMPLEMENTATION!
    fn geo(&self, other: &Bivector<N>) -> Multivector<N> {
        self.to_multivector()
            .geo(&other.to_multivector())
    }
}
impl<N: Float> GeometricProduct<Vector<N>, N> for Bivector<N> {
    /// WARNING! INEFFICIENT IMPLEMENTATION!
    fn geo(&self, other: &Vector<N>) -> Multivector<N> {
        self.to_multivector()
            .geo(&other.to_multivector())
    }
}
impl<N: Float> GeometricProduct<Trivector<N>, N> for Bivector<N> {
    /// WARNING! INEFFICIENT IMPLEMENTATION!
    fn geo(&self, other: &Trivector<N>) -> Multivector<N> {
        self.to_multivector()
            .geo(&other.to_multivector())
    }
}

// Scalar multiplication
impl<N> Mul<N> for Bivector<N>
where N: Float {
    type Output = Self;
    fn mul(self, rhs: N) -> Self::Output {
        Bivector {
            e01: self.e01 * rhs,
            e20: self.e20 * rhs,
            e12: self.e12 * rhs,
        }
    }
}
impl<N> Mul<N> for &Bivector<N>
where N: Float {
    type Output = Bivector<N>;
    fn mul(self, rhs: N) -> Self::Output {
        Bivector {
            e01: self.e01 * rhs,
            e20: self.e20 * rhs,
            e12: self.e12 * rhs,
        }
    }
}

// Wedge product
impl<N: Float> OuterProduct<Bivector<N>, N> for Bivector<N> {
    type Output = N;
    /// The wedge product between two bivectors in this basis is meaningless, and will always return zero.    
    fn wedge(&self, _other: &Bivector<N>) -> Self::Output {
        N::zero()
    }
}
impl<N: Float> OuterProduct<Vector<N>, N> for Bivector<N> {
    /// Bivector^Vector will give a trivector.
    type Output = Trivector<N>;

    /// Warning! Unoptimized!
    /// Perform a wedge product between a bivector and a vector! Result is automatically cast to a trivector!
    fn wedge(&self, other: &Vector<N>) -> Self::Output {
        let product = self.geo(other);
        product.grade_proj(3)
            .to_trivector()
            .unwrap()
    }
}

// Inner product

// Regressive product, just doing between two bivectors for now since that's the one that matters.
impl<N: Float> RegressiveProduct<Bivector<N>, N> for Bivector<N> {
    type Output = Vector<N>;

    /// The line formed by two points. 
    /// This is a specialized form for the join of two bivectors in pga.
    /// This might be wrong? Hopefully not.
    fn regressive(&self, other: &Bivector<N>) -> Self::Output {
        Vector {
            e0: 
                self.e01 * other.e20
                 - self.e20 * other.e01,
            e1: 
                 - self.e01 * other.e12
                 + self.e12 * other.e01,
            e2: 
                self.e20 * other.e12
                 - self.e12 * other.e20
        }
    }
}
