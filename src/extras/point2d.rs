use std::{fmt::Display, ops::Add};

use num_traits::Float;

use crate::{defs::{bivector::Bivector, vector::Vector}, traits::RegressiveProduct};

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

/// Addition between two points is a common opertion.
impl<N: Float> Add<Point2d<N>> for Point2d<N> {
    type Output = Self;

    /// Add two points.
    /// ```rust
    /// use simply_2dpga::extras::point2d::*;
    /// 
    /// let p1 = Point2d{x: 1.0, y: 2.0};
    /// let p2 = Point2d{x: 3.0, y: 4.0};
    /// let sum = p1+p2;
    /// let expected = Point2d{x: 4.0, y: 6.0};
    /// assert_eq!(sum, expected);
    /// ```
    fn add(self, rhs: Point2d<N>) -> Self::Output {
        Point2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}