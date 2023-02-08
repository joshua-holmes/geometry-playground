use super::{Rectangle, super::SizeError};
use rand::Rng;
use round;

fn random_rect() -> Result<Rectangle, SizeError> {
    let mut rng = rand::thread_rng();
    let height = rng.gen_range(0.1..1_000_000.0);
    let width = rng.gen_range(0.1..1_000_000.0);
    Rectangle::build(height, width)
}

#[test]
fn is_rectangle() {
    if let Err(e) = random_rect() {
        panic!("Rectangle initialization with `build` method failed!\n{}", e);
    }
}

#[test]
fn rectangle_too_tall() {
    let my_rect = Rectangle::build(1_000_001.0, 10.0);
    if my_rect.is_ok() {
        panic!("Rectangle should not be accepted if it's too tall!");
    }
}

#[test]
fn rectangle_too_wide() {
    let my_rect = Rectangle::build(10.0, 1_000_001.0);
    if my_rect.is_ok() {
        panic!("Rectangle should not be accepted if it's too wide!");
    }
}

#[test]
fn square_has_height_of_5() {
    let my_rect = Rectangle::build_square(5.0).unwrap();
    assert_f64_near!(5.0, my_rect.height);
}

#[test]
fn square_has_width_of_5() {
    let my_rect = Rectangle::build_square(5.0).unwrap();
    assert_f64_near!(5.0, my_rect.width);
}

#[test]
fn is_square() {
    let my_rect = Rectangle::build(5.0, 5.0).unwrap();
    assert!(my_rect.is_square());
}

#[test]
fn is_not_square() {
    let my_rect = Rectangle::build(9.0, 5.0).unwrap();
    assert!(!my_rect.is_square());
}

#[test]
fn area_calculation() {
    let my_rect = Rectangle::build(6.0, 5.0).unwrap();
    assert_f64_near!(30.0, my_rect.calc_area());
}

#[test]
fn perimeter_calculation() {
    let my_rect = Rectangle::build(6.0, 5.0).unwrap();
    assert_f64_near!(22.0, my_rect.calc_perim());
}

#[test]
fn diag_length_calculation() {
    let my_rect = Rectangle::build(6.0, 5.0).unwrap();
    assert_f64_near!(7.81, round::round(my_rect.calc_diag_len(), 2));
}

#[test]
fn test_width_getter() {
    let my_rect = Rectangle::build(9.0, 12.0).unwrap();
    assert_f64_near!(12.0, my_rect.get_width());
}

#[test]
fn test_height_getter() {
    let my_rect = Rectangle::build(9.0, 12.0).unwrap();
    assert_f64_near!(9.0, my_rect.get_height());
}

#[test]
fn test_width_setter() {
    let mut my_rect = Rectangle::build(9.0, 12.0).unwrap();
    if let Err(e) = my_rect.set_width(21.0) {
        panic!("Setter method returned an error! {}", e);
    }
    assert_f64_near!(21.0, my_rect.width);
}

#[test]
fn test_height_setter() {
    let mut my_rect = Rectangle::build(9.0, 12.0).unwrap();
    if let Err(e) = my_rect.set_height(41.0) {
        panic!("Setter method returned an error! {}", e);
    }
    assert_f64_near!(41.0, my_rect.height);
}