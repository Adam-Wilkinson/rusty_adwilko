use num::complex::{Complex64, ComplexFloat};

use super::integration_output::IntegrationOutput;

pub fn integrate<F>(integrand : F, lower_limit : f64, upper_limit : f64, target_error : f64) -> IntegrationOutput 
    where F : Fn(f64) -> Complex64
{

    let mut current_estimate = Complex64::new(1.0, 1.0);
    let mut previous_estimate = Complex64::new(0.0, 0.0);

    let first_and_last_terms = (integrand(lower_limit) + integrand(upper_limit)) / 2.0;
    let mut current_sum_of_internal_terms = Complex64::new(0.0, 0.0);
    let mut number_of_subdivisions = 2;
    let mut error = 1.0;

    while (((1.0 - (previous_estimate / current_estimate).abs()).abs() > target_error) && (current_estimate.abs() > 1e-8)) || (number_of_subdivisions < 16) {
        error = (1.0 - (previous_estimate / current_estimate).abs()).abs();
        if number_of_subdivisions >= (2 as u32).pow(16 as u32) {
            println!("Large error produced: {error}");
            break;
        }

        previous_estimate = current_estimate;
        for n in 1..=number_of_subdivisions / 2 {
            current_sum_of_internal_terms += integrand((upper_limit - lower_limit) * ((((n * 2) - 1) as f64 / number_of_subdivisions as f64)));
        }
        current_estimate = (first_and_last_terms + current_sum_of_internal_terms) * (upper_limit - lower_limit) / (number_of_subdivisions as f64);
        number_of_subdivisions *= 2;
    }

    return IntegrationOutput {
        num_function_evaluations : number_of_subdivisions,
        error_estimate : error,
        integral : current_estimate
    }
}

#[cfg(test)]
mod tests {
    use num::complex::{Complex64, ComplexFloat};
    use super::integrate;
    use std::f64::consts::PI;

    #[test]
    fn sine_x() {
        let result = integrate(|x| Complex64::new(x.sin(), 0.0), 0.0, 2.0 * PI, 0.01);
        assert!(result.integral.abs() < 0.1);
    }

    #[test]
    fn x_squared() {
        assert!(integrate(|x| Complex64::new(x.powi(2), 0.0), 0.0, 3.0, 0.01).integral.abs() - 9.0 < 0.05);
    }
}