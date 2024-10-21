mod one_d_domain;
mod two_d_domain;

use std::time::Instant;
use std::fmt::Debug;
pub use one_d_domain::OneDimensionalDomain;
pub use two_d_domain::TwoDimensionalDomain;

pub trait Domain {
    type PointInDomain;
    type Range<T : Debug> : Debug;

    fn execute_multimap<'a, TFunc, TParams, TOutput, TParamSpec, const N : usize>(&'a self, function : TFunc, parameter_specifications : TParamSpec) -> CalculationResults<'a, Self, TOutput, N> 
        where
            TParamSpec : Into<ParameterSpecification<TParams, N>>, 
            TParams : Sized + Sync,
            TFunc : Fn(Self::PointInDomain, &TParams) -> TOutput + Sync,
            TOutput : Send + Debug,
    {
        let params : [(String, TParams); N] = parameter_specifications.into().parameter_values;

        let results = params.iter()
            .map(|x| {
                let time_before = Instant::now();
                let result = self.execute_map(&function, &x.1);
                println!("Completed calculation '{}' in {:.2?}", &x.0, time_before.elapsed());
                return result;
            })
            .collect::<Vec<Self::Range<TOutput>>>().try_into().unwrap();
        let result_names = params.into_iter().map(|x| x.0).collect::<Vec<String>>().try_into().unwrap();

        return CalculationResults {
            domain_data : &self,
            results,
            result_names,
        }
    }

    fn map_range<TFunc, TInput : Debug, TOutput : Debug>(input_range : &Self::Range<TInput>, function : TFunc) -> Self::Range<TOutput>
        where TFunc : FnMut(&TInput) -> TOutput;

    fn execute_map<TFunc,TParams,TOutput : Debug>(&self, function : TFunc, parameters : &TParams) -> Self::Range<TOutput>
        where TParams : Sized + Sync, 
            TFunc : Fn(Self::PointInDomain , &TParams) -> TOutput + Sync,
            TOutput : Send;
}

impl<T> From<T> for ParameterSpecification<T, 1> {
    fn from(value: T) -> Self {
        ParameterSpecification::<T, 1> {
            parameter_values : [("".to_owned(), value)]
        }
    }
}

impl<T, const N : usize> From<[(String, T); N]> for ParameterSpecification<T, N> {
    fn from(value: [(String, T); N]) -> Self {
        ParameterSpecification::<T, N> {
            parameter_values : value
        }
    }
}

pub struct ParameterSpecification<TParams, const N : usize> {
    pub parameter_values : [(String, TParams); N]
}

pub struct CalculationResults<'a, TDomain, TOutput, const N : usize>
    where TDomain : Domain + ?Sized, TOutput : Debug {
        pub domain_data : &'a TDomain,
        pub results : [<TDomain as Domain>::Range<TOutput>; N],
        pub result_names : [String; N]
}

impl<'a, TDomain, TOutput, const N : usize> CalculationResults<'a, TDomain, TOutput, N>
    where TDomain : Domain, TOutput : Debug {
    pub fn map_output<TFunc, TNewOutput>(&self, function : TFunc) -> CalculationResults<'a, TDomain, TNewOutput, N> 
        where TFunc : Fn(&TOutput) -> TNewOutput, TNewOutput : Debug {
            let new_results : [<TDomain as Domain>::Range<TNewOutput>; N] = self.results.iter()
                .map(|x| <TDomain as Domain>::map_range(x, &function))
                .collect::<Vec<<TDomain as Domain>::Range<TNewOutput>>>()
                .try_into().unwrap();

            CalculationResults::<'a, TDomain, TNewOutput, N> {
                domain_data : self.domain_data,
                results : new_results,
                result_names : self.result_names.clone(),
            }
    }
}