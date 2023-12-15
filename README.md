# Simply: 2D PGA!
Simply, a 2d pga library.  Will be pushed to crates.io Soon.

If you're not familiar with projective geometric algebra, see:
 * https://www.youtube.com/watch?v=0i3ocLhbxJ4
 * https://bivector.net/
 * https://projectivegeometricalgebra.org/

Otherwise, this library is exactly what it sounds like.

# Usage
Usage of this library should be fairly straightforward, assuming a basic understanding of projective geometric algebra.

## Multivectors
A `Multivector<N>` is the most fundamental unit in this library.  If you're coming from traditional vector math, think of it sort of like a transformation matrix.

The `Multivector<N>` struct contains:
 * A Scalar `N`
 * A `Vector<N>`
 * A `Bivector<N>`
 * A `Trivector<N>`

Note the generic '`N`'.  `N` must implement `Float`.  This is to keep this library generalizable.

### declaring a multivector

A multivector with all coefficients set to 0 can be initialized in one line:
```rust
let my_multivector: Multivector<f32> = Multivector::zero();
```

Sometimes you want to initialize a multivector with only a single grade defined.  That can be done as such:
```rust
// A multivector with only grade-0 coefficients:
let my_scalar = Multivector::from_scalar(scalar_number_here);
// A multivector with only grade-1 coefficients:
let my_vector = Multivector::from_vector(vector_defn_here);
// A multivector with only grade-2 coefficients:
let my_bivector = Multivector::from_bivector(bivector_defn_here);
// A multivector with only grade-3 coefficients:
let my_trivector = Multivector::from_trivector(trivector_defn_here);
```

Alternately, the `Vector`, `Bivector`, and `Trivector` structs have a `to_multivector()` function:
```rust
let my_vector = Vector{ e0: 1.0, e1: 1.0, e2: 1.0}.to_multivector();
let my_bivector = Bivector{ e01: 1.0, e20: 1.0, e12: 1.0}.to_multivector();
let my_trivctor = Trivector{ e012: 1.0 }.to_multivector();
```

## Vectors
A vector is a a geometric object of grade 1 in projective geometric algebra. It is constructed from basis vectors e<sub>0</sub>, e<sub>1</sub>, and e<sub>2</sub>, where e<sub>0</sub><sup>2</sup> == 0.

In 2d PGA, a vector represents a **line**, with direction and magnitude.

For a line `ax+by+c = 0`, the corresponding vector will be `ae1+be2+ce0`.  In code, this will look like:
```rust
let example_line: Vector<f32> = Vector {
            e0: number_c,
            e1: number_a,
            e2: number_b
        };
```

## Bivectors
A bivector is a geometric object of grade 2.  In 2d projective geometric algebra, it is the point where intersecting lines *meet*.  That is to say: a bivector represents a 2d point. 

For a point `(x, y)`, the corresponding bivector will be `xe20+ye01+1e12`.  Note that the basis bivector e<sub>12</sub> is 1. When constructing a 2d point, the e<sub>12</sub> bivector should be normalized such that this coefficient is 1. In code, this will look like:
```rust
let example_bivector: Bivector<f32> = Bivector { 
            e01: x, 
            e20: y, 
            e12: 1.0 
        };
```

e<sub>12</sub> is a projective coordinate.  When it is '0', you will be representing a point at infinity, which is sometimes simply referred to as a 'direction'. 

The struct `extras::point2d::Point2d` exists as a wrapper around bivectors to make working with 2d eucludian points a bit more intuitive.

## Trivectors
The k-vector of grade 3 in 2d projective geometric algebra is called the 'pseudoscalar'.  It has a single coefficient.  Wrapping it was likely unnecessary, but I did so anyways to remain consistent.

Here is an example of a trivector:
```rust
let example_trivector: Trivector<f32> = Trivector { e012: 2.0 }
```

## The K-Vector struct
Admittedly, this part of the library ended up being more or less useless.  It was meant as a convenience feature, but most geometric objects are converted to multivectors during operations anyways.  It has been kept regardless incase anyone finds it interesting or useful.

`KVector<N>` is an **enum** which encapsulates every valid grade of k-vector in 2d PGA.


# Operators

# Extras
## 2d Points
...

## Rotors Motors and Transformers
...



