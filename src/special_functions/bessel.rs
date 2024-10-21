use std::f64::consts::PI;

extern crate libm;

const PRECISION : f64 = 0.008;
const PRECISION_EIGHTH_ROOT : f64 = 0.54687;

/// The Bessel function of the first kind (DLMF 10.2.2)
pub fn jn(argument : f64, order : i32) -> f64 {
    let mut factor = 1.0;
    let positive_argument = argument.abs();

    if argument < 0.0 {
        if order % 2 == 0 {
            factor = 1.0;
        }
        else {
            factor = -1.0;
        };
    };

    match order {
        0 => j0_positive(positive_argument) * factor,
        1 => j1_positive(positive_argument) * factor,
        o if (o as f64) < positive_argument * PRECISION_EIGHTH_ROOT => large_argument_expansion(o, positive_argument),
        o if (o as f64) < positive_argument => bessel_forward_recurrence(o, positive_argument),
        o if positive_argument < 10.0 || 0.1 * positive_argument * positive_argument / 4.0 < (o as f64) => small_argument_expansion(o, positive_argument),
        o => backwards_recurrence(o, positive_argument)
    }
}

fn j0_positive(argument : f64) -> f64 {
    // #[cfg(test)]
    // println!("J0");
    if argument == 0.0 {
        return 1.0;
    };


    libm::j0(argument)
}

fn j1_positive(argument : f64) -> f64 {
    // #[cfg(test)]
    // println!("J1");
    if argument == 0.0 {
        return 0.0;
    }


    libm::j1(argument)
}

// MmcMahon
// Large argument expansion as used in Florentin, J. J., Milton Abramowitz and Irene A. Stegun. �Handbook of Mathematical Functions.� American Mathematical Monthly 73 (1966): 1143. 9.2.28 & 9.2.29
fn large_argument_expansion(order : i32, argument : f64) -> f64 {
    // #[cfg(test)]
    // println!("Large argument");

    let mu = (4 * order * order) as f64;

    let argument_cubed = argument * argument * argument;
    let argument_to_five = argument_cubed * argument * argument;
    let argument_to_seven = argument_to_five * argument * argument;

    let large_expansion_denom = 4.0 * argument * argument;
    let large_expansion_denom_squared = large_expansion_denom * large_expansion_denom;
    let large_expansion_denom_cubed = large_expansion_denom_squared * large_expansion_denom;

    let prefactor = (2.0 / (PI * argument)).sqrt();

    prefactor * f64::sqrt(
        1.0
        + 0.5 * (mu - 1.0) / large_expansion_denom
        + 0.375 * (mu - 1.0) * (mu - 9.0) / large_expansion_denom_squared
        + 0.3125 * (mu - 1.0) * (mu - 9.0) * (mu - 25.0) / large_expansion_denom_cubed
    )
    * f64::cos(
        argument
        - (0.5 * (order as f64) + 0.25) * PI
        + (mu - 1.0) / (8.0 * argument)
        + (mu - 1.0) * (mu - 25.0) / (384.0 * argument_cubed)
        + (mu - 1.0) * (mu * mu - 144.0 * mu + 1073.0) / (5120.0 * argument_to_five)
        + (mu - 1.0) * (5.0 * mu * mu * mu - 1535.0 * mu * mu + 54703.0 * mu - 375733.0) / (229376.0 * argument_to_seven)
    )
}

fn bessel_forward_recurrence(order : i32, argument : f64) -> f64 {
    // #[cfg(test)]
    // println!("Forward recurrence");

    let fac = 2.0/argument;
    let mut prev = j0_positive(argument);
    let mut curr = j1_positive(argument);
    let mut value : f64;
    for i in 1..order {
        value = (i as f64) * fac * curr - prev;
        prev = curr;
        curr = value;
    }

    curr
}

// DLMF 10.2.2
fn small_argument_expansion(order : i32, argument : f64) -> f64 {
    // #[cfg(test)]
    // println!("Small argument");
    let minus_argument_over_two_squared = -argument * argument / 4.0;
    let mut successes_in_a_row = 0;
    let mut sum_result = 1.0;
    let mut current_sum_addition = 1.0;
    let mut k = 1;
    while successes_in_a_row < 2  {
        current_sum_addition *= minus_argument_over_two_squared / ((k * (k + order)) as f64);
        sum_result += current_sum_addition;
        k += 1;
        successes_in_a_row = (successes_in_a_row + 1) * (((current_sum_addition / sum_result).abs() < PRECISION) as i32);
    };

    // order! won't overflow
    if order < 171 {
        sum_result * (argument / 2.0).powi(order) / libm::tgamma(order as f64 + 1.0)
    }
    else {
        sum_result * f64::exp(order as f64 * (argument / 2.0).ln() - libm::lgamma(order as f64 + 1.0))
    }
}

fn backwards_recurrence(order : i32, argument : f64) -> f64 {
    // #[cfg(test)]
    // println!("Backwards recurrence");

    libm::jn(order, argument)
}

#[cfg(test)]
mod tests {
    extern crate paste;

    macro_rules! grid_bessel_tests {
        ($argumentList:tt; [$($order:literal),*]) => {
            $(
                bessel_tests_multiple_order!($order, $argumentList);
            )*
        };
    }

    macro_rules! bessel_tests_multiple_order {
        ($order:literal, [$($argument:expr),*]) => {
            $(
                paste::item! { 
                    #[test]
                    fn [< order_ $order _argument_$argument >]() {
                        let my_result = super::jn($argument as f64, $order);
                        let their_result = libm::jn($order, $argument as f64);
                        println!("I got {my_result}, but libm got {their_result}");
                        assert!((my_result - their_result).abs() < 0.02);
                    }

                    #[test]
                    fn [< order_ $order _argument_minus_$argument >]() {
                        let my_result = super::jn(-$argument as f64, $order);
                        let their_result = libm::jn($order, -$argument as f64);
                        println!("I got {my_result}, but libm got {their_result}");
                        assert!((my_result - their_result).abs() < 0.02);
                    }
                }
            )*
        };
    }

    grid_bessel_tests!{ [25e3, 2e-5, 5, 10, 10e-3, 6e1, 2e-4]; [2, 10, 400, 7000, 40000, 3000000] }
}