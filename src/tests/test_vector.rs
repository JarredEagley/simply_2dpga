
#[cfg(test)]
mod vector_operators {
    use crate::{vectors::{vector::Vector, multivector::Multivector, bivector::Bivector, trivector::Trivector}, traits::GeometricProduct};

    #[test]
    fn test_geometric_product() {
        
        // (2e0+2.5e1+3e2)
        // (3.5e0+4.5e1+5.5e2)

        let v1: Vector<f32> = Vector {
            e0: 2.0,
            e1: 2.5,
            e2: 3.0,
        };
        let v2: Vector<f32> = Vector {
            e0: 3.5,
            e1: 4.5,
            e2: 5.5,
        };

        let result1 = v1.geo(&v2);
        let result2 = v2.geo(&v1);

        let correct_result1: Multivector<f32> = Multivector {
            scalar: 27.75,
            vector: Vector::zero(),
            bivector: Bivector { e01: 0.25, e20: -0.5, e12: 0.25},
            trivector: Trivector::zero()           
        };
        let correct_result2: Multivector<f32> = Multivector {
            scalar: 27.75,
            vector: Vector::zero(),
            bivector: Bivector { e01: -0.25, e20: 0.5, e12: -0.25},
            trivector: Trivector::zero()      
        };

        assert_eq!(result1, correct_result1);
        assert_eq!(result2, correct_result2);
    }
}