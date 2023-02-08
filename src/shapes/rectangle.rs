use super::SizeError;


pub struct Rectangle {
    height: f64,
    width: f64
}
impl Rectangle {
    fn check_size(size: f64) -> i8 {
        if size > 1_000_000.0 {
            1
        } else if size <= 0.0 {
            -1
        } else {
            0
        }
    }

    pub fn build(height: f64, width: f64) -> Result<Self, SizeError> {
        let results: [i8; 2] = [Self::check_size(height), Self::check_size(width)];
        for result in results {
            match result {
                1 => return Err(SizeError::TooBig),
                -1 => return Err(SizeError::TooSmall),
                _ => {}
            }
        }
        Ok(Self { height, width })
    }

    pub fn build_square(side: f64) -> Result<Self, SizeError> {
        Self::build(side, side)
    }

    pub fn calc_area(&self) -> f64 {
        self.height * self.width
    }

    pub fn calc_perim(&self) -> f64 {
        (self.height + self.width) * 2.0
    }

    pub fn calc_diag_len(&self) -> f64 {
        (self.height.powi(2) + self.width.powi(2)).sqrt()
    }

    pub fn is_square(&self) -> bool {
        self.height == self.width
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn set_height(&mut self, new_height: f64) -> Result<(), SizeError> {
        let result = Self::check_size(new_height);
        match result {
            1 => Err(SizeError::TooBig),
            -1 => Err(SizeError::TooSmall),
            _ => {
                self.height = new_height;
                Ok(())
            }
        }
    }

    pub fn set_width(&mut self, new_width: f64) -> Result<(), SizeError> {
        let result = Self::check_size(new_width);
        match result {
            1 => Err(SizeError::TooBig),
            -1 => Err(SizeError::TooSmall),
            _ => {
                self.width = new_width;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests;