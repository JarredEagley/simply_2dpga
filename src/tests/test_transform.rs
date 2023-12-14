
#[cfg(test)]
mod test_reflections {
    // todo: Reflection unit tests
}

#[cfg(test)]
mod test_rotors {
    use crate::{
        vectors::bivector::Bivector, 
        extras::{
            angle::Angle, 
            transformations::{Rotor, RigidTransformation}, 
            point2d::Point2d}
        };

    #[test]
    fn test_rotor_bivector() {
        // The point we're rotating around.
        let axis = Bivector {
            e20: 4.5f32,
            e01: 7.2f32,
            e12: 1.0,
        };
        // The point we're rotating.
        let point = Bivector {
            e20: 3.2f32,
            e01: -4.2,
            e12: 1.0,
        };
        // The point that should result from this rotation.
        let correct_result = Bivector {
            e20: 11.64177848998413f32,
            e01: -1.7802553,
            e12: 1.0,
        };

        // Build the rotor and perform the rotation.
        let rotor_angle = Angle::from_degrees(45.0f32);
        let rotor = Rotor::new(Point2d::from_bivector(&axis), rotor_angle);

        let result = rotor.apply(&point);

        assert_eq!(result, correct_result)
    }
    
    #[test]
    fn test_rotor_vector() {
    }
    
    #[test]
    fn test_rotor_multivector() {
    }
}

#[cfg(test)]
mod test_motors {

}

#[cfg(test)]
mod test_multitransform {

}
