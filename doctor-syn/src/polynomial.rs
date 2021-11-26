//! Polynomial approximation for efficient function generation.

use crate::bdmath::*;

/// A polynomial
pub struct Polynomial {
    terms: Vec<BigDecimal>,
}

/// Find the first row of the divided differences.
pub(crate) fn divided_differences(x: &[BigDecimal], y: &mut [BigDecimal], num_digits: i64) {
    let k = y.len();
    for i in 0..k - 1 {
        for j in (i..k - 1).rev() {
            y[j + 1] = round((&y[j + 1] - &y[j]) / (&x[j + 1] - &x[j - i]), num_digits);
        }
    }
}

#[allow(dead_code)]
impl Polynomial {
    pub fn from_points(x: &[BigDecimal], y: &[BigDecimal], num_digits: i64) -> Self {
        // https://en.wikipedia.org/wiki/Newton_polynomial

        let k = y.len();
        let mut dd = Vec::from(y);
        let mut newton = vec![zero(); k];
        let mut terms = vec![zero(); k];
        newton[0] = one();

        divided_differences(x, &mut *dd, num_digits);

        let k = y.len();
        for i in 0..k {
            for j in 0..k {
                terms[j] += round(&newton[j] * &dd[i], num_digits);
            }

            // Multiply "newton" by (x - x[i])
            let c = -&x[i];
            for i in (1..k).rev() {
                newton[i] = round(&newton[i] * &c + &newton[i - 1], num_digits);
            }
            newton[0] *= c;
        }
        Self { terms }
    }

    pub fn eval(&self, x: BigDecimal) -> BigDecimal {
        let l = self.terms.len();
        let mut y = self.terms[l - 1].clone();
        for i in (0..l - 1).rev() {
            y = y * &x + &self.terms[i];
        }
        y
    }

    pub fn terms(&self) -> &[BigDecimal] {
        &*self.terms
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // #[test]
    // fn test_dd() {
    //     let x = &[-3.0/2., -3.0/4., 0., 3.0/4., 3.0/2.];
    //     let y = &mut [-14.1014, -0.931596, 0., 0.931596, 14.1014];

    //     divided_differences(x, y);
    // }

    #[test]
    fn test_poly() {
        // test the two examples from Wikipedia.
        let x = &[bigd(1), bigd(2), bigd(3), bigd(4)];
        let y = &mut [bigd(6), bigd(9), bigd(2), bigd(5)];

        let p = Polynomial::from_points(x, y, 30);
        let y2 = (0..x.len())
            .map(|i| p.eval(x[i].clone()))
            .collect::<Vec<_>>();

        let err = (0..x.len())
            .map(|i| (&y2[i] - &y[i]).abs())
            .collect::<Vec<_>>();
        assert!(!err.iter().any(|e| e > &bigdf(0.00001)));

        // let x = &[-3.0 / 2., -3.0 / 4., 0., 3.0 / 4., 3.0 / 2.];
        // let y = &mut [-14.1014, -0.931596, 0., 0.931596, 14.1014];

        // let p = Polynomial::from_points(x, y);
        // let y2 = (0..x.len()).map(|i| p.eval(x[i])).collect::<Vec<_>>();

        // let err = (0..x.len())
        //     .map(|i| (y2[i] - y[i]).abs())
        //     .collect::<Vec<_>>();
        // assert!(!err.iter().any(|&e| e > 0.00001));
    }
}
