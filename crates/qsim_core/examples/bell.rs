use qsim_core::{Circuit, QState, measure_all};
use rand::{SeedableRng, rngs::StdRng};

fn main() {
    let shots = 10_000usize;
    let mut rng = StdRng::seed_from_u64(2026);
    let mut counts = [0usize; 4];
    let mut entangled_checks = 0usize;

    for _ in 0..shots {
        let mut state = QState::zero(2);
        let mut circuit = Circuit::new(2);
        circuit.h(0).cnot(0, 1);
        circuit.run(&mut state);

        if state.is_entangled_two_qubit() {
            entangled_checks += 1;
        }

        let outcome = measure_all(&mut state, &mut rng);
        counts[outcome] += 1;
    }

    println!("Bell-state entanglement checks passed: {entangled_checks}/{shots}");
    println!("Measured counts [|00>, |01>, |10>, |11>]: {:?}", counts);
}
