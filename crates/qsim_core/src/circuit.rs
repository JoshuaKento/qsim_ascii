use crate::{
    gates::{apply_cnot, apply_h, apply_x},
    state::QState,
};

// for queing operation
pub enum Operation {
    X(usize),
    H(usize),
    CNOT(usize, usize),
}

// builder, constructer, runner
pub struct Circuit {
    nqubits: usize,
    ops: Vec<Operation>,
}

impl Circuit {
    // constructer
    pub fn new(nqubits: usize) -> Self {
        Circuit {
            nqubits,
            ops: Vec::new(),
        }
    }

    // builder for x
    pub fn x(&mut self, target: usize) -> &mut Self {
        self.ops.push(Operation::X(target));

        return self;
    }

    // builder for h
    pub fn h(&mut self, target: usize) -> &mut Self {
        self.ops.push(Operation::H(target));

        return self;
    }

    // bilder for cnot
    pub fn cnot(&mut self, control: usize, target: usize) -> &mut Self {
        self.ops.push(Operation::CNOT(control, target));
        return self;
    }

    // run
    /// ToDo: HIGH validate state.nqubits == self.nqubits
    pub fn run(&self, state: &mut QState) {
        for i in &self.ops {
            match *i {
                Operation::H(t) => apply_h(state, t),
                Operation::X(t) => apply_x(state, t),
                Operation::CNOT(c, t) => apply_cnot(state, c, t),
            }
        }
    }
}
