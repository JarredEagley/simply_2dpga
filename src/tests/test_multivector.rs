
#[cfg(test)]
mod multivector_constructors {

}

#[cfg(test)]
mod multivector_operators {
    use crate::{vectors::{multivector::Multivector, vector::Vector, bivector::Bivector, trivector::Trivector}, traits::GeometricProduct};


    #[test]
    fn test_geometric_product() {
        let mv1: Multivector<f32> = Multivector {
            scalar: 0.5,
            vector: Vector{ e0: 2.0, e1: 3.0, e2: 4.0 },
            bivector: Bivector {e01: 5.0, e20: 6.0, e12: 7.0},
            trivector: Trivector {e012: 8.0},
        };
        let mv2: Multivector<f32> = Multivector { 
            scalar: 2.0,
            vector: Vector { e0: 1.0, e1: 2.0, e2: 3.0 }, 
            bivector: Bivector { e01: 3.0, e20: 2.0, e12: 1.0 },
            trivector: Trivector { e012: 2.0 },
        };

        let correct_result_1: Multivector<f32> = Multivector { 
            scalar: 12.0,
            vector: Vector { e0: -26.5, e1: 24.0, e2: -1.5 }, 
            bivector: Bivector { e01: 36.5, e20: 49.0, e12: 15.5 },
            trivector: Trivector { e012: 71.0 }, 
        };
        let correct_result_2: Multivector<f32> = Multivector { 
            scalar: 12.0,
            vector: Vector { e0: -8.5, e1: -10.0, e2: 20.5 },
            bivector: Bivector { e01: 50.5, e20: 21.0, e12: 13.5 },
            trivector: Trivector { e012: 71.0 },
        };

        let result1 = mv1.geo(&mv2);
        let result2 = mv2.geo(&mv1);

        assert_eq!(result1, correct_result_1);
        assert_eq!(result2, correct_result_2);
    }
}