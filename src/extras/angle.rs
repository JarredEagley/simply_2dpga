use std::{ops::Add, f32::consts::PI};

use num_traits::Float;

/// Uses radians under the hood.  Handles conversions and such for you.
/// Just going to implement the f32 varient. f64 is overkill.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Angle<N: Float> {
    radians: N
}

impl<N: Float> Angle<N> {
    pub fn from_radians(radians: N) -> Angle<N> {
        Angle { radians }
    }

    pub fn get_radians(&self) -> N {
        self.radians
    }
}

/// Constructors and getters.
impl Angle<f32> {

    pub fn from_degrees(degrees: f32) -> Angle<f32> {
        Angle {
            radians: degrees * (PI/180.0f32)
        }        
    }

    // Might want to cache? Depends on how often this gets called i guess
    pub fn get_degrees(&self) -> f32 {
        self.radians * (180.0f32/PI)        
    }
}

// impl Angle<f64> {
//     pub fn from_degrees(degrees: f64) -> Angle<f64> {
//         todo!()
//     }

//     // Might want to cache? Depends on how often this gets called i guess
//     pub fn get_degrees(&self) -> f64 {
//         todo!()
//     }
// }

/// Need to be able to add to angles together!
impl<N: Float> Add for Angle<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Angle {
            radians: self.radians + rhs.radians
        }
    }
}