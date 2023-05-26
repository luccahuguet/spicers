use num::complex::Complex;

// Bring functions into scope
use crate::fns::{
    impedance_capacitor, impedance_inductor, parallel, particular_response, rect_to_polar, series,
    voltage_divider,
};

// Question 2
pub fn t4() {
    println!("Questão 2:\n");

    // Letra a: Frequência angular em rad/s
    let w = 1000.0;
    println!("a) Frequência angular em rad/s: {}", w);

    // Letra b: Impedância do resistor
    let r = Complex::new(50.0, 0.0);
    println!("b) Impedância do resistor: {}", r);

    // Letra c: Impedância do indutor
    let l = 25e-3;
    let imp_inductor = impedance_inductor(w, l);
    println!("c) Impedância do indutor: {}", imp_inductor);

    // Letra d: Impedância do capacitor
    let c = 10e-6;
    let imp_capacitor = impedance_capacitor(w, c);
    println!("d) Impedância do capacitor: {}", imp_capacitor);

    // Letra f: Impedância de Entrada na forma polar
    let imp_entrada = series(&[r, series(&[imp_inductor, imp_capacitor])]);
    println!("f) Impedância de Entrada na forma polar: {}", imp_entrada);

    // Letra g: Admitância de Entrada em S
    let adm_entrada = 1.0 / imp_entrada;
    println!("g) Admitância de Entrada em S: {}", adm_entrada);

    // Letra h: Fasores de Corrente e Tensão
    let v_magnitude = 35.0;
    let v_phase = 30_f64.to_radians(); // Convert the phase from degrees to radians
    println!("h) Fasor de Tensão: {:.2}∠{:.2}°", v_magnitude, v_phase);

    let v = v_magnitude * Complex::new(v_phase.cos(), v_phase.sin()); // Phasor representation of the current
    let i = v / imp_entrada;
    let i_phasor = rect_to_polar(i);
    println!("Fasor de Corrente: {:.2}∠{:.2}°", i_phasor.0, i_phasor.1);

    // Letra i: Resposta Particular ip(t)
    println!(
        "i) Resposta Particular ip(t): {}",
        particular_response(i, w)
    );
}
