extern crate num;
extern crate param_struct;

mod data_io;
mod calculators;
pub mod integration;
pub mod special_functions;
pub mod domain;
pub mod with_error;

use std::fmt::Debug;
use domain::Domain;
pub use param_struct::param_struct;
pub use data_io::{Savable, Plot, PRESENTATION_STYLE, PAPER_STYLE, LIGHT, DARK, Numpy, save};
pub use calculators::iterative_calculation;

pub const EM_GAMMA : f64 = 0.577215664901532860606512090082402431_f64;

pub fn save_in_domain<TFunc, TDomain, TParams, TOutput, TSaver, TError>(function : TFunc, domain : TDomain, parameter_values : &[((f64, &str), TParams)], path : &str, format : &TSaver) -> Result<(), TError> where 
    TDomain : Domain + Savable<TSaver, TError>,
    <TDomain as Domain>::Range<TOutput> : Savable<TSaver, TError>,
    TFunc : Fn(<TDomain as Domain>::PointInDomain, &TParams) -> TOutput + Sync,
    TOutput : Send + Debug,
    TParams : Sync
{
    save(&domain, &(path.to_string() + "/Domain"), format)?;

    for parameter_value in parameter_values {
        save(
            &domain.execute_map(&function, &parameter_value.1), 
            &(path.to_string() + "/" + parameter_value.0.1 + " = " + &parameter_value.0.0.to_string()), 
            format
        )?;
    }

    Ok(())
}