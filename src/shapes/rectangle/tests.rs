use super::{Rectangle, super::SizeError};
use rand::Rng;

fn random_rect() -> Result<Rectangle, SizeError> {
    let mut rng = rand::thread_rng();
    let height = rng.gen_range(0.1..1_000_000.0);
    let width = rng.gen_range(0.1..1_000_000.0);
    Rectangle::new(height, width)
}

#[test]
fn is_rectangle() {
    if let Err(e) = random_rect() {
        panic!("Rectangle initialization with `new` method failed!\n{}", e);
    }
}

#[test]
fn is_square() {
    let my_rect = Rectangle::new_square(5.0).unwrap();
    assert!(my_rect.is_square());
}

#[test]
fn is_not_square() {
    let my_rect = Rectangle::new(9.0, 5.0).unwrap();
    assert!(!my_rect.is_square());
}

#[test]
fn test_area() {
    let my_rect = random_rect().unwrap();
    let area = my_rect.width * my_rect.height;
    assert_f64_near!(my_rect.get_area(), area);
}

#[test]
fn test_perimeter() {
    let my_rect = random_rect().unwrap();
    let perim = (my_rect.width + my_rect.height) * 2.0;
    assert_f64_near!(my_rect.get_perim(), perim);
}

#[test]
fn test_diag_length() {
    let my_rect = random_rect().unwrap();
    let diag = (my_rect.width.powi(2) + my_rect.height.powi(2)).sqrt();
    assert_f64_near!(my_rect.get_diag_len(), diag);
}

#[test]
fn test_width_getter() {
    let my_rect = Rectangle::new(9.0, 12.0).unwrap();
    assert_f64_near!(12.0, my_rect.get_width());
}

#[test]
fn test_height_getter() {
    let my_rect = Rectangle::new(9.0, 12.0).unwrap();
    assert_f64_near!(9.0, my_rect.get_height());
}

#[test]
fn test_width_setter() {
    let mut my_rect = Rectangle::new(9.0, 12.0).unwrap();
    if let Err(e) = my_rect.set_width(21.0) {
        panic!("Setter method returned an error! {}", e);
    }
    assert_f64_near!(21.0, my_rect.width);
}

#[test]
fn test_height_setter() {
    let mut my_rect = Rectangle::new(9.0, 12.0).unwrap();
    if let Err(e) = my_rect.set_height(41.0) {
        panic!("Setter method returned an error! {}", e);
    }
    assert_f64_near!(41.0, my_rect.height);
}