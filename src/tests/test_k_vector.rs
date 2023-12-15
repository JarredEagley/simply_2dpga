
// #[cfg(test)]
// mod k_vector_operators {
//     use crate::{vectors::{bivector::Bivector, k_vector::KVector, vector::Vector, trivector::Trivector}, traits::GeometricProduct};

//     #[test]
//     fn test_geometric_product() {
//         let point = Bivector {e20: 5.0f32, e01: 1.0, e12: 1.0}
//             .to_k_vector();

//         let line = Vector {
//             e0: 0.0f32,
//             e1: 1.0,
//             e2: 1.0,
//         }
//         .to_k_vector();

//         let i = Trivector {e012: 1.0f32}
//             .to_k_vector();

//         // reflect across x=y line.
//         // let k_vec = line.geo(&point.geo(&line));

//         // Multiply by the unit pseudoscalar.
        
//         todo!("I need to revisit geometric product of K-vectors!")
//     }
// }