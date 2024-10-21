use std::fmt::Debug;

use crate::{domain::{CalculationResults, Domain}, with_error::WithError};

pub fn save<TData, TSaver, TError>(data : &TData, file_path : &str, saver : &TSaver) -> Result<(), TError>
    where TData : Savable<TSaver, TError>{
        data.save(file_path, saver)
}

pub trait Savable<TContext, TError> {
    fn save(&self, file_path : &str, context : &TContext) -> Result<(), TError>;
}

impl<TContext, TError> Savable<TContext, TError> for Vec<&Vec<f64>> 
    where Vec<Vec<f64>> : Savable<TContext, TError> {
        fn save(&self, file_path : &str, context : &TContext) -> Result<(), TError> {
            let borrowed = self.into_iter().map(|x| (*x).clone()).collect::<Vec<Vec<f64>>>();
            borrowed.save(file_path, context)
        }
}

impl<'a, TContext, TError, TDomain, TValue, const N : usize> Savable<TContext, TError> for CalculationResults<'a, TDomain, WithError<TValue>, N>
    where TDomain : Domain,
          TValue : Copy + Debug,
          CalculationResults<'a, TDomain, TValue, N> : Savable<TContext, TError>,
          CalculationResults<'a, TDomain, f64, N> : Savable<TContext, TError>
{
    fn save(&self, file_path : &str, context : &TContext) -> Result<(), TError> {
        self.map_output(|with_error| with_error.value).save(file_path, context)?;
        self.map_output(|with_error| with_error.error).save(&(file_path.to_owned() + " - ERROR"), context)
    }
}