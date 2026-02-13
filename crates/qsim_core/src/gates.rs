use std::f64::consts::SQRT_2;

use crate::state::QState;

// Pauli-X flip
pub fn apply_x(state: &mut QState, target: usize) {
    let bit = 1usize << target; // target's bit position
    let n = state.amps.len(); // dimension of n

    for i in 0..n {
        if (i & bit) == 0 {
            // i is 0 in index bit
            let j = i ^ bit; // flip i in index bit (0 > 1)
            state.amps.swap(i, j);
        }
    }
}

// Hadamard
pub fn apply_h(state: &mut QState, target: usize) {
    let bit = 1usize << target; // target's bit position
    let n = state.amps.len(); // dimension of n

    for i in 0..n {
        if (i & bit) == 0 {
            // i is 0 in index bit

            let j = i ^ bit; // flip i in index bit (0 > 1)

            let a0 = state.amps[i];
            let a1 = state.amps[j];

            state.amps[i] = (a0 + a1) / SQRT_2;
            state.amps[j] = (a0 - a1) / SQRT_2;
        }
    }
}
