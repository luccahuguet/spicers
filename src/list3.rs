use num::complex::Complex;

// Bring functions into scope
use crate::fns::{
    eval_in_t, impedance_capacitor, impedance_inductor, polar_to_rect, rect_to_polar, series,
};

#[allow(dead_code)]
pub fn list3() {
    println!("Questão 1:\n");
    {
        let i = Complex::new(30e-3, 10e-3);
        let w = 1000.0;
        let r = Complex::new(40.0, 0.0);
        let l = 30e-3;
        let c = 40e-6;
        let t = 1e-3;

        // letter a
        let v_r = i * r;
        let v_r_phasor = rect_to_polar(v_r);
        println!("Vr = {:.2}∠{:.2}°", v_r_phasor.0, v_r_phasor.1);
        println!("Vr(1) = {:.2}", eval_in_t(v_r, t, w));

        // letter b
        let v_l = i * impedance_inductor(w, l);
        let v_l_phasor = rect_to_polar(v_l);
        println!("Vl = {:.2}∠{:.2}°", v_l_phasor.0, v_l_phasor.1);
        println!("Vl(1) = {:.2}", eval_in_t(v_l, t, w));

        // letter c
        let v_c = i * impedance_capacitor(w, c);
        let v_c_phasor = rect_to_polar(v_c);
        println!("Vc = {:.2}∠{:.2}°", v_c_phasor.0, v_c_phasor.1);
        println!("Vc(1) = {:.2}", eval_in_t(v_c, t, w));
    }
    println!("\n\nQuestão 2:\n");
    {
        let r = Complex::new(200.0, 0.0);
        let l = 0.04;
        let c = 0.25 * 1e-6;
        let w = [8_000, 10_000, 12_500].map(f64::from);
        let i_mag = 30e-3;
        let i_phase = 45.0;
        let i = polar_to_rect(i_mag, i_phase);

        for w in w.iter() {
            let z = series(&[r, impedance_inductor(*w, l), impedance_capacitor(*w, c)]);
            let v = i * z;
            let v_phasor = rect_to_polar(v);
            println!("for w = {:.2}: V = {:.2}∠{:.2}°", w, v_phasor.0, v_phasor.1);
        }
    }
}
