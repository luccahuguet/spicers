use num::complex::{Complex, ComplexFloat};

// fp = fator de potência
// q = potência reativa
// p = potência ativa
// v = tensão
// f = frequência
// theta_fp = ângulo de fp
// s = potência aparente
// w = frequência angular
// qc = reatância capacitiva
// c = capacitância

#[allow(dead_code)]
pub fn t6() {
    {
        let fp: f64 = 0.85; // atrasado
        let q = 140_000.0; // Var
        let v = 110.0; // V
        let f = 60.0; // Hz

        let theta_fp = (fp).acos().to_degrees();
        let s = q / theta_fp.to_radians().sin();
        let p = s * fp;

        // Fator de potência desejado
        let new_fp: f64 = 1.0;
        // Achar o novo theta
        let new_theta_fp = (new_fp).acos().to_degrees();
        // Como a potencia ativa é a mesma, podemos calcular a nova potencia reativa usando arco tangente
        let new_q = new_theta_fp.to_radians().tan() / p;

        let w = 2.0 * std::f64::consts::PI * f;
        let q_adicionado = (new_q - q).abs();

        // Qc = v^2 / Xc
        let impedancia_cap = v.powi(2) / q_adicionado;
        let capacitancia = 1.0 / (w * impedancia_cap);

        println!("\n Questão 1:\n");
        println!("novo fator de potência: {:.2}", new_fp);
        println!("novo theta: {:.2}", new_theta_fp);
        println!("nova potência reativa: {:.2} VAR", new_q);
        println!("potencia aparante: {:.2} kVA", s / 1000.0);
        println!("impendância capacitiva: {:.2} ohms", impedancia_cap);
        println!("capacitância: {:.2} mF", capacitancia * 1_000.0);
    }
    {
        let p1 = 2_000.0; // VA
        let fp1: f64 = 0.75; // early
        let p2 = 4_000.0; // VA
        let fp2: f64 = 0.95; // late
        let theta1 = -(fp1).acos().to_degrees();
        let theta2 = (fp2).acos().to_degrees();
        let q1 = p1 / fp1 * theta1.to_radians().sin();
        let q2 = p2 / fp2 * theta2.to_radians().sin();

        let potencia_complexa: Complex<f64> = Complex::new(p1 + p2, q1 + q2);

        println!("\nQuestão 2:\n");
        println!(
            "Potência complexa total: {:.2} + {:.2}j kVA",
            potencia_complexa.re / 1000.0,
            potencia_complexa.im / 1000.0
        );
    }
    {
        println!("\nQuestão 3:\n");
        let v = Complex::new(120.0, 0.0);
        let f: f64 = 60.0; // Hz
        let p1 = 4_000.0; // W
        let fp1: f64 = 0.8; // late
        let fp2: f64 = 0.95; // late

        // a)
        let theta1 = (fp1).acos().to_degrees();
        let q1 = p1 * theta1.to_radians().tan();
        let s1: f64 = p1 / fp1;
        println!("Letra a:");
        println!("Potência aparente: {:.2} VA", s1);
        println!("Potência reativa: {:.2} VAR", q1);

        // b)
        let theta2 = (fp2).acos().to_degrees();
        let q2 = p1 * theta2.to_radians().tan();
        let s2: f64 = p1 / fp2;
        println!("\nLetra b:");
        println!("Potência aparente2: {:.2} VA", s2);
        println!("Potência reativa2: {:.2} VAR", q2);

        // d) the capacitor to change the power factor from 0.8 to 0.95
        let q_adicionado = (q2 - q1).abs();
        let w = 2.0 * std::f64::consts::PI * f;
        let impedancia_cap = v.powi(2) / q_adicionado;
        let capacitancia = 1.0 / (w * impedancia_cap);
        println!("\nLetra d:");
        println!("Potência reativa adicionada: {:.2} VAR", q_adicionado);
        println!("impendância capacitiva: {:.2} ohms", impedancia_cap);
        println!("Capacitância: {:.2} uF", capacitancia * 1_000_000.0);

        // e)
        // complex conjugate
        let i_eff1 = (s1 / v).conj().abs();
        let i_eff2 = (s2 / v).conj().abs();
        println!("\nLetra e:\nCorrente eficaz Inicial: {:.2} A", i_eff1);
        println!("Corrente eficaz final: {:.2} A", i_eff2);
        println!("Diferença: {:.2} A", i_eff2 - i_eff1);
    }
    {
        let v = Complex::new(120.0, 0.0);
        let f = 60.0; // Hz
        let p1 = 4_000.0; // W
        let fp1: f64 = 0.8; // atrasado
        let theta1 = (fp1).acos().to_degrees();
        let q1 = p1 * theta1.to_radians().tan();
        let _s1 = p1 / fp1;

        let fp2: f64 = 0.95; // atrasado
        let theta2 = (fp2).acos().to_degrees();
        let q2 = p1 * theta2.to_radians().tan();
        let _s2 = p1 / fp2;

        let q_adicionado = q2 - q1;
        let w = 2.0 * std::f64::consts::PI * f;
        let imp_cap = (v.powi(2) / q_adicionado).conj();
        let capacitancia = 1.0 / (w * imp_cap.abs());

        println!("\nQuestão 4:\n");
        println!("Capacitância: {:.2} uF", capacitancia * 1_000_000.0);
        println!("Impedância capacitiva: {:.2} ohms", imp_cap);
    }
}
