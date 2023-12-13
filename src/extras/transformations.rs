use num_traits::Float;

use crate::{vectors::{vector::Vector, bivector::{Bivector}, multivector::Multivector, trivector::Trivector}, traits::{GeometricProduct, Dagger}};

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
pub trait Transformation<V> {
    fn apply(&self, target: &V) -> V;
}

pub struct Rotor<N: Float> {
    angle: Angle<N>,
    multivector: Multivector<N>
}

impl<N: Float> Rotor<N> {
    /// Create a new rotor from an angle, and a point to rotate around.
    pub fn new(axis: &Point2d<N>, angle: &Angle<N>) -> Rotor<N> {
        let alpha = angle.get_radians();
        let half = N::from(0.5).unwrap();
        let cos = (alpha * half).cos();
        let sin = (alpha * half).sin();

        // let multivector = axis.to_bivector() * (sin+cos);
        let mv = Multivector {
            scalar: cos,
            vector: crate::vectors::vector::Vector::zero(),
            bivector: (axis.to_bivector() * sin),
            trivector: Trivector::zero(),
        };

        Rotor { 
            angle: angle.clone(), 
            multivector: mv
        }
    }

    pub fn get_angle(&self) -> Angle<N> {
        self.angle
    }
}
impl<N: Float> Transformation<Bivector<N>> for Rotor<N> {
    fn apply(&self, target: &Bivector<N>) -> Bivector<N> {
        // Sandwich product with the reverse multivector, the input vector, and the multivector.
        self.multivector.reverse().geo(&target.geo(&self.multivector))
            .bivector // take only the bivector component.
    }
}
impl<N: Float> Transformation<Vector<N>> for Rotor<N> {
    fn apply(&self, target: &Vector<N>) -> Vector<N> {
        self.multivector.reverse().geo(&target.geo(&self.multivector))
            .vector
    }
}


/// Motors are a translator in PGA.  They can be combined with rotors to represent any rigid transformation.
pub struct Motor<N: Float> {
    x: N,
    y: N,
    displacement: N,
    multivector: Multivector<N>
}

impl<N: Float> Motor<N> {
    pub fn new(x: N, y: N, d: N) -> Motor<N> {
        let direction = Bivector {
            e20: x,
            e01: y,
            e12: N::zero()
        };
        let half = N::from(0.5).unwrap();

        Motor {
            x, y,
            displacement: d,
            multivector: Multivector {
                scalar: N::from(1.0).unwrap(),
                vector: Vector::zero(),
                bivector: direction * (d * half),
                trivector: Trivector::zero()
            }
        }
    }

    pub fn get_direction(&self) -> Bivector<N> {
        Bivector {
            e20: self.x,
            e01: self.y,
            e12: N::zero()
        }
    }
    pub fn get_displacement(&self) -> N {
        self.displacement
    }
}

impl<N: Float> Transformation<Bivector<N>> for Motor<N> {
    fn apply(&self, target: &Bivector<N>) -> Bivector<N> {
        // Sandwich product with the reverse multivector, the input vector, and the multivector.
        self.multivector.reverse().geo(&target.geo(&self.multivector))
            .bivector // take only the bivector component.
    }
} 
impl<N: Float> Transformation<Vector<N>> for Motor<N> {
    fn apply(&self, target: &Vector<N>) -> Vector<N> {
        self.multivector.reverse().geo(&target.geo(&self.multivector))
            .vector
    }
} 

//  // I'll figure out how to make that work later!
// /// General rigid transformation encapsulation!
// pub struct Transformer<N: Float> {
//     multivector: Multivector<N>
// }
// impl<N: Float> Transformer<N> {
//     pub fn new(transformations: Vec<dyn Transformation<Multivector<N>>>) -> Transformer<N> {

//     }
// }