use std::fmt::Display;

use num_traits::Float;

use crate::{vectors::{bivector::Bivector, vector::Vector}, traits::RegressiveProduct};

/// In 2d pga, a point is a bivector.  This will be a simple eucludian point that
/// can be converted to and from the bivector struct, as a convenient handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point2d<N: Float> {
    pub x: N,
    pub y: N
}

// Constructors and conversions for point 2d //

impl<N: Float> Point2d<N> {
    /// Create a new point in Eucludian space.
    pub fn new(x: N, y: N) -> Point2d<N> {
        Point2d {x, y}
    }

    /// Cast your 2d point to a bivector.
    pub fn to_bivector(&self) -> Bivector<N> {
        Bivector {
            e12: N::from(1.0).unwrap(),
            e20: self.x,
            e01: self.y,
        }
    }

    /// Build a 2d point from a bivector.
    pub fn from_bivector(bivector: &Bivector<N>) -> Point2d<N> {
        // Implicitely normalizing the bivector.
        Point2d {
            x: bivector.e20/bivector.e12,
            y: bivector.e01/bivector.e12,
        }
    }
}

// Some magic pga utilities!

impl<N: Float> Point2d<N> {
    /// Create a line between two points using the regressive product.
    /// Note that in 2d PGA, a vector is a line.
    pub fn line_between_points(p1: Point2d<N>, p2: Point2d<N>) -> Vector<N> {
        let bv1 = p1.to_bivector();
        let bv2 = p2.to_bivector();
        bv1.regressive(&bv2)
    }
}

// Traits

impl<N: Float+Display> Display for Point2d<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
