use num::complex::{Complex, ComplexFloat};

// Bring functions into scope
use crate::fns::{
    apparent_power, average_power, average_power_resistor, get_p_and_q, impedance_capacitor,
    impedance_inductor, parallel, particular_response, polar_to_rect, power_factor,
    power_factor_angle, rect_to_polar, series, voltage_divider,
};

pub fn t6() {
    {
        println!("Questão 1:\n");

        let power_factor: f64 = 0.85; // late
        let apparent_power = 140_000.0; // VA
        let voltage = 110.0; // V
        let frequency = 60.0; // Hz
                              // desired power factor = 1
        let power_factor_angle = (power_factor).acos().to_degrees();
        let p = apparent_power * power_factor;
        let q = apparent_power * (1.0 - power_factor.powi(2)).sqrt();
    }
    {
        println!("\nQuestão 2:\n");
        let p1 = 2_000.0; // VA
        let power_factor1: f64 = 0.75; // early
        let p2 = 4_000.0; // VA
        let power_factor2: f64 = 0.95; // late
        let angle1 = -(power_factor1).acos().to_degrees();
        let angle2 = (power_factor2).acos().to_degrees();
        let q1 = p1 / power_factor1 * angle1.to_radians().sin();
        let q2 = p2 / power_factor2 * angle2.to_radians().sin();

        let total_complex_power = Complex::new(p1 + p2, q1 + q2);

        println!(
            "Potência complexa total: {} + {}i VA",
            total_complex_power.re / 1000.0,
            total_complex_power.im / 1000.0
        );
    }
    {
        println!("Questão 3:\n");
        let voltage = 120.0; // V
        let frequency: f64 = 60.0; // Hz
        let p1 = 4_000.0; // W
        let power_factor: f64 = 0.8; // late
        let power_factor2: f64 = 0.95; // late

        // a)
        let apparent_power = p1 / power_factor;
        let q1 = (apparent_power * (1.0 - power_factor.powi(2)).sqrt()).round();

        // b)
        let apparent_power2 = p1 / power_factor2;
        let q2 = (apparent_power2 * (1.0 - power_factor2.powi(2)).sqrt()).round();

        // d) the capacitor to change the power factor from 0.8 to 0.95
        let qc1 = (apparent_power * (1.0 - power_factor2.powi(2)).sqrt()).round();
        let qc2 = (apparent_power2 * (1.0 - power_factor2.powi(2)).sqrt()).round();
        let z = voltage.powi(2) / p1;
        let c = 1.0 / (2.0 * std::f64::consts::PI * frequency * z);
        println!("Potência aparente: {} VA", apparent_power);
        println!("Potência reativa: {} VAR", q1);
        println!("Potência aparente2: {} VA", apparent_power2);
        println!("Potência reativa2: {} VAR", q2);
        println!("Capacitância: {} uF", c * 1_000_000.0);
    }
}
