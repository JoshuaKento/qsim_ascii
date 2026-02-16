use std::f64::consts::SQRT_2;

use crate::state::QState;

fn assert_target_in_range(state: &QState, target: usize) {
    assert!(
        target < state.get_nqbits(),
        "target qubit index out of range: target={target}, nqubits={}",
        state.get_nqbits()
    );
}

// Pauli-X flip
pub fn apply_x(state: &mut QState, target: usize) {
    assert_target_in_range(state, target);

    let bit = 1usize << target; // target's bit position
    let n = state.get_amps().len(); // dimension of n

    for i in 0..n {
        if (i & bit) == 0 {
            // i is 0 in index bit
            let j = i ^ bit; // flip i in index bit (0 > 1)
            state.get_amps().swap(i, j);
        }
    }
}

// Hadamard
pub fn apply_h(state: &mut QState, target: usize) {
    assert_target_in_range(state, target);

    let bit = 1usize << target; // target's bit position
    let n = state.get_amps().len(); // dimension of n

    for i in 0..n {
        if (i & bit) == 0 {
            // i is 0 in index bit

            let j = i ^ bit; // flip i in index bit (0 > 1)

            let a0 = state.get_amps()[i];
            let a1 = state.get_amps()[j];

            state.get_amps()[i] = (a0 + a1) / SQRT_2;
            state.get_amps()[j] = (a0 - a1) / SQRT_2;
        }
    }
}

pub fn apply_cnot(state: &mut QState, control: usize, target: usize) {
    assert_target_in_range(state, control);
    assert_target_in_range(state, target);
    assert!(control != target, "control and target must be different");

    let cbit = 1usize << control;
    let tbit = 1usize << target;
    let n = state.get_amps().len();

    for i in 0..n {
        if (cbit & i) != 0 && (tbit & i) == 0 {
            let j = i ^ tbit;
            state.get_amps().swap(i, j);
        }
    }
}
