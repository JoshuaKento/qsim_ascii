/*
Implement render_probabilities in crates/qsim_tui/src/render.rs:
iterate state.probabilities()
format rows like |0> 1.0000, |1> 0.0000
return one multiline String
*/
pub fn render_probabilities(state: &qsim_core::state::QState) -> String {
    let probs = state.probabilities();
    let n = state.get_nqubits();

    probs
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let label = if n <= 1 {
                format!("|{}>", i)
            } else {
                format!("|{:0width$b}>", i, width = n)
            };

            format!("{label} {:.4}", p)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/*
Implement render_help in crates/qsim_tui/src/render.rs:
return a String containing at least [q] [h] [x] [m] [r]
*/
pub fn render_help() -> String {
    [
        "[q] quit",
        "[h] apply H gate",
        "[x] apply X gate",
        "[m] MeasureAll",
        "[r] reset",
    ]
    .join("")
}
