use qsim_core::{circuit::Circuit, state::QState};

fn main() {
    let nqbits = 1;
    let mut state = QState::zero(nqbits);
    let mut circuit = Circuit::new();

    circuit.h(0).x(0).h(0);
    circuit.run(&mut state);

    println!("{:?}", state.probabilities());
}
