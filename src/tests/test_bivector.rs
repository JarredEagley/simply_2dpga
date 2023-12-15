
#[cfg(test)]
mod bivector_operators {
    use crate::{vectors::{bivector::Bivector, multivector::Multivector, vector::Vector, trivector::Trivector}, traits::{GeometricProduct, RegressiveProduct, Normalize}};

    #[test]
    fn test_geometric_product() {
        // (3.5e01+4.5e20+5.5e12)
        // (7e01+6e20+5e12)
        let bv1: Bivector<f32> = Bivector { 
            e01: 3.5,
            e20: 4.5,
            e12: 5.5 
        };
        let bv2: Bivector<f32> = Bivector { 
            e01: 7.0, 
            e20: 6.0, 
            e12: 5.0
        };

        let result1 = bv1.geo(&bv2);
        let result2 = bv2.geo(&bv1);

        let correct_result_1: Multivector<f32> = Multivector {
            scalar: -27.5,
            vector: Vector::zero(),
            bivector: Bivector { e01: -10.5, e20: 21.0, e12: 0.0 },
            trivector: Trivector::zero(),
        };
        let correct_result_2: Multivector<f32> = Multivector {
            scalar: -27.5,
            vector: Vector::zero(),
            bivector: Bivector { e01: 10.5, e20: -21.0, e12: 0.0 },
            trivector: Trivector::zero(),
        };

        assert_eq!(result2, correct_result_2);
        assert_eq!(result1, correct_result_1);
    }

    #[test]
    fn test_regresssive_product() {
        // The join of two points is a line.
        let point1: Bivector<f32> = Bivector{
            e01: 4.0,
            e20: 4.0,
            e12: 1.0,
        };
        let point2: Bivector<f32> = Bivector{
            e01: -1.0,
            e20: -4.0,
            e12: 1.0,
        };

        let join = point1.regressive(&point2);

        let correct_join: Vector<f32> = Vector { 
            e0: -12.0,
            e1: -5.0,
            e2: 8.0
        };

        assert_eq!(join, correct_join);
    }

    #[test]
    fn test_normalization() {
        let bv: Bivector<f32> = Bivector { 
            e01: 3.0,
            e20: 6.0,
            e12: 9.0
        };

        let normalized = bv.normalized();
        let normalized_correct: Bivector<f32> = Bivector {
            e01: 0.3333333433,
            e20: 0.6666666865,
            e12: 1.0
        };

        assert_eq!(normalized, normalized_correct);
    }

}