use num::complex::Complex64;
use std::f64::consts::{PI,TAU};

const G : usize = 7;

const G_PLUS_HALF : f64 = G as f64 + 0.5;

const P: [f64 ;G+2] = [
    0.99999999999980993,
    676.5203681218851,
    -1259.1392167224028,
    771.32342877765313,
    -176.61502916214059,
    12.507343278686905,
    -0.13857109526572012,
    9.9843695780195716e-6,
    1.5056327351493116e-7
];

pub fn gamma(input : Complex64) -> Complex64{
    if input.re < 0.5 {
        return PI / (gamma(1.0 - input) * (PI * input).sin());
    }

    let z = input - 1.0;
    let t = z + G_PLUS_HALF;
    let x = P[0] + P.iter().skip(1).enumerate().map(|(i, x)| x / (z + i as f64)).sum::<Complex64>();
    return TAU.sqrt() * t.powc(z + 0.5) * (-t).exp() * x;
}

#[cfg(test)]
mod tests {
    use crate::special_functions::gamma;
    use libm::tgamma;
    use num::complex::Complex64;

    #[test]
    fn gamma_test_20() {
        // assert_eq!(gamma(Complex64::from(20.0)), Complex64::from(tgamma(20.0)));
    }
}