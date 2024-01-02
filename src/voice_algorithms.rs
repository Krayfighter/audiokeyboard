
use std::collections::HashMap;

// pub mod voice_algorithms {

use std::f32::consts::PI;
use std::f32::consts::TAU;

pub const VOICES: &[(&'static str, fn(f32, f32, f32) -> f32)] = &[
    ("sine", sine_function),
    ("soft square wave", soft_square),
    ("electric piano", electric_piano),
    ("vibrawave", vibrawave),
    ("oboe synth", vibrawave2),
];

pub fn sine_function(x: f32, freq: f32, srate: f32) -> f32 {
    return sine(x, freq, srate);
}

#[inline]
fn angle(x: f32, freq: f32) -> f32 {
    return TAU * x * freq;
}

#[inline]
fn sine(x: f32, freq: f32, srate: f32) -> f32 {
    return ((TAU * x * freq) / srate).sin();
}

pub fn soft_square(x: f32, freq: f32, srate: f32) -> f32 {
    return ((
        angle(x, freq) / srate
    ).sin() * 100.).tanh();
}


pub fn electric_piano(x: f32, freq: f32, srate: f32) -> f32 {
    return (
        sine(x, freq, srate)+
        sine(2.*x, freq, srate)
    ) / 2.0;
}

pub fn vibrawave(x: f32, freq: f32, srate: f32) -> f32 {
    return (
        sine(x      , freq, srate)+
        sine(0.22*x , freq, srate)+
        sine(1.05*x , freq, srate)+
        sine(0.25*x , freq, srate)+
        sine(0.25*x , freq, srate)+
        sine(0.275*x, freq, srate)
    ) / 1.6; // ???? translated from old code
}

pub fn vibrawave2(x: f32, freq: f32, srate: f32) -> f32 {
    return (
        sine(x      , freq, srate)+
        sine(0.8*x  , freq, srate)+
        sine(0.6*x  , freq, srate)+
        sine(1.2*x  , freq, srate)
    ) / 1.6; // again weird constant
}

#[inline]
pub fn _square_wave1(xval: f32) -> f32 {
    if xval.sin() > 0.0 {
        return 1.0
    }else {
        return -1.0
    }
}

// #[inline]
// pub fn _square_wave2(xval: f32) -> f32 {
//     // println
//     return (xval.sin().powf(0.2)+xval.sin())/1000000000.0; // good square sound but flawed
// }

#[inline]
pub fn _double_sine(xval: f32) -> f32 {
    return (xval.sin()+(xval*2.0).sin())/1.8
}

#[inline]
pub fn _triple_sine(xval: f32) -> f32 {
    return (xval.sin()+(xval*2.0).sin()+(xval*3.0).sin())/2.5
}

#[inline]
pub fn _quadruple_sine(xval: f32) -> f32 {
    return (xval.sin()+(xval*2.0).sin()+(xval*3.0).sin()+(xval*4.0).sin())/3.3
}

// #[inline]
// pub fn _test(xval: f32) -> f32 {
//     // return ((xval.sin()).powf(0.333)).sin()
//     // return (xval.sin()*xval).powf(0.2).sin()
//     // return (xval.sin()*xval.powf(2.0)).powf(0.2).sin()
//     // return xval.sin().powf(0.333);
//     // return (xval.powf(2.0).sin())/2.0+(xval+0.5).cos()
//     // return ((xval.powf(2.0).sin()/10.0)+xval.sin())/1.2 // good space sound
// }

#[inline]
pub fn _clarinet1(xval: f32) -> f32 {
    return (
        xval.sin()+
        ((2.0*xval).sin()*0.04)+
        ((3.0*xval).sin()*0.99)+
        ((4.0*xval).sin()*0.12)+
        ((5.0*xval).sin()*0.53)+
        ((6.0*xval).sin()*0.11)+
        ((7.0*xval).sin()*0.26)+
        ((8.0*xval).sin()*0.05)+
        ((9.0*xval).sin()*0.24)+
        ((10.0*xval).sin()*0.07)+
        ((11.0*xval).sin()*0.02)+
        ((12.0*xval).sin()*0.03)+
        ((13.0*xval).sin()*0.02)+
        ((14.0*xval).sin()*0.03)
    )/2.0
}

// #[inline]
// pub fn test(xval: f32) -> f32 {
//     return xval.log10().sin()
// }

// }
