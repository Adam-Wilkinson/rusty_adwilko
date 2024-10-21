use std::f64::consts::PI;

use num::complex::{Complex64, ComplexFloat};

use crate::iterative_calculation;

/// Complete elliptic integral of the first kind (DLMF 19.2.8a)
pub fn k(x : Complex64) -> Complex64 {
    if x.abs() < 1.0 {
        let mut k_n = 1.0;
        // Handle the n=0 term separately, and then for the rest we modify k appropriately and add the next term
        return (PI / 2.0) * Complex64::new(1.0, 0.0) + iterative_calculation(0.001, 1, 1, |n| {
            k_n = k_n * ((2 * n - 1) as f64) / ((n * n * n) as f64);
            k_n * x.powi(2 * n)
        });
    }

    panic!();
}

#[cfg(test)]
mod tests {
    extern crate num;

    // use super::k;
    // use num::complex::{Complex64, ComplexFloat};

    // #[test]
    // fn complete_elliptic_k_five() {
    //     assert!((k(5.0.into()) - Complex64::new(0.7422062367111, 1.00945291)).abs() < 0.001);
    // }
}