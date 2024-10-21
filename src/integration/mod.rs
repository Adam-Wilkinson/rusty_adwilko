mod trapezium;
mod double_exponential;
pub mod integration_output;

pub use self::trapezium::integrate as trapezium;
pub use self::double_exponential::integrate as double_exponential;