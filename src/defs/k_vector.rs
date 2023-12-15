use std::fmt::Display;

use num_traits::Float;
use crate::{defs::vector::Vector, traits::OuterProduct};
use crate::defs::bivector::Bivector;
use crate::defs::trivector::Trivector;
use crate::traits::GeometricProduct;

use super::{multivector::Multivector};

/// A K-Vector, where 'K' is the grade of the vector.
#[derive(Clone, Debug)]
pub enum KVector<N: Float> {
    Scalar(N),
    Vector(Vector<N>),
    Bivector(Bivector<N>),
    Trivector(Trivector<N>),
}

/// Display trait for k-vector.  Simply displays whatever it's wrapped.
impl<N: Float+Display> Display for KVector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KVector::Scalar(s) => write!(f, "{}", *s),
            KVector::Vector(v) => write!(f, "{}", v),
            KVector::Bivector(bv) => write!(f, "{}", bv),
            KVector::Trivector(tv) => write!(f, "{}", tv),
        }
    }
}

/// Utilitiy functions for castingk-vector enums to particular vector types.
impl<N: Float> KVector<N> {
    /// Attempt to cast this K-Vector to a scalar.
    pub fn to_scalar(&self) -> Result<N, &'static str> {
        if let KVector::Scalar(s) = *self {
            return Result::Ok(s);
        }
        return Err("Illegal scalar cast!");
    }
    /// Attempt to cast this K-Vector to a 1-vector.
    pub fn to_vector(&self) -> Result<Vector<N>, &'static str> {
        if let KVector::Vector(v) = self {
            return Result::Ok(v.clone());
        }
        return Err("Illegal vector cast!");
    }
    /// Attempt to cast this K-Vector to a bivector.
    pub fn to_bivector(&self) -> Result<Bivector<N>, &'static str> {
        if let KVector::Bivector(b) = self {
            return Result::Ok(b.clone());
        }
        return Err("Illegal bivector cast!");
    }
    /// Attempt to cast this K-Vector to a trivector.
    pub fn to_trivector(&self) -> Result<Trivector<N>, &'static str> {
        if let KVector::Trivector(t) = self {
            return Result::Ok(t.clone());
        }
        return Err("Illegal trivector cast!");
    }
}

/// Private helpers for the general k-vector geometric product.
impl<N: Float> KVector<N> {
    /// Geometric product when 'this' is a scalar.
    fn geo_scalar(self_scalar: &N, other: &Self) -> Multivector<N> {
        match other {
            // Note: Scalar multiplication is commutitive.
            KVector::Scalar(other_scalar) => {
                Multivector::from_scalar((*self_scalar) * (*other_scalar))
            },
            KVector::Vector(other_vec) => {
                Multivector::from_vector(other_vec * (*self_scalar))
            },
            KVector::Bivector(other_bivec) => {
                Multivector::from_bivector(other_bivec * (*self_scalar))
            },
            KVector::Trivector(other_trivec) => {
                Multivector::from_trivector(other_trivec * (*self_scalar))
            }
        }
    }
    
    /// Geometric product when 'this' is a vector.
    fn geo_vector(self_vec: &Vector<N>, other: &Self) -> Multivector<N> {
        match other {
            KVector::Scalar(other_scalar) => {
                Multivector::from_vector(self_vec * (*other_scalar))
            },
            KVector::Vector(other_vec) => {
                self_vec.geo(other_vec)
            },
            KVector::Bivector(other_bivec) => {
                self_vec.geo(other_bivec)
            },
            KVector::Trivector(other_trivec) => {
                self_vec.geo(other_trivec)
            }
        }
    }
    
    /// Geometric product when 'this' is a bivector.
    fn geo_bivector(self_bivec: &Bivector<N>, other: &Self) -> Multivector<N> {
        match other {
            KVector::Scalar(other_scalar) => {
                Multivector::from_bivector(self_bivec * (*other_scalar))
            },
            KVector::Vector(other_vec) => {
                self_bivec.geo(other_vec)
            },
            KVector::Bivector(other_bivec) => {
                self_bivec.geo(other_bivec)
            },
            KVector::Trivector(other_trivec) => {
                self_bivec.geo(other_trivec)
            }
        }
    }
    
    /// Geometric product when 'this' is the trivector.
    fn geo_trivector(self_trivec: &Trivector<N>, other: &Self) -> Multivector<N> {
        match other {
            KVector::Scalar(other_scalar) => {
                Multivector::from_trivector(self_trivec * (*other_scalar))
            },
            KVector::Vector(other_vec) => {
                self_trivec.geo(other_vec)
            },
            KVector::Bivector(other_bivec) => {
                self_trivec.geo(other_bivec)
            },
            KVector::Trivector(other_trivec) => {
                self_trivec.geo(other_trivec)
            }
        }
    }
}

/// Geometric product for a generic K-Vector.
impl<N: Float> GeometricProduct<KVector<N>, N> for KVector<N> {
    /// Perform the geometric product with another k-vector. Will return a multivector, even if that multivector only contains one grade.
    fn geo(&self, other: &Self) -> Multivector<N> {
         match self {
            KVector::Scalar (s) => KVector::geo_scalar(s, other),
            KVector::Vector (vec) => KVector::geo_vector(vec, other),
            KVector::Bivector (bivec) => KVector::geo_bivector(bivec, other),
            KVector::Trivector (e012) => KVector::geo_trivector(e012, other)
        }
    }
}

/// Private helpers for the general k-vector wedge product.
impl<N: Float> KVector<N> {
    /// Helper function for when the lhs is a vector.
    fn wedge_vec(self_vec: &Vector<N>, other: &KVector<N>) -> KVector<N> {
        match other {
            KVector::Scalar(other_scalar) => {
                KVector::Vector(self_vec * (*other_scalar))
            },
            KVector::Vector(other_vec) => {
                self_vec.wedge(other_vec).to_k_vector()
            },
            KVector::Bivector(other_bivec) => {
                self_vec.wedge(other_bivec).to_k_vector()
            },
            KVector::Trivector(_other_trivec) => {
                // Vector wedged with a trivector will try to make a 4-vector, which is not accepted.
                return KVector::Scalar(N::zero())
            }
        }
    }
    /// Helper function for when the lhs is a bivector.  
    fn wedge_bivec(self_bivec: &Bivector<N>, other: &KVector<N>) -> KVector<N> {
        match other {
            KVector::Scalar(other_scalar) => {
                KVector::Bivector(self_bivec * (*other_scalar))
            },
            KVector::Vector(other_vec) => {
                self_bivec.wedge(other_vec).to_k_vector()
            },
            // Any higher, and we get into grades that don't eexist.
            _ => KVector::Scalar(N::zero())
        }
    }
    /// Helper function for when the lhs is a trivector.
    fn wedge_trivec(self_trivec: &Trivector<N>, other: &KVector<N>) -> KVector<N> {
        match other {
            KVector::Scalar(other_scalar) => {
                KVector::Trivector(self_trivec * (*other_scalar))
            },
            // Any other grade of K-Vector will cause the wedge product to ask for a '4-vector' or higher, which does not exist in this vector space.
            _ => KVector::Scalar(N::zero())
        }
    }


}

/// General K-Vector wedge product.
impl<N: Float> OuterProduct<KVector<N>, N> for KVector<N> {
    type Output = KVector<N>;

    /// Perform the wedge product between two K-Vectors. Results are not cast.
    fn wedge(&self, other: &KVector<N>) -> Self::Output {
        match self {
            // Wedging a scalar just gives you scalar multiplication. So I'm doing that inline.
            KVector::Scalar(self_scalar) => {
                match other {
                    KVector::Scalar(other_scalar) => {
                        KVector::Scalar((*self_scalar) * (*other_scalar))
                    }
                    KVector::Vector(other_vec) => {
                        KVector::Vector(other_vec * *self_scalar) // praying this doesnt cause borrowing problems
                    },
                    KVector::Bivector(other_bivec) => {
                        KVector::Bivector(other_bivec * *self_scalar)
                    },
                    KVector::Trivector(other_trivec) => {
                        KVector::Trivector(other_trivec * *self_scalar)
                    }
                }
            },
            // 'this' is a vector
            KVector::Vector(self_vec) => KVector::wedge_vec(self_vec, other),
            // 'this' is a bivector
            KVector::Bivector(self_bivec) => KVector::wedge_bivec(self_bivec, other),
            // 'this' is a trivector
            KVector::Trivector(self_trivec) => KVector::wedge_trivec(self_trivec, other)
        }
    }
}

// TODO: Inner product

// TODO: Grade operator

// TODO: Dagger operator

// TODO: Inverse operator