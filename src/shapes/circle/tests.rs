use super::{Circle, super::SizeError};
use rand::Rng;
use round;

fn random_circ() -> Result<Circle, SizeError> {
    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(0.1..1_000_000.0);
    Circle::build(radius)
}

#[test]
fn is_circle() {
    if let Err(e) = random_circ() {
        panic!("Circle initialization with `build` method failed!\n{}", e);
    }
}

#[test]
fn is_not_circle_negative_radius() {
    let my_circ = Circle::build(-10.0);
    if my_circ.is_ok() {
        panic!("Circle with a negative radius should not be accepted!");
    }
}

#[test]
fn is_not_circle_radius_too_big() {
    let my_circ = Circle::build(1_000_001.0);
    if my_circ.is_ok() {
        panic!("Circle with a negative radius should not be accepted!");
    }
}

#[test]
fn diameter_calculation() {
    let circle = Circle::build(10.0).unwrap();
    assert_f64_near!(20.0, circle.calc_dia());
}

#[test]
fn circumference_calculation() {
    let circle = Circle::build(5.0).unwrap();
    assert_f64_near!(31.42, round::round(circle.calc_cir(), 2));
}

#[test]
fn area_calculation() {
    let circle = Circle::build(10.0).unwrap();
    assert_f64_near!(314.16, round::round(circle.calc_area(), 2));
}

#[test]
fn test_radius_getter() {
    let my_circ = Circle::build(9.0).unwrap();
    assert_f64_near!(9.0, my_circ.get_rad());
}

#[test]
fn test_radius_setter() {
    let mut my_circ = Circle::build(9.0).unwrap();
    if let Err(e) = my_circ.set_rad(21.0) {
        panic!("Setter method returned an error! {}", e);
    }
    assert_f64_near!(21.0, my_circ.radius);
}
