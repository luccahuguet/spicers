use num::complex::Complex;

// Import the fns module
// mod fns;

// Bring functions into scope
use crate::fns::{
    complex_to_phasor, impedance_capacitor, impedance_inductor, parallel, particular_response,
    series, voltage_divider,
};

// Question 1
pub fn t3() {
    {
        let w = 1000.0;
        let l = 10e-3;
        let c = 100e-6;
        let r1 = Complex::new(10.0, 0.0);
        let r2 = Complex::new(5.0, 0.0);
        let magnitude = 10.0;
        let phase = 0_f64.to_radians(); // Convert the phase from degrees to radians
        let i = magnitude * Complex::new(phase.cos(), phase.sin()); // Phasor representation of the current
        let impedance = parallel(
            impedance_capacitor(w, c),
            series(r1, parallel(r2, impedance_inductor(w, l))),
        );

        let v = i * impedance;

        // Convert the complex voltage to phasor form
        let v_magnitude = v.norm();
        let v_phase = v.im.atan2(v.re).to_degrees(); // Convert the phase from radians to degrees

        println!(
            "The voltage in phasor form is: {:.2}∠{:.2}°",
            v_magnitude, v_phase
        );
    }
    // Question 2
    {
        let w = 10.0;
        let l = 0.5;
        let c = 1.0 / 20.0;
        let r = Complex::new(10.0, 0.0);
        let magnitude = 20.0;
        let phase = 100_f64.to_radians(); // Convert the phase from degrees to radians
        let vin = magnitude * Complex::new(phase.cos(), phase.sin()); // Phasor representation of the current
        let v0 = voltage_divider(
            vin,
            parallel(r, impedance_inductor(w, l)),
            impedance_capacitor(w, c),
        );

        // Convert the complex voltage to phasor form
        let (v0_magnitude, v0_phase) = complex_to_phasor(v0);

        println!(
            "The voltage in phasor form is: {:.2}∠{:.2}°",
            v0_magnitude, v0_phase
        );
    }
    // Question 3
    {
        let w = 50.0;
        let l = 0.2;
        let c1 = 2e-3;
        let c2 = 10e-3;
        let r1 = Complex::new(3.0, 0.0);
        let r2 = Complex::new(8.0, 0.0);
        let impedance = series(
            impedance_capacitor(w, c1),
            parallel(
                series(r1, impedance_capacitor(w, c2)),
                series(r2, impedance_inductor(w, l)),
            ),
        );

        println!("\n The impedance is: {}", impedance);
    }
    // Question 3
    {
        let w = 50.0;
        let l = 0.2;
        let c1 = 2e-3;
        let c2 = 10e-3;
        let r1 = Complex::new(3.0, 0.0);
        let r2 = Complex::new(8.0, 0.0);
        let impedance = series(
            impedance_capacitor(w, c1),
            parallel(
                series(r1, impedance_capacitor(w, c2)),
                series(r2, impedance_inductor(w, l)),
            ),
        );

        println!("\n The impedance is: {}", impedance);
    }
    // Question 4
    {
        let i2 = Complex::new(2.0, 0.0);
        let i1 = Complex::new(20.0, 4.0) / Complex::new(6.0, 1.0);
        let r = Complex::new(4.0, 0.0);
        println!("i1 = {}", i1);
        let v_phasor = complex_to_phasor(r * (i1 - i2));
        println!("v = {:.2}∠{:.2}°", v_phasor.0, v_phasor.1);
    }
}
