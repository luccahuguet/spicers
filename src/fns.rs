use std::string;

use num::complex::Complex;
const J: Complex<f64> = Complex::new(0.0, 1.0);

pub fn parallel(ra: Complex<f64>, rb: Complex<f64>) -> Complex<f64> {
    1.0 / (1.0 / ra + 1.0 / rb)
}

pub fn series(resistances: &[Complex<f64>]) -> Complex<f64> {
    resistances.iter().sum()
}

pub fn impedance_inductor(w: f64, l: f64) -> Complex<f64> {
    J * w * l
}

pub fn impedance_capacitor(w: f64, c: f64) -> Complex<f64> {
    -J * 1.0 / (w * c)
}

pub fn voltage_divider(vin: Complex<f64>, r1: Complex<f64>, r2: Complex<f64>) -> Complex<f64> {
    vin * (r2 / (r1 + r2))
}

pub fn rect_to_polar(phasor: Complex<f64>) -> (f64, f64) {
    let phasor_magnitude = phasor.norm();
    let phasor_phase: f64 = phasor.im.atan2(phasor.re).to_degrees(); // Convert the phase from radians to degrees
    (phasor_magnitude, phasor_phase)
}

pub fn polar_to_rect(magnitude: f64, phase: f64) -> Complex<f64> {
    let phase_rad = phase.to_radians();
    Complex::new(magnitude * phase_rad.cos(), magnitude * phase_rad.sin())
}

pub fn particular_response(phasor: Complex<f64>, w: f64) -> string::String {
    let (magnitude, phase) = rect_to_polar(phasor);
    format!("{:.2}cos({:.2}t + {:.2}°)", magnitude, w, phase)
}

pub fn eval_in_t(phasor: Complex<f64>, t: f64, w: f64) -> f64 {
    let (magnitude, phase) = rect_to_polar(phasor);
    magnitude * (w * t + phase.to_radians()).cos()
}

pub fn average_power(peak_i: Complex<f64>, peak_v: Complex<f64>) -> f64 {
    let (i_magnitude, i_phase) = rect_to_polar(peak_i);
    let (v_magnitude, v_phase) = rect_to_polar(peak_v);
    i_magnitude * v_magnitude * (v_phase - i_phase).to_radians().cos() / 2.0
}

pub fn average_power_resistor(peak_i: Complex<f64>, impedance: Complex<f64>) -> f64 {
    let (i_magnitude, _) = rect_to_polar(peak_i);
    i_magnitude.powi(2) * impedance.re / 2.0
}

pub fn power_factor(i: Complex<f64>, v: Complex<f64>) -> f64 {
    let (_i_magnitude, i_phase) = rect_to_polar(i);
    let (_v_magnitude, v_phase) = rect_to_polar(v);
    (v_phase - i_phase).to_radians().cos()
}

pub fn power_factor_angle(q: f64, p: f64) -> f64 {
    q.atan2(p).to_degrees()
}

pub fn apparent_power(i: Complex<f64>, v: Complex<f64>) -> f64 {
    let (i_magnitude, _) = rect_to_polar(i);
    let (v_magnitude, _) = rect_to_polar(v);
    i_magnitude * v_magnitude
}

pub fn get_p_and_q(apparent_power: f64, power_factor: f64) -> (f64, f64) {
    let p = apparent_power * power_factor;
    let q = apparent_power * (1.0 - power_factor.powi(2)).sqrt();
    (p, q)
}
