/// This entire document uses https://link.springer.com/article/10.1007/bf02142806 for numerics

use std::f64::consts::PI;
use num::{complex::ComplexFloat, Float, NumCast};
use crate::{calculators::polynomial_ratio, EM_GAMMA};


/// Sine integral Si (DLMF 6.2.9)
pub fn capital_si<TNum : ComplexFloat>(x : TNum) -> TNum {
    let input_norm = x.abs();
    let input_norm_squared = x.abs() * x.abs();

    // Begin with limiting cases where machine epsilon is relevant and simplifies things
    if input_norm_squared < TNum::Real::epsilon() * NumCast::from(18).unwrap() {
        return x;
    }

    if input_norm > <TNum::Real as NumCast>::from(2).unwrap() / TNum::Real::epsilon() {
        return NumCast::from(PI / 2.0).unwrap();
    }

    if input_norm_squared > <TNum::Real as NumCast>::from(6).unwrap() / TNum::Real::epsilon() {
        return <TNum as NumCast>::from(PI / 2.0).unwrap() - x.cos() / x - x.sin() / (x * x);
    }

    if input_norm < NumCast::from(6).unwrap() {
        return x * polynomial_ratio(&SI_NUMERATOR, &SI_DENOMINATOR, x * x).0;
    }

    let sign_flipped = if x.re().is_sign_negative() { -x } else { x };
    let sine_sign_flipped = <TNum as NumCast>::from(PI / 2.0).unwrap() - f(sign_flipped) * sign_flipped.cos() - g(sign_flipped) * sign_flipped.sin();

    return if x.re().is_sign_negative() { -sine_sign_flipped } else { sine_sign_flipped };
}

// Cosine integral, not defined for negative real values
pub fn ci<TNum : ComplexFloat>(x : TNum) -> TNum {
    let x_abs = x.abs();

    if x_abs < <TNum::Real as NumCast>::from(2).unwrap() * num::Float::sqrt(<TNum as ComplexFloat>::Real::epsilon()) {
        return <TNum as NumCast>::from(EM_GAMMA).unwrap() + x.ln();
    }

    if x_abs > <TNum::Real as NumCast>::from(PI).unwrap() / TNum::Real::epsilon() {
        return TNum::zero();
    }

    if x_abs < NumCast::from(3).unwrap() {
        let ci_r0_cast = <TNum as NumCast>::from(CI_R0).unwrap();
        return (x / ci_r0_cast).ln() + (x * x - ci_r0_cast * ci_r0_cast) * polynomial_ratio(&CI_NUMERATOR_LEQ_3, &CI_DENOMINATOR_LEQ_3, x * x).0;
    }

    if x_abs < NumCast::from(6).unwrap() {
        let ci_r1_cast = <TNum as NumCast>::from(CI_R1).unwrap();
        return (x / ci_r1_cast).ln() + (x * x - ci_r1_cast * ci_r1_cast) * polynomial_ratio(&CI_NUMERATOR_G_3_LEQ_6, &CI_DENOMINATOR_G_3_LEQ_6, x * x).0;
    }

    return f(x) * x.sin() - g(x) * x.cos();
}

/// Complementary cosine integral Cin (DLMF 6.2.12)
pub fn cin<TNum : ComplexFloat>(x : TNum) -> TNum {
    let x_pos = if x.re().is_sign_negative() { -x } else { x };
    return -ci(x_pos) + x_pos.ln() + NumCast::from(EM_GAMMA).unwrap()
}

/// Auxiliary trigonometric integral, not defined for negative real values
pub fn f<TNum : ComplexFloat>(x : TNum) -> TNum {
    if x.abs() < <TNum::Real as NumCast>::from(6).unwrap() {
        return ci(x) * x.sin() - (capital_si(x) - NumCast::from(PI / 2.0).unwrap()) * x.cos()
    }

    if x.abs() <= NumCast::from(12.0).unwrap() {
        return polynomial_ratio(&F_NUMERATOR_SMALL, &F_DENOMINATOR_SMALL, TNum::one() / (x * x)).0 / x;
    }

    return (TNum::one() - polynomial_ratio(&F_NUMERATOR_BIG, &F_DENOMINATOR_BIG, TNum::one() / (x * x)).0 / (x * x)) / x;
}

// Auxiliary trigonometric integral, not defined for negative real values
pub fn g<TNum : ComplexFloat>(x : TNum) -> TNum {
    if x.abs() < NumCast::from(6).unwrap() {
        return -ci(x) * x.cos() - (capital_si(x) - NumCast::from(PI / 2.0).unwrap()) * x.sin();
    }

    if x.abs() <= NumCast::from(12.0).unwrap() {
        return polynomial_ratio(&G_NUMERATOR_SMALL, &G_DENOMINATOR_SMALL, TNum::one() / (x * x)).0 / (x * x);
    }

    return (TNum::one() - polynomial_ratio(&G_NUMERATOR_BIG, &G_DENOMINATOR_BIG, TNum::one() / (x * x)).0 / (x * x)) / (x * x);
}

const CI_R0 : f64 = 0.616505485620716233797110404100;
const CI_R1 : f64 = 3.384180422851186426397851146402;

const SI_NUMERATOR : [f64; 8] = [
    1.00000000000000000000E0,
    -0.44663998931312457298E-1,
    0.11209146443112369449E-2,
    -0.13276124407928422367E-4,
    0.85118014179823463879E-7,
    -0.29989314303147656479E-9,
    0.55401971660186204711E-12,
    -0.42406353433133212926E-15,
];

const SI_DENOMINATOR : [f64; 8] = [
    1.00000000000000000000E0,
    0.10891556624243098264E-1,
    0.59334456769186835896E-4,
    0.21231112954641805908E-6,
    0.54747121846510390750E-9,
    0.10378561511331814674E-11,
    0.13754880327250272679E-14,
    0.10223981202236205703E-17
];

const CI_NUMERATOR_LEQ_3 : [f64; 6] = [
    -0.24607411378767540707E0,
    0.72113492241301534559E-2,
    -0.11867127836204767056E-3,
    0.90542655466969866243E-6,
    -0.34322242412444409037E-8,
    0.51950683460656886834E-11,
];

const CI_DENOMINATOR_LEQ_3 : [f64; 6] = [
    1.00000000000000000000,
    0.12670095552700637845E-1,
    0.78168450570724148921E-4,
    0.29959200177005821677E-6,
    0.73191677761328838216E-9,
    0.94351174530907529061E-12,
];

const CI_NUMERATOR_G_3_LEQ_6 : [f64; 8] = [
    -0.15684781827145408780E0,
    0.66253165609605468916E-2,
    -0.12822297297864512864E-3,
    0.12360964097729408891E-5,
    -0.66450975112876224532E-8,
    0.20326936466803159446E-10,
    -0.33590883135343844613E-13,
    0.23686934961435015119E-16,
];

const CI_DENOMINATOR_G_3_LEQ_6 : [f64; 8] = [
    1.00000000000000000000E0,
    0.96166044388828741188E-2,
    0.45257514591257035006E-4,
    0.13544922659627723233E-6,
    0.27715365686570002081E-9,
    0.37718676301688932926E-12,
    0.27706844497155995398E-15,
    0.00000000000000000000E0,
];

const F_NUMERATOR_SMALL : [f64; 8] = [
    0.99999999962173909991, 
    0.36451060338631902917e3, 
    0.44218548041288440874e5,
    0.22467569405961151887e7, 
    0.49315316723035561922e8, 
    0.43186795279670283193e9,
    0.11847992519956804350e10, 
    0.45573267593795103181e9,
];

const F_DENOMINATOR_SMALL : [f64; 8] = [
    1.0, 
    0.36651060273229347594e3, 
    0.44927569814970692777e5, 
    0.23285354882204041700e7,
    0.53117852017228262911e8, 
    0.50335310667241870372e9, 
    0.16575285015623175410e10,
    0.11746532837038341076e10,
];

const G_NUMERATOR_SMALL : [f64; 9] = [
    0.99999999920484901956, 
    0.51385504875307321394e3, 
    0.92293483452013810811e5,
    0.74071341863359841727e7, 
    0.28142356162841356551e9, 
    0.49280890357734623984e10,
    0.35524762685554302472e11, 
    0.79194271662085049376e11, 
    0.17942522624413898907e11,
];

const G_DENOMINATOR_SMALL : [f64; 9] = [
    1.0, 
    0.51985504708814870209e3, 
    0.95292615508125947321e5, 
    0.79215459679762667578e7,
    0.31977567790733781460e9, 
    0.62273134702439012114e10, 
    0.54570971054996441467e11,
    0.18241750166645704670e12, 
    0.15407148148861454434e12,
];

const F_NUMERATOR_BIG : [f64; 8] = [
    0.19999999999999978257e1, 
    0.22206119380434958727e4, 
    0.84749007623988236808e6,
    0.13959267954823943232e9, 
    0.10197205463267975592e11, 
    0.30229865264524075951e12,
    0.27504053804288471142e13, 
    0.21818989704686874983e13,
];

const F_DENOMINATOR_BIG : [f64; 8] = [
    1.0, 
    0.11223059690217167788e4, 
    0.43685270974851313242e6, 
    0.74654702140658116258e8,
    0.58580034751805687471e10, 
    0.20157980379272098841e12, 
    0.26229141857684496445e13,
    0.87852907334918467516e13,
];

const G_NUMERATOR_BIG : [f64; 9] = [
    0.59999999999999993089e1, 
    0.96527746044997139158e4, 
    0.56077626996568834185e7,
    0.15022667718927317198e10, 
    0.19644271064733088465e12, 
    0.12191368281163225043e14,
    0.31924389898645609533e15, 
    0.25876053010027485934e16, 
    0.12754978896268878403e16,
];

const G_DENOMINATOR_BIG : [f64; 9] = [
    1.0, 
    0.16287957674166143196e4, 
    0.96636303195787870963e6, 
    0.26839734750950667021e9,
    0.37388510548029219241e11, 
    0.26028585666152144496e13, 
    0.85134283716950697226e14,
    0.11304079361627952930e16, 
    0.42519841479489798424e16,
];

#[cfg(test)]
mod tests {
    extern crate num;

    use super::{capital_si,cin};
    use num::complex::{Complex64, ComplexFloat};

    #[test]
    fn si_two() {
        assert!((capital_si(2.0) - 1.60541).abs() < 0.001);
    }

    #[test]
    fn cin_two() {
        assert!((cin(2.0) - 0.847382) < 0.001)
    }

    #[test]
    fn cin_minus_five() {
        assert!((cin(-5.0) - 2.37668) < 0.001);
    }

    #[test]
    fn si_six_plus_two_i() {
        assert!((capital_si(Complex64::new(6.0, 2.0)) - Complex64::new(0.974993, -0.063689)).abs() < 0.001);
    }
}