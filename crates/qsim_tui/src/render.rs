pub fn render_probabilities(state: &qsim_core::state::QState) -> String {
    render_probabilities_internal(state, None)
}

pub fn render_probabilities_with_targets(
    state: &qsim_core::state::QState,
    selected_target: usize,
) -> String {
    render_probabilities_internal(state, Some(selected_target))
}

pub fn render_amplitudes_with_targets(
    state: &qsim_core::state::QState,
    selected_target: usize,
) -> String {
    let amps = state.amps();
    let n = state.get_nqbits();
    const AMP_HALF_WIDTH: usize = 16;

    let rows = amps
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let label = basis_label(i, n);
            let target_cols = target_columns(i, n, Some(selected_target));
            let re = a.re.clamp(-1.0, 1.0);
            let im = a.im.clamp(-1.0, 1.0);

            let re_bar = render_signed_gradient_bar(re, AMP_HALF_WIDTH);
            let im_bar = render_signed_gradient_bar(im, AMP_HALF_WIDTH);

            let indent = " ".repeat(label.len() + target_cols.len() + 1);

            format!(
                "{label}{target_cols} Re {:+.4} |{re_bar}|\n{indent}Im {:+.4} |{im_bar}|",
                a.re, a.im
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{rows}\n\nGraph\n  Re : {}\n  Im : {}\n  idx: 0..{}",
        render_sparkline(amps.iter().map(|a| a.re)),
        render_sparkline(amps.iter().map(|a| a.im)),
        amps.len().saturating_sub(1)
    )
}

fn render_probabilities_internal(
    state: &qsim_core::state::QState,
    selected_target: Option<usize>,
) -> String {
    let probs = state.probabilities();
    let n = state.get_nqbits();
    const BAR_WIDTH: usize = 32;

    let rows = probs
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let label = basis_label(i, n);
            let target_cols = target_columns(i, n, selected_target);

            let clamped = p.clamp(0.0, 1.0);
            let bar = render_unit_gradient_bar(clamped, BAR_WIDTH);
            let pct = clamped * 100.0;

            format!("{label} {:.4}{target_cols} ({:>6.2}%) |{bar}|", p, pct)
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{rows}\n\nGraph\n  prob: {}\n  idx : 0..{}",
        render_sparkline(probs.iter().copied()),
        probs.len().saturating_sub(1)
    )
}

fn basis_label(index: usize, nqubits: usize) -> String {
    if nqubits <= 1 {
        format!("|{}>", index)
    } else {
        format!("|{:0width$b}>", index, width = nqubits)
    }
}

fn target_columns(index: usize, nqubits: usize, selected_target: Option<usize>) -> String {
    match selected_target {
        Some(t) if t < nqubits && nqubits >= 2 => {
            let c = (t + 1) % nqubits;
            let t_bit = (index >> t) & 1;
            let c_bit = (index >> c) & 1;
            format!("  t[q{}]={} c[q{}]={}", t, t_bit, c, c_bit)
        }
        Some(t) if t < nqubits => {
            let t_bit = (index >> t) & 1;
            format!("  t[q{}]={}", t, t_bit)
        }
        _ => String::new(),
    }
}

fn render_unit_gradient_bar(prob: f64, width: usize) -> String {
    let total_subcells = (prob * (width as f64) * 8.0).round() as isize;
    let mut bar = String::with_capacity(width);

    for col in 0..width {
        let col_start = (col * 8) as isize;
        let fill = (total_subcells - col_start).clamp(0, 8) as usize;
        bar.push(subcell_char(fill));
    }

    bar
}

fn render_signed_gradient_bar(value: f64, half_width: usize) -> String {
    let magnitude = value.abs().clamp(0.0, 1.0);
    let mut left = " ".repeat(half_width);
    let mut right = " ".repeat(half_width);

    if value >= 0.0 {
        right = render_segment_from_center(magnitude, half_width, true);
    } else {
        left = render_segment_from_center(magnitude, half_width, false);
    }

    format!("{left}|{right}")
}

fn render_segment_from_center(magnitude: f64, width: usize, to_right: bool) -> String {
    let total_subcells = (magnitude * (width as f64) * 8.0).round() as isize;
    let mut s = String::with_capacity(width);

    for col in 0..width {
        let dist = if to_right { col } else { width - 1 - col };
        let col_start = (dist * 8) as isize;
        let fill = (total_subcells - col_start).clamp(0, 8) as usize;
        s.push(subcell_char(fill));
    }

    s
}

fn render_sparkline(values: impl Iterator<Item = f64>) -> String {
    values
        .map(|v| {
            let clamped = v.abs().clamp(0.0, 1.0);
            let level = (clamped * 8.0).round() as usize;
            subcell_char(level)
        })
        .collect()
}

fn subcell_char(fill: usize) -> char {
    match fill {
        0 => ' ',
        1 => '.',
        2 => ':',
        3 => '-',
        4 => '=',
        5 => '+',
        6 => '*',
        7 => '#',
        _ => '@',
    }
}

pub fn render_help() -> String {
    [
        "Controls",
        "  [a] toggle render mode (prob/amplitude)",
        "  [0-9] select target qubit",
        "  [h] Hadamard on target",
        "  [x] X on target",
        "  [c] CNOT (target -> next qubit)",
        "  [m] measure all",
        "  [r] reset to |0...0>",
        "  [v] toggle recording",
        "  [p] play recorded macro (animated)",
        "  [k] clear recorded macro",
        "  [q] quit",
    ]
    .join("\n")
}
