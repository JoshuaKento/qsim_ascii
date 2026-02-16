use crate::{
    gates::{apply_cnot, apply_h, apply_x},
    state::QState,
};

// For queuing operations.
pub enum Operation {
    X(usize),
    H(usize),
    CNOT(usize, usize),
}

// Builder, constructor, runner.
pub struct Circuit {
    nqubits: usize,
    ops: Vec<Operation>,
}

impl Circuit {
    // Constructor.
    pub fn new(nqubits: usize) -> Self {
        assert!(nqubits > 0, "circuit qubit width must be >= 1");
        Self {
            nqubits,
            ops: Vec::new(),
        }
    }

    fn assert_target_in_range(&self, target: usize) {
        assert!(
            target < self.nqubits,
            "target qubit index out of range: target={target}, nqubits={}",
            self.nqubits
        );
    }

    fn assert_control_target_in_range(&self, control: usize, target: usize) {
        assert!(
            control < self.nqubits,
            "control qubit index out of range: control={control}, nqubits={}",
            self.nqubits
        );
        assert!(
            target < self.nqubits,
            "target qubit index out of range: target={target}, nqubits={}",
            self.nqubits
        );
        assert!(control != target, "control and target must be different");
    }

    // Builder for X.
    pub fn x(&mut self, target: usize) -> &mut Self {
        self.assert_target_in_range(target);
        self.ops.push(Operation::X(target));
        self
    }

    // Builder for H.
    pub fn h(&mut self, target: usize) -> &mut Self {
        self.assert_target_in_range(target);
        self.ops.push(Operation::H(target));
        self
    }

    // Builder for CNOT.
    pub fn cnot(&mut self, control: usize, target: usize) -> &mut Self {
        self.assert_control_target_in_range(control, target);
        self.ops.push(Operation::CNOT(control, target));
        self
    }

    // Run.
    pub fn run(&self, state: &mut QState) {
        assert!(
            state.get_nqbits() == self.nqubits,
            "circuit/state width mismatch: circuit={}, state={}",
            self.nqubits,
            state.get_nqbits()
        );

        for i in &self.ops {
            match *i {
                Operation::H(t) => apply_h(state, t),
                Operation::X(t) => apply_x(state, t),
                Operation::CNOT(c, t) => apply_cnot(state, c, t),
            }
        }
    }
}
