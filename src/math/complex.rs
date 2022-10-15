use std::ops::{Add, Sub, Div};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64
}

impl Complex {
    pub fn abs(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn powf(&self, p: f64) -> Self {
        let r = self.abs().powf(p);
        let theta = p * self.arg();
        Complex {
            re: f64::cos(theta) * r,
            im: f64::sin(theta) * r
        }
    }

    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        ((self.re - other.re).powi(2) + (self.im - other.im).powi(2)).sqrt()
    }

    pub fn zero() -> Complex {
        Complex {
            re: 0.,
            im: 0.
        }
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        (*self - *other).abs() < 0.00001
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let k = rhs.re.powi(2) + rhs.im.powi(2);
        let re = (self.re * rhs.re + self.im * rhs.im) / k;
        let im = (self.im * rhs.re - self.re * rhs.im) / k;

        Self { re, im}
    }
}

impl Complex {
    pub fn multiply(self, f: f64) -> Self {
        Self {
            re: self.re * f,
            im: self.im * f
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i", self.re, self.im)
    }
}
