use std::fmt::Display;

use num_traits::Float;
use crate::{traits::{GeometricProduct, GradeProjection, Dagger, GradeInvolution, MagnitudeSqr}};

use super::{bivector::Bivector, trivector::{Trivector}, vector::Vector, k_vector::KVector};


/// A geometric algebra multivector.  I find it has a lot of parallels to a transformation matrix.
/// A multivector is comprised of coefficients multiplied with basis k-vector elements.
/// 2D PGA is a 3-dimensional algebra, and thus will have 1 scalar, 3 vectors, 3 bivectors, and 1 trivector.
#[derive(Clone, Debug)]
pub struct Multivector<N>
where N: Float {
    pub scalar: N,
    pub vector: Vector<N>,
    pub bivector: Bivector<N>,
    pub trivector: Trivector<N>,
}

/// Constructors.
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

/// Multivectors implement the display attribute, so long as the float they're defined with also implement Display.
impl<N: Float+Display> Display for Multivector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scalar = self.scalar;
        let vector = &self.vector;
        let bivector = &self.bivector;
        let pseudoscalar = &self.trivector;
        write!(f, "{{\n\t{scalar}\n\t+ {vector}\n\t+ {bivector}\n\t+ {pseudoscalar}\n}}")
    }
}

/// Implementation for the geometric product between two multivectors.
impl<N> GeometricProduct<Multivector<N>, N> for Multivector<N> 
where N: Float {
    /// Geometric product between a multivector and another multivector.
    /// This is the heart of this library.  Almost all other operations can be traced to this function.
    /// 
    /// # Example
    /// ```rust
    /// use simply_2dpga::prelude::*;
    /// 
    /// let mv1: Multivector<f32> = Multivector::from_vector(Vector{ e0: 1.0, e1: 2.0, e2: 3.0 });
    /// let mv2: Multivector<f32> = Multivector::from_vector(Vector{ e0: 4.0, e1: 5.0, e2: 6.0 });
    /// 
    /// let product_1 = mv1.geo(&mv2);
    /// let product_2 = mv2.geo(&mv1);
    /// 
    /// assert_ne!(product_1, product_2);
    /// ```
    fn geo(&self, other: &Multivector<N>) -> Multivector<N> {
        // Using Cayley table to hardcode the operations:
        let scalar: N = 
            self.vector.e1      * other.vector.e1
            + self.vector.e2    * other.vector.e2
            - self.bivector.e12 * other.bivector.e12
            + self.scalar * other.scalar; // Almost forgot this!

        let vector: Vector<N> = Vector {
            e0: 
                - self.vector.e1 * other.bivector.e01
                + self.vector.e2 * other.bivector.e20
                + self.bivector.e01 * other.vector.e1
                - self.bivector.e20 * other.vector.e2
                - self.bivector.e12 * other.trivector.e012
                - self.trivector.e012 * other.bivector.e12
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
                 + self.trivector.e012 * other.vector.e2
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


/// Grade projection
impl<N: Float> GradeProjection<N> for Multivector<N> {
    /// A grade projection operation for a multivector.  This can be safely considered obsolete, as its
    /// simpler to just grab the 'k' component from a multivector directly.
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
    /// A reverse (dagger) operation.  This will flip the sign of the coefficient every two grades.
    /// This is often used in ways analagous to an inverse operation, though they're not the same operation.
    fn reverse(&self) -> Self {
        let neg = N::from(-1.0).unwrap();
        Multivector {
            scalar: self.scalar,
            vector: self.vector.clone(),
            bivector: self.bivector.clone() * neg,
            trivector: self.trivector.clone() * neg
        }        
    }
}

/// Grade involution (star) operator.
impl<N: Float> GradeInvolution for Multivector<N> {
    /// Grade involution operation.  This will flip the sign of the coefficient every grade.
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
    /// The square magnitude of a multivector.  This is produced by multiplying the reverse of the multivector with itself, then
    /// taking the scalar component.
    /// Converting the magnitude squared to a magnitude is left as an exercise for the reader.
    fn magnitude_sqr(&self) -> N {
        self.reverse()
            .geo(&self)
            .scalar // grade projection, but I got lazy.
    }
}