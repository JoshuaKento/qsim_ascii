pub fn render_probabilities(state: &qsim_core::state::QState) -> String {
    let probs = state.probabilities();
    let n = state.get_nqbits();

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

pub fn render_help() -> String {
    [
        "[q] quit",
        "[h] apply H gate",
        "[x] apply X gate",
        "[m] MeasureAll",
        "[r] reset",
    ]
    .join("\n")
}
