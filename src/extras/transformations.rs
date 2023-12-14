use num_traits::Float;

use crate::{vectors::{vector::Vector, bivector::{Bivector}, trivector::Trivector, multivector::Multivector}, traits::{GeometricProduct, Dagger}};

use super::{angle::Angle, point2d::Point2d};

// BASIC REFLECTIONS //

impl<N: Float> Multivector<N> {
    /// Reflect a multivector across a vector (line).   
    /// Note that the result will NOT be normalized! 
    pub fn reflect(&self, other: &Vector<N>) -> Multivector<N> {
        let a = other.geo(self);
        self.geo(&a)
    }   
}
impl<N: Float> Vector<N> {
    /// Reflect a vector (line) across another vector (line).   
    /// Note that the result will NOT be normalized! 
    pub fn reflect(&self, other: &Vector<N>) -> Vector<N> {
        other.geo(&self.geo(other))
            .vector
    }   
}
impl<N: Float> Bivector<N> {
    /// Reflect a bivector (point) across another vector (line).   
    /// Note that the result will NOT be normalized! 
    pub fn reflect(&self, other: &Vector<N>) -> Bivector<N> {
        other.geo(&self.geo(other))
            .bivector
    }   
}

// TRANSFORMERS //

/// A thing that can apply a sandwich product to do a rigid transformation for you!
pub trait RigidTransformation<V> {
    /// Apply a sandwich product. The target will be sandwiched between
    /// the rigid transform's multivector and its reverse.
    fn apply(&self, target: &V) -> V;
}


/// A general rigid transformation handler.
pub struct Transformer<N: Float> {
    multivector: Multivector<N>
}
impl<N: Float> Transformer<N> {
    pub fn get_multivector(&self) -> &Multivector<N> {
        &self.multivector
    }
}
impl<N: Float> RigidTransformation<Multivector<N>> for Transformer<N> {
    fn apply(&self, target: &Multivector<N>) -> Multivector<N> {
        self.multivector.reverse()
            .geo(&target.geo(&self.multivector))
    }
}
impl<N: Float> RigidTransformation<Vector<N>> for Transformer<N> {
    fn apply(&self, target: &Vector<N>) -> Vector<N> {
        self.multivector.reverse()
            .geo(&target.geo(&self.multivector))
            .vector
    }
}
impl<N: Float> RigidTransformation<Bivector<N>> for Transformer<N> {
    fn apply(&self, target: &Bivector<N>) -> Bivector<N> {
        self.multivector.reverse()
            .geo(&target.geo(&self.multivector))
            .bivector
    }
}


pub struct Rotor<N: Float> {
    angle: Angle<N>,
    transformer: Transformer<N>,
}
impl<N: Float> Rotor<N> {
    /// Create a rotor with an axis (point we want to rotate around) and an angle.
    pub fn new(axis: Point2d<N>, angle: Angle<N>) -> Rotor<N> {
        let alpha = angle.get_radians();
        let half = N::from(0.5).unwrap();
        let cos = (alpha * half).cos();
        let sin = (alpha * half).sin();
        let mv = Multivector {
            scalar: cos,
            vector: crate::vectors::vector::Vector::zero(),
            bivector: (axis.to_bivector() * sin),
            trivector: Trivector::zero(),
        };
        Rotor { angle, transformer: Transformer { multivector: mv }}
    }

    /// Gets this transfermor's angle by value.  Should be immutable.
    pub fn get_angle(&self) -> Angle<N> {
        self.angle
    }
    /// Get the underlying transformer.
    pub fn get_transformer(&self) -> &Transformer<N> {
        &self.transformer
    }
}
impl<N: Float> RigidTransformation<Multivector<N>> for Rotor<N> {
    fn apply(&self, target: &Multivector<N>) -> Multivector<N> {
        self.transformer.apply(target)
    }
}
impl<N: Float> RigidTransformation<Vector<N>> for Rotor<N> {
    fn apply(&self, target: &Vector<N>) -> Vector<N> {
        self.transformer.apply(target)
    }
}
impl<N: Float> RigidTransformation<Bivector<N>> for Rotor<N> {
    fn apply(&self, target: &Bivector<N>) -> Bivector<N> {
        self.transformer.apply(target)
    }
}

/// Motors are a translator in PGA.  They can be combined with rotors to represent any rigid transformation.
pub struct Motor<N: Float> {
    x: N,
    y: N,
    displacement: N,
    transformer: Transformer<N>,
}
/// A motor- which handles translations.
impl<N: Float> Motor<N> {
    /// Create a new motor! 'x' and 'y' are a direction (point at infinity), and 'd' is displacement.
    pub fn new(x: N, y: N, d: N) -> Motor<N> {
        let direction = Bivector {
            e20: x,
            e01: y,
            e12: N::zero()
        };
        let half = N::from(0.5).unwrap();

        let multivector = Multivector {
                scalar: N::from(1.0).unwrap(),
                vector: Vector::zero(),
                bivector: direction * (d * half),
                trivector: Trivector::zero()
            };

        Motor {
            x, y,
            displacement: d,
            transformer: Transformer { multivector }
        }
    }

    /// The point at infinity this motor translates towards.
    pub fn get_direction(&self) -> Bivector<N> {
        Bivector {
            e20: self.x,
            e01: self.y,
            e12: N::zero()
        }
    }

    /// The displacement of this translation.
    pub fn get_displacement(&self) -> N {
        self.displacement
    }

    /// Get the underlying transformer.
    pub fn get_transformer(&self) -> &Transformer<N> {
        &self.transformer
    }
}
impl<N: Float> RigidTransformation<Multivector<N>> for Motor<N> {
    fn apply(&self, target: &Multivector<N>) -> Multivector<N> {
        self.transformer.apply(target)
    }
} 
impl<N: Float> RigidTransformation<Bivector<N>> for Motor<N> {
    fn apply(&self, target: &Bivector<N>) -> Bivector<N> {
        self.transformer.apply(target)
    }
} 
impl<N: Float> RigidTransformation<Vector<N>> for Motor<N> {
    fn apply(&self, target: &Vector<N>) -> Vector<N> {
        self.transformer.apply(target)
    }
} 


/// Combines multiple rigid transforms into one transform.
pub struct MultiTransform<N: Float> {
    multivector: Multivector<N>
}
impl<N: Float> MultiTransform<N> {
    pub fn new(transformations: Vec<&Transformer<N>>) -> MultiTransform<N> {
        let mut mv = transformations
            .first()
            .unwrap()
            .multivector
            .clone();

        for tr in transformations.iter().skip(1) {
            mv = mv.geo(&tr.multivector);
        }

        MultiTransform { 
            multivector: mv
        }
    }
}
impl<N: Float> RigidTransformation<Multivector<N>> for MultiTransform<N> {
    fn apply(&self, target: &Multivector<N>) -> Multivector<N> {
        self.multivector.reverse()
            .geo(&target.geo(&self.multivector))
    }
}
impl<N: Float> RigidTransformation<Vector<N>> for MultiTransform<N> {
    fn apply(&self, target: &Vector<N>) -> Vector<N> {
        self.multivector.reverse()
            .geo(&target.geo(&self.multivector))
            .vector
    }
}
impl<N: Float> RigidTransformation<Bivector<N>> for MultiTransform<N> {
    fn apply(&self, target: &Bivector<N>) -> Bivector<N> {
        self.multivector.reverse()
            .geo(&target.geo(&self.multivector))
            .bivector
    }
}
