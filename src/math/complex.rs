use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

pub const ZERO: Complex = Complex { re: 0., im: 0. };

// Complex number with real and imaginary parts
#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    // The "size" of the complex number
    pub fn abs(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    // the "angle" of the complex number
    // todo benchmark different ways of computing it based on
    // https://en.wikipedia.org/wiki/Argument_(complex_analysis)
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    // raise the complex number to i-th power
    pub fn powi(&self, p: i32) -> Self {
        if p == 0 {
            return Complex { re: 1., im: 0. };
        }

        if p == 1 {
            return *self;
        }

        let r = self.abs().powi(p);
        let theta = self.arg() * (p as f64);
        Complex {
            re: f64::cos(theta) * r,
            im: f64::sin(theta) * r,
        }
    }
}

// Two complex numbers are considered equal if
// the magninute of their difference is less than delta
impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        // todo evaluate alternatives that doesn't use abs
        // abs uses square root, which is expensive
        (*self - *other).abs() < 0.001
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let k = rhs.re.powi(2) + rhs.im.powi(2);
        let re = (self.re * rhs.re + self.im * rhs.im) / k;
        let im = (self.im * rhs.re - self.re * rhs.im) / k;

        Self { re, im }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        return Self { re, im };
    }
}

impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{:.3} + {:.3}i", self.re, self.im)
        } else {
            write!(f, "{:.3} - {:.3}i", self.re, self.im.abs())
        }
    }
}
