use num::complex::{Complex, ComplexFloat};

// Bring functions into scope
use crate::fns::{
    apparent_power, average_power, average_power_resistor, impedance_capacitor, impedance_inductor,
    parallel, particular_response, polar_to_rect, power_factor, rect_to_polar, series,
    voltage_divider,
};

pub fn t5() {
    {
        println!("Questão 1:\n");

        let r = [Complex::new(3.0, 0.0), Complex::new(6.0, 0.0)];
        let c = 1.0 / 2.0;
        let w = 1.0;
        let imp = series(&[r[0], parallel(r[1], impedance_capacitor(w, c))]);
        let v = polar_to_rect(6.0, 0_f64);
        let i = v / imp;

        let avg_pow_r1 = average_power_resistor(i, r[1]);

        println!(
            "A potência média em R1 ({:.2} + {:.2}i) é {:.3} W",
            r[0].re, r[0].im, avg_pow_r1
        );
    }
    {
        println!("\nQuestão 2:\n");
        let i = polar_to_rect(10.0, 30_f64);
        // Z = 20< -22 degrees
        let impedance = polar_to_rect(20.0, -22_f64);

        let avg_pow = average_power_resistor(i, impedance);

        println!("impendância: {}", impedance.abs());

        println!(
            "A potência média em R ({:.2} + {:.2}i) é {:.3} W",
            impedance.re, impedance.im, avg_pow
        );
    }
    {
        println!("Questão 3:\n");
        let i_rms = Complex::new(25.0, 0.0);
        let v_rms = Complex::new(230.0, 0.0);
        let capacitive_power = 5000;
        let apparent_pow = apparent_power(i_rms, v_rms);

        println!("Potência aparente: {:.2} VA", apparent_pow);
        let power_factor = (capacitive_power as f64 / apparent_pow).asin().cos();

        println!("Fator de potência: {:.2}", power_factor);
    }
}
