use num::complex::{Complex64, ComplexFloat};

const EGAMMA : f64 = 0.577215664901532860606512090082402431_f64;
    
const EIN_TERM_COUNT : usize = 11;
const EIN_NUMERATOR: [f64; EIN_TERM_COUNT] = [
    0.10000000000000000000e1,
    0.20502084567791706628e0,
    0.39390751931629655245e-1,
    0.34858236552923791179e-2,
    0.29317755061426648946e-3,
    0.13754735702992239386e-4,
    0.60964461747745580030e-6,
    0.14447186550089174805e-7,
    0.30430043273133224684e-9,
    0.22059389087476526250e-11,
    0.49848280581687288340e-14,
];
const EIN_DENOMINATOR: [f64; EIN_TERM_COUNT] = [
    0.10000000000000000000e1,
    0.45502084567791806628e0,
    0.97590407795553366258e-1,
    0.13021156399851994778e-1,
    0.11999111377470476100e-2,
    0.80015095592166145984e-4,
    0.39222830738857592254e-5,
    0.14003621189603245150e-6,
    0.55465894537386945817e-10,
    0.42591339012402143020e-12,
    0.0,
];

const E1_TERM_COUNT: usize = 11;
const E1_NUMERATOR: [i32; E1_TERM_COUNT] = [
    1,
    109,
    4842,
    114064, 
    1553663, 
    12518100, 
    58603440, 
    150023520, 
    184386240, 
    80627040, 
    3628800,
];
const E1_DENOMINATOR: [i32; E1_TERM_COUNT] = [
    1,
    110,
    4950,
    118800,
    1663200,
    13970880,
    69854400,
    199584000,
    299376000,
    199584000,
    39916800,
];

/// Exponential integral (DLMF 6.2.3)
pub fn ein(input: Complex64) -> Complex64  {
    if input.abs() < 4.0 {
        let numerator : Complex64 = EIN_NUMERATOR.iter().enumerate().map(|(pos, x)| x * input.powi(pos as i32)).sum();
        let denominator : Complex64 = EIN_DENOMINATOR.iter().enumerate().map(|(pos, x)| x * input.powi(pos as i32)).sum();
        return input * numerator / denominator;
    }

    let numerator : Complex64 = E1_NUMERATOR.iter().enumerate().map(|(pos, x)| *x as f64 * input.powi((E1_TERM_COUNT - 1 - pos) as i32)).sum();
    let denominator : Complex64 = E1_DENOMINATOR.iter().enumerate().map(|(pos, x)| *x as f64 * input.powi((E1_TERM_COUNT - 1 - pos) as i32)).sum();
    return (-input).exp() * input.powi(-1) * numerator / denominator + input.ln() + EGAMMA;
}