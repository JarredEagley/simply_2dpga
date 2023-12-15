use num_traits::Float;

use crate::defs::{multivector::Multivector, k_vector::KVector};

/// The geometric product.  This is what makes geometric algebra geometric algebra.
pub trait GeometricProduct<S, N> 
where S: GeometricProduct<S, N>, N: Float {
    fn geo(&self, other: &S) -> Multivector<N>;
}

/// The wedge product, analogous to the cross product.  Very useful.  
/// Note: In PGA, the wedge product is also called the 'meet'.
pub trait OuterProduct<S, N> 
where S: OuterProduct<S, N>, N: Float {
    type Output;
    fn wedge(&self, other: &S) -> Self::Output;
}

/// The regressive product.
pub trait RegressiveProduct<S, N>
where S: RegressiveProduct<S, N>, N: Float {
    type Output;
    /// The regressive product, also sometimes called the 'join' in pga.
    fn regressive(&self, other: &S) -> Self::Output;
}

/// Left, right contractions, and of course, the ever-useful dot procut! (Inner product)
pub trait Contraction<S, N> 
where S: Contraction<S, N>, N: Float {
    type DotOutput;
    type LeftOutput;
    type RightOuptut;
    fn inner(&self, other: &S) -> N;
    fn contract_left(&self, other: &S) -> N;
    fn contract_right(&self, other: &S) -> N;
}

/// The dagger operator, also known as 'reverse'.
pub trait Dagger {
    /// In theory: reverses all of the vectors then reorganizes them again, changing minus signs as you go along.
    /// In practice: Flips minus sign every two grades.
    fn reverse(&self) -> Self;
}

pub trait GradeInvolution {
    /// Grade involution! Flips the sign every other grade.
    fn grade_involution(&self) -> Self;
}

// pub trait Inverse {} might not implement.

pub trait GradeProjection<N: Float> {
    fn grade_proj(&self, grade: u16) -> KVector<N>;
}

pub trait MagnitudeSqr<N: Float> {
    fn magnitude_sqr(&self) -> N;
}

/// Return a normalized k-vector.
pub trait Normalize {
    fn normalized(&self) -> Self;
}