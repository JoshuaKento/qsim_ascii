use crate::state::QState;
use rand::RngExt;

fn assert_target_in_range(state: &QState, target: usize) {
    assert!(
        target < state.get_nqbits(),
        "target qubit index out of range: target={target}, nqubits={}",
        state.get_nqbits()
    );
}

fn qubit_weights(state: &QState, target: usize) -> (f64, f64) {
    assert_target_in_range(state, target);
    let mut p0 = 0.0;
    let mut p1 = 0.0;
    for (idx, amp) in state.amps().iter().enumerate() {
        if ((idx >> target) & 1) == 0 {
            p0 += amp.norm_sqr();
        } else {
            p1 += amp.norm_sqr();
        }
    }
    (p0, p1)
}

pub fn qubit_probability_one(state: &QState, target: usize) -> f64 {
    let (p0, p1) = qubit_weights(state, target);
    let total = p0 + p1;
    assert!(
        total.is_finite() && total > 0.0,
        "cannot compute qubit probability from non-positive total probability"
    );
    p1 / total
}

pub fn measure_qubit<R: rand::Rng + ?Sized>(
    state: &mut QState,
    target: usize,
    rng: &mut R,
) -> usize {
    let (p0, p1) = qubit_weights(state, target);
    let total = p0 + p1;
    assert!(
        total.is_finite() && total > 0.0,
        "cannot measure state with non-positive total probability"
    );

    let r: f64 = rng.random::<f64>() * total;
    let outcome = if r < p0 { 0usize } else { 1usize };

    for (idx, amp) in state.get_amps().iter_mut().enumerate() {
        if ((idx >> target) & 1) != outcome {
            *amp = num_complex::Complex64::new(0.0, 0.0);
        }
    }
    state.normalize();

    outcome
}

/// Measure the full register and collapse to one basis state.
pub fn measure_all<R: rand::Rng + ?Sized>(state: &mut QState, rng: &mut R) -> usize {
    let probs: Vec<f64> = state.amps().iter().map(|a| a.norm_sqr()).collect();
    let total: f64 = probs.iter().sum();
    assert!(
        total.is_finite() && total > 0.0,
        "cannot measure state with non-positive total probability"
    );

    let r: f64 = rng.random::<f64>() * total;

    let mut acc = 0.0;
    let mut measured = probs.len().saturating_sub(1);
    for (k, p) in probs.iter().enumerate() {
        acc += *p;
        if r < acc {
            measured = k;
            break;
        }
    }

    let amps = state.get_amps();
    for a in amps.iter_mut() {
        *a = num_complex::Complex64::new(0.0, 0.0);
    }
    amps[measured] = num_complex::Complex64::new(1.0, 0.0);

    measured
}

pub fn measure_all_with_thread_rng(state: &mut QState) -> usize {
    let mut rng = rand::rng();
    measure_all(state, &mut rng)
}
