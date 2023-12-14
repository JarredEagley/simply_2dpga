use std::{fmt::Display, ops::Mul};

use num_traits::Float;

use crate::traits::{GeometricProduct, OuterProduct, GradeProjection, Contraction};

use super::{multivector::{Multivector}, trivector::Trivector, bivector::Bivector, k_vector::KVector};
    
/// In 2d PGA, a vector represents a line with an orientation and magnitude.
#[derive(Clone, Debug)]
pub struct Vector<N> 
where N: Float{
    pub e0: N,
    pub e1: N,
    pub e2: N,
}

// Constructors
impl<N: Float> Vector<N> {
    pub fn zero() -> Vector<N> {
        Vector {
            e0: N::zero(),
            e1: N::zero(),
            e2: N::zero(),
        }
    }

    /// Create a new vector.  Note: In 2D PGA, a 'vector' represents a line with direction and magnitude.  Not an arrow.
    pub fn new(e0: N, e1: N, e2: N) -> Self {
        Vector {
            e0, e1, e2
        }
    }
}

// Conversions

impl<N: Float> Vector<N> {
    /// Get this vector in multivector form.
    pub fn to_multivector(&self) -> Multivector<N> {
        Multivector { 
            trivector: Trivector::zero(), 
            bivector: Bivector::zero(),
            vector: self.clone(), 
            scalar: N::zero()
        }
    }

    /// Get a generic k-vector wrapping this vector.
    pub fn to_k_vector(&self) -> KVector<N> {
        KVector::Vector(self.clone())
    }
}

impl<N: Float+Display> Display for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}e0, {}e1, {}e2 }}", self.e0, self.e1, self.e2)
    }
}

// Operators //

// impl<N: Float> Vector<N> {
//     /// Manually implemented geometric product between two vectors.
//     /// Note to self: Need to move this into its appropriate trait. 
//     pub fn geo(&self, other: &Self) -> Multivector<N> {
//         Multivector { 
//             trivector: Trivector::zero(),
//             bivector: Bivector {    
//                 e01: (self.e0*other.e1) - (self.e1*other.e0),
//                 e20: (self.e2*other.e0) - (self.e0*other.e2),
//                 e12: (self.e1*other.e2) - (self.e2*other.e1),
//             },
//             vector: Vector::zero(),
//             scalar: (self.e1*other.e1) + (self.e2*other.e2)
//         }
//     }
// }

// Geometric product
impl<N: Float> GeometricProduct<Multivector<N>, N> for Vector<N> {
    fn geo(&self, other: &Multivector<N>) -> Multivector<N> {
        self.to_multivector()
            .geo(other)
    }
}
impl<N> GeometricProduct<Vector<N>, N> for Vector<N>
where N: Float {
    /// WARNING! INEFFICIENT IMPLEMENTATION!
    fn geo(&self, other: &Vector<N>) -> Multivector<N> {
        self.to_multivector()
            .geo(&other.to_multivector())
    }
}
impl<N> GeometricProduct<Bivector<N>, N> for Vector<N>
where N: Float {
    /// WARNING! INEFFICIENT IMPLEMENTATION!
    fn geo(&self, other: &Bivector<N>) -> Multivector<N> {
        self.to_multivector()
            .geo(&other.to_multivector())
    }
}
impl<N> GeometricProduct<Trivector<N>, N> for Vector<N>
where N: Float {
    /// WARNING! INEFFICIENT IMPLEMENTATION!
    fn geo(&self, other: &Trivector<N>) -> Multivector<N> {
        self.to_multivector()
            .geo(&other.to_multivector())
    }
}

// Scalar multiplication
impl<N:Float> Mul<N> for Vector<N> {
    type Output = Self;
    fn mul(self, rhs: N) -> Self::Output {
        Vector {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}
impl<N: Float> Mul<N> for &Vector<N> {
    type Output = Vector<N>;
    fn mul(self, rhs: N) -> Self::Output {
        Vector {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

// Wedge product
impl<N: Float> OuterProduct<Vector<N>, N> for Vector<N> {
    // Vector^Vector = Bivector
    type Output = Bivector<N>;
    
    /// Warning! Unoptimized!
    /// Performs wedge product between two vectors. Automatically casts to bivector!
    fn wedge(&self, other: &Vector<N>) -> Self::Output {
        let product = self.geo(other);
        product.grade_proj(2)
        .to_bivector()
        .unwrap()
    }
}
impl<N: Float> OuterProduct<Bivector<N>, N> for Vector<N> {
    // Vector^BiVector = Trivector
    type Output = Trivector<N>;
    
    /// Warning! Unoptimized!
    /// Performs wedge product between a vector and bivector! Result is automatically cast to a trivector.
    fn wedge(&self, other: &Bivector<N>) -> Self::Output {
        let product = self.geo(other);
        product.grade_proj(3)
        .to_trivector()
        .unwrap()
    }
}

// Inner product operators.
// Note that between two vectors, there's really not any difference between the dot product and the
// contraction operators. I might pull the contraction operators into their own trait because of that, 
// even though they're supposedly more fundamental in GA.
impl<N: Float> Contraction<Vector<N>, N> for Vector<N> {
    // Dotting a vector with a vector, we get 1-1=0; a scalar. This will go for all outputs. 
    // I'll just directly output it as a float. I might convert to the scalar enum if needed. We'll see.
    type DotOutput = N;
    type LeftOutput = N;
    type RightOuptut = N;

    /// Warning! This is an UNOPTIMIZED implementation!
    fn inner(&self, other: &Vector<N>) -> N {
        self.geo(other)
            .grade_proj(0)
            .to_scalar()
            .unwrap()
    }
    
    /// Warning! This is an UNOPTIMIZED implementation!
    fn contract_left(&self, other: &Vector<N>) -> N {
        self.geo(other)
            .grade_proj(0)
            .to_scalar()
            .unwrap()
    }
    
    /// Warning! This is an UNOPTIMIZED implementation!
    fn contract_right(&self, other: &Vector<N>) -> N {
        self.geo(other)
            .grade_proj(0)
            .to_scalar()
            .unwrap()
    }
}