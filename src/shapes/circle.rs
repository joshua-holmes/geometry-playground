use super::SizeError;

const PI: f64 = std::f64::consts::PI;

pub struct Circle {
    radius: f64
}
impl Circle {
    fn check_size(size: f64) -> i8 {
        if size > 1_000_000.0 {
            1
        } else if size <= 0.0 {
            -1
        } else {
            0
        }
    }

    pub fn new(radius: f64) -> Result<Self, SizeError> {
        let result = Self::check_size(radius);
        match result {
            1 => Err(SizeError::TooBig),
            -1 => Err(SizeError::TooSmall),
            _ => Ok(Self { radius })
        }
    }

    pub fn get_cir(&self) -> f64 {
        2.0 * PI * self.radius
    }

    pub fn get_area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    pub fn get_dia(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn get_rad(&self) -> f64 {
        self.radius
    }

    pub fn set_rad(&mut self, new_radius: f64) -> Result<(), SizeError> {
        let result = Self::check_size(new_radius);
        match result {
            1 => Err(SizeError::TooBig),
            -1 => Err(SizeError::TooSmall),
            _ => {
                self.radius = new_radius;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests;