use std::f64::consts::SQRT_2;

use crate::state::QState;

/* ToDo HIGH : Target value validation
High apply_x/apply_h can panic on invalid target(out-of-range bit index),
because they compute j and index/swap without bounds checks. */

// Pauli-X flip
pub fn apply_x(state: &mut QState, target: usize) {
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
