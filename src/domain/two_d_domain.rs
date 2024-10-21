extern crate rayon;
extern crate ndarray;

use std::fmt::Debug;

use crate::data_io::Savable;
use super::Domain;
use ndarray::Array2;
use ndarray::Zip;

pub struct TwoDimensionalDomain {
    pub resolution : usize,
    pub x_limits : (f64, f64),
    pub y_limits : (f64, f64),
    pub x_values : Vec<f64>,
    pub y_values : Vec<f64>,
}

impl<TContext, TError> Savable<TContext, TError> for TwoDimensionalDomain 
    where Vec<Vec<f64>> : Savable<TContext, TError> {
    fn save(&self, file_path : &str, context : &TContext) -> Result<(), TError> {
        vec![self.x_values.clone(), self.y_values.clone()].save(file_path, context)
    }
}

impl Domain for TwoDimensionalDomain {
    type PointInDomain = (f64, f64);
    type Range<T : Debug> = Array2<T>;

    fn execute_map<TFunc,TParams,TOutput>(&self, function : TFunc, parameters : &TParams) -> Self::Range<TOutput>
            where 
                TParams : Sized + Sync, 
                TFunc : Fn(Self::PointInDomain , &TParams) -> TOutput + Sync,
                TOutput : Send + Debug {
        let mut x_values = Array2::zeros((self.resolution, self.resolution));
        x_values.indexed_iter_mut().for_each(|((_, i), x)| *x = self.x_values[i]);

        let mut y_values = Array2::zeros((self.resolution, self.resolution));
        y_values.indexed_iter_mut().for_each(|((j, _), x)| *x = self.y_values[j]);

        Zip::from(&mut x_values).and(&y_values).par_map_collect(|x, y| {
            function((*x, *y), &parameters)
        })
    }

    fn map_range<TFunc, TInput, TOutput>(input_range : &Self::Range<TInput>, function : TFunc) -> Self::Range<TOutput>
            where TFunc : FnMut(&TInput) -> TOutput, TInput : Debug, TOutput : Debug {
        Array2::map(input_range, function)
    }
}

impl TwoDimensionalDomain {
    pub fn new(x_limits : (f64, f64), y_limits : (f64, f64), resolution : usize) -> TwoDimensionalDomain{
        TwoDimensionalDomain {
            resolution,
            x_limits,
            y_limits,
            x_values : (0..resolution).into_iter().map(|n| (n as f64) * (x_limits.1 - x_limits.0) / (resolution as f64 - 1.0) + x_limits.0).collect(),
            y_values : (0..resolution).into_iter().map(|n| (n as f64) * (y_limits.1 - y_limits.0) / (resolution as f64 - 1.0) + y_limits.0).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TwoDimensionalDomain;

    #[test]
    fn one_d_domain_has_correct_range() {
        let test_domain = TwoDimensionalDomain::new((1.0, 3.0), (4.0, 6.0), 3);
        assert_eq!(test_domain.x_values, vec![1.0, 2.0, 3.0]);
        assert_eq!(test_domain.y_values, vec![4.0, 5.0, 6.0]);
    }
}