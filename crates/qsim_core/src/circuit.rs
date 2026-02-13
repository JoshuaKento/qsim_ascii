use crate::{
    gates::{apply_h, apply_x},
    state::QState,
};

// for queing operation
pub enum Operation {
    X(usize),
    H(usize),
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

    // run
    pub fn run(&self, state: &mut QState) {
        for i in &self.ops {
            match *i {
                Operation::H(t) => apply_h(state, t),
                Operation::X(t) => apply_x(state, t),
            }
        }
    }
}
