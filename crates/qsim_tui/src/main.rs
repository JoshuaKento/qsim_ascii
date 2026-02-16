mod render;
mod term;

use qsim_core::state::QState;

use self::{
    render::{render_amplitudes_with_targets, render_help, render_probabilities_with_targets},
    term::{Command, TerminalEvent, map_input_to_command},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMode {
    Probabilities,
    Amplitudes,
}

pub struct AppState {
    pub nqbits: usize,
    pub selected_target: usize,
    pub state: QState,
    pub render_mode: RenderMode,
    pub is_recording: bool,
    pub recorded_keys: Vec<char>,
    pub playback_queue: Vec<char>,
    pub playback_index: usize,
    pub playback_running: bool,
}

fn reset_to_even_chance(app: &mut AppState) {
    let n = app.state.get_nqbits();
    app.state = QState::zero(n);
    app.selected_target = 0;
    for target in 0..n {
        qsim_core::gates::apply_h(&mut app.state, target);
    }
}

fn reset_to_basis_zero(app: &mut AppState) {
    let n = app.state.get_nqbits();
    app.state = QState::zero(n);
    app.selected_target = 0;
}

pub fn new_app(nqbits: usize) -> AppState {
    AppState {
        nqbits,
        selected_target: 0,
        state: QState::zero(nqbits),
        render_mode: RenderMode::Probabilities,
        is_recording: false,
        recorded_keys: Vec::new(),
        playback_queue: Vec::new(),
        playback_index: 0,
        playback_running: false,
    }
}

pub fn handle_key(app: &mut AppState, key: char) -> bool {
    let Some(cmd) = map_input_to_command(key) else {
        // unmapped key: do nothing
        return false;
    };

    match cmd {
        Command::ToggleRenderMode => {
            app.render_mode = match app.render_mode {
                RenderMode::Probabilities => RenderMode::Amplitudes,
                RenderMode::Amplitudes => RenderMode::Probabilities,
            };
            return false;
        }
        Command::ToggleRecord => {
            app.is_recording = !app.is_recording;
            return false;
        }
        Command::PlayRecording => {
            if app.playback_running {
                app.playback_running = false;
                app.playback_index = 0;
                return false;
            }

            if app.recorded_keys.is_empty() {
                return false;
            }
            app.playback_queue = app.recorded_keys.clone();
            app.playback_index = 0;
            app.playback_running = true;
            return false;
        }
        Command::ClearRecording => {
            app.recorded_keys.clear();
            app.playback_queue.clear();
            app.playback_index = 0;
            app.playback_running = false;
            return false;
        }
        Command::Quit => return true,
        _ => {}
    }

    if app.is_recording && should_record_command(cmd) {
        app.recorded_keys.push(key);
    }

    execute_command(app, cmd)
}

fn should_record_command(cmd: Command) -> bool {
    matches!(
        cmd,
        Command::ApplyH(_)
            | Command::ApplyX(_)
            | Command::ApplyCnot
            | Command::MeasureAll
            | Command::Reset
            | Command::SelectTarget(_)
    )
}

fn playback_tick(app: &mut AppState) -> bool {
    if !app.playback_running {
        return false;
    }

    loop {
        if app.playback_index >= app.playback_queue.len() {
            app.playback_running = false;
            return false;
        }

        let key = app.playback_queue[app.playback_index];
        app.playback_index += 1;

        let Some(cmd) = map_input_to_command(key) else {
            continue;
        };

        if matches!(
            cmd,
            Command::Quit
                | Command::ToggleRecord
                | Command::PlayRecording
                | Command::ClearRecording
                | Command::ToggleRenderMode
        ) {
            continue;
        }

        let _ = execute_command(app, cmd);
        return true;
    }
}

fn execute_command(app: &mut AppState, cmd: Command) -> bool {
    match cmd {
        Command::Quit => true,
        Command::ApplyH(_) => {
            qsim_core::gates::apply_h(&mut app.state, app.selected_target);
            false
        }
        Command::ApplyX(_) => {
            qsim_core::gates::apply_x(&mut app.state, app.selected_target);
            false
        }
        Command::ApplyCnot => {
            if app.nqbits >= 2 {
                let control = app.selected_target;
                let target = (control + 1) % app.nqbits;
                qsim_core::gates::apply_cnot(&mut app.state, control, target);
            }
            false
        }
        Command::MeasureAll => {
            qsim_core::measure::measure_all_with_thread_rng(&mut app.state);
            false
        }
        Command::Reset => {
            reset_to_basis_zero(app);
            false
        }
        Command::SelectTarget(t) => {
            if t < app.nqbits {
                app.selected_target = t;
            }
            false
        }
        Command::ToggleRenderMode
        | Command::ToggleRecord
        | Command::PlayRecording
        | Command::ClearRecording => false,
    }
}

pub fn render_screen(app: &AppState) -> String {
    render_screen_for_height(app, usize::MAX)
}

fn render_screen_for_height(app: &AppState, max_lines: usize) -> String {
    let view = match app.render_mode {
        RenderMode::Probabilities => {
            render_probabilities_with_targets(&app.state, app.selected_target)
        }
        RenderMode::Amplitudes => render_amplitudes_with_targets(&app.state, app.selected_target),
    };
    let help = render_help();
    let state_probs = app.state.probabilities();
    let total_prob: f64 = state_probs.iter().sum();
    let active_states = state_probs.iter().filter(|p| **p > 1e-9).count();
    let cnot_target = if app.nqbits <= 1 {
        "n/a".to_string()
    } else {
        format!("q{}", (app.selected_target + 1) % app.nqbits)
    };
    let cnot_route = if app.nqbits <= 1 {
        "CNOT route: unavailable (need >= 2 qubits)".to_string()
    } else {
        format!("CNOT route: q{} -> {}", app.selected_target, &cnot_target)
    };
    let render_mode = match app.render_mode {
        RenderMode::Probabilities => "probabilities",
        RenderMode::Amplitudes => "amplitudes (Re/Im)",
    };
    let record_status = if app.is_recording { "ON" } else { "OFF" };
    let playback_status = if app.playback_running {
        format!(
            "PLAYING ({}/{})",
            app.playback_index.min(app.playback_queue.len()),
            app.playback_queue.len()
        )
    } else {
        "idle".to_string()
    };

    let screen = format!(
        "======================= QSIM ASCII =======================\n\
qubits: {}    target: q{}    cnot target: {}    active basis states: {}\n\
view: {}    record: {}    recorded keys: {}    playback: {}\n\
{}\n\
total probability: {:.4}\n\
----------------------------------------------------------\n\
{}\n\
----------------------------------------------------------\n\
{}",
        app.nqbits,
        app.selected_target,
        cnot_target,
        active_states,
        render_mode,
        record_status,
        app.recorded_keys.len(),
        playback_status,
        cnot_route,
        total_prob,
        view,
        help
    );

    fit_to_height(&screen, max_lines)
}

fn fit_to_height(screen: &str, max_lines: usize) -> String {
    if max_lines == usize::MAX {
        return screen.to_string();
    }

    let lines: Vec<&str> = screen.lines().collect();
    if lines.len() <= max_lines {
        return screen.to_string();
    }

    if max_lines <= 4 {
        return lines
            .into_iter()
            .take(max_lines)
            .collect::<Vec<_>>()
            .join("\n");
    }

    let tail_keep = ((max_lines / 3).max(3)).min(10).min(max_lines - 2);
    let head_keep = max_lines - tail_keep - 1;
    let omitted = lines.len().saturating_sub(head_keep + tail_keep);

    let mut out: Vec<String> = Vec::with_capacity(max_lines);
    for line in lines.iter().take(head_keep) {
        out.push((*line).to_string());
    }
    out.push(format!(
        "... ({omitted} lines omitted; press [a] to switch view or reduce qubits)"
    ));
    for line in lines.iter().skip(lines.len().saturating_sub(tail_keep)) {
        out.push((*line).to_string());
    }

    out.join("\n")
}

pub fn run_scripted_session(nqbits: usize, keys: &[char]) -> Vec<String> {
    let mut app = new_app(nqbits);
    let mut frames = Vec::with_capacity(keys.len() + 1);

    // initial screen
    frames.push(render_screen(&app));

    for &key in keys {
        let should_quit = handle_key(&mut app, key);
        frames.push(render_screen(&app));

        if should_quit {
            break;
        }

        while playback_tick(&mut app) {
            frames.push(render_screen(&app));
        }
    }

    while playback_tick(&mut app) {
        frames.push(render_screen(&app));
    }

    frames
}

fn main() {
    let nqbits = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n > 0 && n <= 12)
        .unwrap_or(1);

    let mut app = new_app(nqbits);
    reset_to_even_chance(&mut app);
    let mut out = std::io::stdout();
    let mut needs_redraw = true;

    if let Err(err) = term::enter_terminal(&mut out) {
        eprintln!("failed to initialize terminal: {err}");
        return;
    }

    let run_result: std::io::Result<()> = (|| {
        loop {
            if needs_redraw {
                let term_height = term::terminal_height();
                term::draw_screen(&mut out, &render_screen_for_height(&app, term_height))?;
                needs_redraw = false;
            }

            let poll_timeout = if app.playback_running {
                std::time::Duration::from_millis(0)
            } else {
                std::time::Duration::from_millis(250)
            };

            if let Some(event) = term::poll_event(poll_timeout)? {
                match event {
                    TerminalEvent::Esc => break,
                    TerminalEvent::Key(ch) => {
                        if handle_key(&mut app, ch) {
                            break;
                        }
                        needs_redraw = true;
                    }
                    TerminalEvent::Resize => needs_redraw = true,
                }
            }

            if playback_tick(&mut app) {
                needs_redraw = true;
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
        Ok(())
    })();

    let _ = term::leave_terminal(&mut out);

    if let Err(err) = run_result {
        eprintln!("qsim_tui runtime error: {err}");
    }
}
