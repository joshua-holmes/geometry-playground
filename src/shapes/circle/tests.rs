use super::{Circle, super::SizeError};
use rand::Rng;

const PI: f64 = std::f64::consts::PI;

fn random_circ() -> Result<Circle, SizeError> {
    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(0.1..1_000_000.0);
    Circle::new(radius)
}

#[test]
fn is_circle() {
    if let Err(e) = random_circ() {
        panic!("Circle initialization with `new` method failed!\n{}", e);
    }
}

#[test]
fn test_diameter() {
    let circle = random_circ().unwrap();
    let diameter = circle.radius * 2.0;
    assert_f64_near!(circle.get_dia(), diameter);
}

#[test]
fn test_circumference() {
    let circle = random_circ().unwrap();
    let circumference = circle.radius * 2.0 * PI;
    assert_f64_near!(circle.get_cir(), circumference);
}

#[test]
fn test_area() {
    let circle = random_circ().unwrap();
    let area = circle.radius.powi(2) * PI;
    assert_f64_near!(circle.get_area(), area);
}

#[test]
fn test_radius_getter() {
    let my_circ = Circle::new(9.0).unwrap();
    assert_f64_near!(9.0, my_circ.get_rad());
}

#[test]
fn test_radius_setter() {
    let mut my_circ = Circle::new(9.0).unwrap();
    if let Err(e) = my_circ.set_rad(21.0) {
        panic!("Setter method returned an error! {}", e);
    }
    assert_f64_near!(21.0, my_circ.radius);
}
