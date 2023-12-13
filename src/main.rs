use simply_2dpga::{vectors::{vector::Vector, bivector::Bivector}, traits::{Contraction, RegressiveProduct}, extras::{point2d::{Point2d, self}, transformations::{Rotor, Transformation}, angle::Angle, transformations, transformations::Motor}};

fn main() {
    // sandbox for testing purposes.

    let point1 = Point2d::new(2.0f32, 0.0); // axis
    let point2 = Point2d::new(2.0f32, 4.0); // point i want to rotate.

    // let angle: Angle<f32> = Angle::from_radians(3.1415926535);
    let angle: Angle<f32> = Angle::from_degrees(45.0f32);

    let motor = Motor::new(1.0f32, 2.0, 2.0);
    let rotor = Rotor::new(&point1, &angle);

    let speen = rotor.apply(&point2.to_bivector());
    let poosh = motor.apply(&point2.to_bivector());
    
    println!("rotated = {} | {}", speen, Point2d::from_bivector(&speen));
    println!("displaced = {} | {}", poosh, Point2d::from_bivector(&poosh));

    // println!("result:\n\t{}", Point2d::from_bivector(&result));
}