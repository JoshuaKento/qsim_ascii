use crate::state::QState;

/// Measure all qubits
/// ToDo HIGH validate total > 0.0
pub fn measure_all<R: rand::RngExt + ?Sized>(state: &mut QState, rng: &mut R) -> usize {
    // 1) compute probabilities p[k] = |a[k]|^2
    let mut total: f64 = 0.0;
    for i in state.get_amps().iter() {
        // p[k] = a[k] ^ 2
        total += i.norm_sqr();
    }

    // 2) sample an index m according to p
    // "measuring" r
    let r: f64 = rng.random_range(0.0..total);

    let mut acc = 0.0;

    // will be overwritten when r is crossed
    let mut measured: usize = 0;

    for (k, a) in state.get_amps().iter().enumerate() {
        acc += a.norm_sqr();

        if r < acc {
            measured = k;
            break;
        }
    }

    // 3) collapse: set amps[measured] = 1, everything else = 0
    for a in state.get_amps().iter_mut() {
        *a = num_complex::Complex64::new(0.0, 0.0);
    }

    state.get_amps()[measured] = num_complex::Complex64::new(1.0, 0.0);

    measured
}
