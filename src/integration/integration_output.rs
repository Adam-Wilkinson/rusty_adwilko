use num::complex::Complex64;

#[derive(Clone, Copy, Debug)]
pub struct IntegrationOutput {
    pub num_function_evaluations: u32,
    pub error_estimate: f64,
    pub integral: Complex64,
}

impl IntegrationOutput {
    pub fn scale(self, c: f64) -> Self {
        IntegrationOutput {
            num_function_evaluations: self.num_function_evaluations,
            error_estimate: c * self.error_estimate,
            integral: c * self.integral,
        }
    }
}