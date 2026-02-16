use qsim_core::{circuit::Circuit, state::QState};

fn main() {
    let nqbits = 2;
    let mut state = QState::zero(nqbits);
    let mut circuit = Circuit::new();

    circuit.h(0).cnot(0, 1);
    circuit.run(&mut state);

    println!("{:?}", state.probabilities());
}
