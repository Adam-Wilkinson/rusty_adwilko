extern crate rayon;

use std::fmt::Debug;

use crate::data_io::Savable;
use super::Domain;

use rayon::prelude::*;

pub struct OneDimensionalDomain {
    pub values : Vec<f64>
}

impl<TContext, TError> Savable<TContext, TError> for OneDimensionalDomain 
    where Vec<f64> : Savable<TContext, TError> {
    fn save(&self, file_path : &str, context : &TContext) -> Result<(), TError> {
        self.values.save(file_path, context)
    }
}

impl Domain for OneDimensionalDomain {
    type PointInDomain = f64;
    type Range<T : Debug> = Vec<T>;

    fn execute_map<TFunc,TParams,TOutput>(&self, function : TFunc, parameters : &TParams) -> Self::Range<TOutput>
            where TParams : Sized + Sync,
                TFunc : Fn(Self::PointInDomain , &TParams) -> TOutput + Sync,
                TOutput : Send + Debug, {
                self.values.par_iter().map(|x| function(*x, &parameters)).collect()
    }

    fn map_range<TFunc, TInput, TOutput>(input_range : &Self::Range<TInput>, function : TFunc) -> Self::Range<TOutput>
            where TFunc : FnMut(&TInput) -> TOutput, TInput : Debug, TOutput : Debug {
        input_range.iter().map(function).collect::<Vec<TOutput>>()
    }
}

impl OneDimensionalDomain {
    pub fn new(lower_limit : f64, upper_limit : f64, resolution : u32) -> OneDimensionalDomain{
        OneDimensionalDomain {
            values : (0..resolution).into_iter().map(|n| (n as f64) * (upper_limit - lower_limit) / (resolution as f64 - 1.0) + lower_limit).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OneDimensionalDomain;

    #[test]
    fn one_d_domain_has_correct_range() {
        assert_eq!(OneDimensionalDomain::new(1.0, 3.0, 3).values, vec![1.0, 2.0, 3.0])
    }
}