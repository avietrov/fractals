use crate::math::complex::{Complex, ZERO};
use std::fmt;

// represents a polynomial
// the i-th coefficient is the coefficiet in front of x^i
#[derive(Debug, Clone)]
pub struct Polynomial {
    coeff: Vec<i32>,
}

impl Polynomial {
    pub fn new(coeff: Vec<i32>) -> Self {
        Polynomial { coeff }
    }

    // derivates coefficient
    pub fn derivative(&self) -> Polynomial {
        if self.coeff.len() <= 1 {
            return Polynomial::new(vec![]);
        }

        let new_coef = self
            .coeff
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, coef)| coef * (index as i32))
            .collect();

        Polynomial { coeff: new_coef }
    }

    // returns the value of coefficient at a given point
    // todo clean up type casts
    pub fn evaluate(&self, z: &Complex) -> Complex {
        let mut acc = ZERO;
        for i in 0..self.coeff.len() {
            let coef = self.coeff[i];
            if coef == 0 {
                continue;
            }
            let p = z.powi(i as i32);
            let pm = p * (coef as f64);
            acc = acc + pm
        }
        acc
    }
}

fn show_part(index: usize, coef: i32) -> String {
    let s_coef = if coef == 1 {
        String::from("")
    } else {
        format!("{}", coef)
    };
    match index {
        0 => format!("{}", coef),
        1 => format!("{}x", s_coef),
        _ => format!("{}x^{}", s_coef, index),
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = self
            .coeff
            .iter()
            .enumerate()
            .filter(|(_, coef)| **coef > 0)
            .map(|(index, coef)| show_part(index, *coef))
            .reduce(|a, b| a + " + " + &b)
            .unwrap_or_else(|| String::from(""));

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivate_constant() {
        // y = 1
        // y = 1*x^0
        let pol = Polynomial::new(vec![1]);
        assert_eq!(true, pol.derivative().coeff.is_empty());
    }

    #[test]
    fn derivate_linear() {
        // y = x
        // y = 0*x^0 + 1*x^1
        let pol = Polynomial::new(vec![0, 1]);
        assert_eq!(vec![1], pol.derivative().coeff);
    }

    #[test]
    fn derivate_pol2() {
        // y = x ^ 2
        // y = 0*x^0 + 0*x^1 + 1*x^2
        let pol = Polynomial::new(vec![0, 0, 1]);
        assert_eq!(vec![0, 2], pol.derivative().coeff);
    }

    #[test]
    fn derivate_pol5() {
        let pol = Polynomial::new(vec![1, 1, 2, 3, 5, 8]);
        // y = 1 + x + 2x^2 + 3x^3 + 5x^4 + 8x^5
        // y' = 1 + 4x + 9x^2 + 20x^4 + 40x^5
        assert_eq!(vec![1, 4, 9, 20, 40], pol.derivative().coeff);
    }

    #[test]
    fn display_constant() {
        // y = 1
        let pol = Polynomial::new(vec![1]);
        assert_eq!("1", format!("{}", pol));
    }

    #[test]
    fn display_linear() {
        // y = x
        let pol = Polynomial::new(vec![0, 1]);
        assert_eq!("x", format!("{}", pol));
    }

    #[test]
    fn display_pol2() {
        // y = x ^ 2
        let pol = Polynomial::new(vec![0, 0, 1]);
        assert_eq!("x^2", format!("{}", pol));
    }

    #[test]
    fn display_pol5() {
        let pol = Polynomial::new(vec![1, 1, 2, 3, 5, 8]);
        // y = 1 + x + 2x^2 + 3x^3 + 5x^4 + 8x^5
        assert_eq!("1 + x + 2x^2 + 3x^3 + 5x^4 + 8x^5", format!("{}", pol));
    }

    #[test]
    fn evalute_constant() {
        let point = Complex { re: 1., im: 1. };
        let pol = Polynomial::new(vec![3]); // y = 3
        let actual = pol.evaluate(&point);
        assert_eq!(Complex { re: 3., im: 0. }, actual)
    }

    #[test]
    fn evalute_linear() {
        let point = Complex { re: 3., im: 2. };
        let pol = Polynomial::new(vec![0, 1]); // y = x
        let actual = pol.evaluate(&point);
        assert_eq!(point, actual)
    }

    #[test]
    fn evaluate_pol3() {
        let point = Complex { re: 4., im: 2. };
        let pol = Polynomial::new(vec![0, 1, -2, 1]); // 3
        let actual = pol.evaluate(&point);
        assert_eq!(Complex { re: -4., im: 58. }, actual)
    }

    #[test]
    fn evaluate_pol3_0() {
        let point = Complex { re: 0., im: 0. };
        let pol = Polynomial::new(vec![0, 1, -2, 1]); // y = x^3 - 2x^2 + x
        let actual = pol.evaluate(&point);
        assert_eq!(Complex { re: 0., im: 0. }, actual)
    }

    #[test]
    fn evaluate_pol3_neg() {
        let point = Complex { re: -7.5, im: -5.2 };
        let pol = Polynomial::new(vec![0, 1, -2, 1]); // y = x^3 - 2x^2 + x
        let actual = pol.evaluate(&point);
        assert_eq!(
            Complex {
                re: 120.605,
                im: -898.092
            },
            actual
        )
    }
}
