use std::fmt::Display;

use num_traits::Float;
use crate::{traits::{GeometricProduct, GradeProjection, Dagger, GradeInvolution, MagnitudeSqr}};

use super::{bivector::Bivector, trivector::{Trivector}, vector::Vector, k_vector::KVector};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Multivector<N>
where N: Float {
    pub trivector: Trivector<N>,
    pub bivector: Bivector<N>,
    pub vector: Vector<N>,
    pub scalar: N    
}

// Constructors
impl<N: Float> Multivector<N> {
    /// A multivector with all components zero'd out.
    pub fn zero() -> Multivector<N> {
        Multivector { 
            trivector: Trivector::zero(), 
            bivector: Bivector::zero(), 
            vector: Vector::zero(), 
            scalar: N::zero()
        }
    }

    /// Create a multivector with only a scalar component.
    pub fn from_scalar(scalar: N)  -> Multivector<N> {
        Multivector { 
            trivector: Trivector::zero(),
            bivector: Bivector::zero(),
            vector: Vector::zero(),
            scalar
        }
    }

    /// Creates a multivector out of the provided vector component.
    pub fn from_vector(vector: Vector<N>) -> Multivector<N> {
        Multivector { 
            trivector: Trivector::zero(),
            bivector: Bivector::zero(),
            vector,
            scalar: N::zero()
        }
    }

    /// Creates a multivector out of the provided bivector component.
    pub fn from_bivector(bivector: Bivector<N>) -> Multivector<N> {
        Multivector { 
            trivector: Trivector::zero(),
            bivector,
            vector: Vector::zero(),
            scalar: N::zero()
        }
    }

    /// Creates a multivector with the pseudoscalar (trivector)'s coefficient set to the provided value.
    pub fn from_trivector(trivector: Trivector<N>) -> Multivector<N> {
        Multivector { 
            trivector,
            bivector: Bivector::zero(),
            vector: Vector::zero(),
            scalar: N::zero()
        }
    }
}

impl<N: Float+Display> Display for Multivector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scalar = self.scalar;
        let vector = &self.vector;
        let bivector = &self.bivector;
        let pseudoscalar = &self.trivector;
        write!(f, "{{\n\t{scalar}\n\t+ {vector}\n\t+ {bivector}\n\t+ {pseudoscalar}\n}}")
    }
}

// Geometric product between a multivector and another multivector.
impl<N> GeometricProduct<Multivector<N>, N> for Multivector<N> 
where N: Float {
    fn geo(&self, other: &Multivector<N>) -> Multivector<N> {
        // Using Cayley table to hardcode the operations:
        let scalar: N = 
            self.vector.e1      * other.vector.e1
            + self.vector.e2    * other.vector.e2
            - self.bivector.e12 * other.bivector.e12;

        let vector: Vector<N> = Vector {
            e0: 
                - self.vector.e1 * other.bivector.e01
                + self.vector.e2 * other.bivector.e20
                + self.bivector.e01 * other.vector.e1
                - self.bivector.e20 * other.vector.e2
                - self.bivector.e12 * other.trivector.e012
                + self.trivector.e012 * other.bivector.e12
                + self.scalar * other.vector.e0  // scalar
                + self.vector.e0 * other.scalar, // scalar
            e1: 
                - self.vector.e2    * other.bivector.e12
                + self.bivector.e12 * other.vector.e2
                + self.scalar * other.vector.e1  // scalar
                + self.vector.e1 * other.scalar, // scalar
            e2:
                self.vector.e1      * other.bivector.e12
                - self.bivector.e12 * other.vector.e1
                + self.scalar * other.vector.e2  // scalar
                + self.vector.e2 * other.scalar, // scalar
        };

        let bivector: Bivector<N> = Bivector { 
            e01: 
                self.vector.e0 * other.vector.e1
                 - self.vector.e1 * other.vector.e0
                 + self.vector.e2 * other.trivector.e012
                 + self.bivector.e20 * other.bivector.e12
                 - self.bivector.e12 * other.bivector.e20
                 + self.scalar * other.bivector.e01  // scalar
                 + self.bivector.e01 * other.scalar, // scalar
            e20: 
                - self.vector.e0 * other.vector.e2
                + self.vector.e1 * other.trivector.e012
                + self.vector.e2 * other.vector.e0
                - self.bivector.e01 * other.bivector.e12
                + self.bivector.e12 * other.bivector.e01
                + self.trivector.e012 * other.vector.e1
                + self.scalar * other.bivector.e20  // scalar
                + self.bivector.e20 * other.scalar, // scalar
            e12: 
                self.vector.e1 * other.vector.e2
                - self.vector.e2 * other.vector.e1
                + self.scalar * other.bivector.e12 // scalar
                + self.bivector.e12 * other.scalar // scalar
        };

        let trivector: Trivector<N> = Trivector { 
            e012:
                self.vector.e0 * other.bivector.e12
                 + self.vector.e1 * other.bivector.e20
                 + self.vector.e2 * other.bivector.e01
                 + self.bivector.e01 * other.vector.e2
                 + self.bivector.e20 * other.vector.e1
                 + self.bivector.e12 * other.vector.e0
                 + self.scalar * other.trivector.e012 // scalar
                 + self.trivector.e012 * other.scalar // scalar
        };

        Multivector { trivector, bivector, vector, scalar }
    }
}


// Grade projection
impl<N: Float> GradeProjection<N> for Multivector<N> {
    fn grade_proj(&self, grade: u16) -> KVector<N> {
        match grade {
            0 => KVector::Scalar(self.scalar),
            1 => self.vector.to_k_vector(),
            2 => self.bivector.to_k_vector(),
            3 => self.trivector.to_k_vector(),
            _ => panic!("Illegal grade projection!")
        }
    }
}

/// Dagger (reverse) operator
impl<N: Float> Dagger for Multivector<N> {
    fn reverse(&self) -> Self {
        let neg = N::from(-1.0).unwrap();
        Multivector {
            scalar: self.scalar,
            vector: self.vector.clone(), // TODO: Does this result in borrwing problems? I think I need to clone this.
            bivector: self.bivector.clone() * neg,
            trivector: self.trivector.clone() * neg
        }        
    }
}

/// Grade involution (star) operator.
impl<N: Float> GradeInvolution for Multivector<N> {
    fn grade_involution(&self) -> Self {
        let neg = N::from(-1.0).unwrap();
        Multivector {
            scalar: self.scalar,
            vector: self.vector.clone() * neg,
            bivector: self.bivector.clone(),
            trivector: self.trivector.clone() * neg
        }
    }
}

impl<N: Float> MagnitudeSqr<N> for Multivector<N> {
    fn magnitude_sqr(&self) -> N {
        self.reverse()
            .geo(&self)
            .scalar // grade projection, but I got lazy.
    }
}