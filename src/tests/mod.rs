
#[cfg(test)]
mod multivector_tests {
    // use super::*;

    #[test]
    fn test_geometric_product() {
        // ...
    }
}

#[cfg(test)]
mod k_vector_tests {
    use crate::{vectors::vector::Vector, traits::{GeometricProduct, Contraction}};

    #[test]
    fn test_geometric_product() {
        let v1 = Vector {
            e0: 1.0f32,
            e1: 2.0f32,
            e2: 3.0f32
        };

        let v2 = Vector {
            e0: 2.0f32,
            e1: 3.0f32,
            e2: 4.0f32
        };

        let _product = v1.geo(&v2);

        // print!("v1: {}\nv2:{}\n Geometric product: {}", v1, v2, product) // todo: implement display on multivector!
    }

    #[test]
    fn test_wedge_product() {

    }

    #[test]
    fn test_dot_product() {
        let v1: Vector<f32> = Vector {
            e0: 2.0,
            e1: 2.0,
            e2: 0.0,
        };
        
        let v2: Vector<f32> = Vector {
            e0: 0.0,
            e1: 2.0,
            e2: 1.0,
        };

        let _dot = v1.inner(&v2);

        //print!("dot product was {}", dot);
    }

}
