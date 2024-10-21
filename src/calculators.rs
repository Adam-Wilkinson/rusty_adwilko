use std::ops::Div;
use num::{complex::ComplexFloat, Complex, Float, NumCast, Zero};

pub fn iterative_calculation<TAbs, TValue, TFunc>(error_tolerence : TAbs, n_0 : i32, delta_n : i32, mut term_function : TFunc) -> TValue
    where TAbs : Float, 
        TValue : NormSquared<TAbs> + Zero + Div<TValue, Output=TValue> + Copy,
        TFunc : FnMut(i32) -> TValue
{
    let error_tolerance_squared = error_tolerence * error_tolerence;
    let mut current_value = TValue::zero();
    let mut most_recent_term;
    let mut successes_in_a_row = 0;
    let mut n = n_0;
    while successes_in_a_row < 4 {
        most_recent_term = term_function(n);
        current_value = current_value + most_recent_term;
        successes_in_a_row = (successes_in_a_row + 1) * (((most_recent_term / current_value).norm_squared() < error_tolerance_squared) as i32);
        n += delta_n;
    }

    current_value
}

pub fn polynomial_ratio<const N : usize, TNum, TCoefficient>(numerator_coefficients : &[TCoefficient; N], denominator_coefficients : &[TCoefficient; N], x : TNum) -> (TNum, TNum)
    where TNum : ComplexFloat,
          TCoefficient : Copy + NumCast {
        let (numerator, denominator, highest_power) = numerator_coefficients.iter()
            .zip(denominator_coefficients)
            .skip(1)
            .fold((TNum::one() * NumCast::from(numerator_coefficients[0]).unwrap(), TNum::one() * NumCast::from(denominator_coefficients[0]).unwrap(), TNum::one()), 
                |(cumulative_numerator, cumulative_denominator, current_x_power), (current_numerator, current_denominator)| {
                let new_x_power = current_x_power * x;
                (
                    cumulative_numerator + new_x_power * NumCast::from(*current_numerator).unwrap(),
                    cumulative_denominator + new_x_power * NumCast::from(*current_denominator).unwrap(),
                    new_x_power
                )
            });

        return (numerator / denominator, highest_power);
}

pub trait NormSquared<TAbs : Float> {
    fn norm_squared(&self) -> TAbs;
}

impl<T : Float> NormSquared<T> for Complex<T> {
    fn norm_squared(&self) -> T {
        self.norm_sqr()
    }
}

impl<T : Float> NormSquared<T> for T {
    fn norm_squared(&self) -> T {
        self.clone() * self.clone()
    }
}