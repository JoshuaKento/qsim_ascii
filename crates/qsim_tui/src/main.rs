mod render;
mod term;

use qsim_core::state::QState;

use rand::SeedableRng;

use crate::{
    render::{render_help, render_probabilities},
    term::{Command, map_input_to_command},
};

pub struct AppState {
    pub nqbits: usize,
    pub state: QState,
}

pub fn new_app(nqbits: usize) -> AppState {
    AppState {
        nqbits,
        state: QState::zero(nqbits),
    }
}

pub fn handle_key(app: &mut AppState, key: char) -> bool {
    let Some(cmd) = map_input_to_command(key) else {
        // unmapped key: do nothing
        return false;
    };

    match cmd {
        Command::Quit => true,

        Command::ApplyH(t) => {
            qsim_core::gates::apply_h(&mut app.state, t);
            false
        }

        Command::ApplyX(t) => {
            qsim_core::gates::apply_x(&mut app.state, t);
            false
        }

        Command::MeasureAll => {
            qsim_core::measure::measure_all(
                &mut app.state,
                &mut rand::rngs::StdRng::seed_from_u64(7),
            );
            false
        }

        Command::Reset => {
            let n = app.state.get_nqbits();
            app.state = QState::zero(n);
            false
        }
    }
}

pub fn render_screen(app: &AppState) -> String {
    let probs = render_probabilities(&app.state);
    let help = render_help();

    format!("{probs}\n\n{help}")
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
    }

    frames
}

fn main() {
    use std::io::Write;

    let nqbits = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| n > 0 && n <= 12)
        .unwrap_or(1);

    let mut app = new_app(nqbits);
    let mut out = std::io::stdout();
    let mut needs_redraw = true;

    if let Err(err) = crossterm::terminal::enable_raw_mode() {
        eprintln!("failed to enable raw mode: {err}");
        return;
    }

    if let Err(err) = crossterm::execute!(out, crossterm::terminal::EnterAlternateScreen) {
        let _ = crossterm::terminal::disable_raw_mode();
        eprintln!("failed to enter alternate screen: {err}");
        return;
    }

    let run_result: std::io::Result<()> = (|| {
        crossterm::execute!(out, crossterm::cursor::Hide)?;
        loop {
            if needs_redraw {
                crossterm::queue!(
                    out,
                    crossterm::cursor::MoveTo(0, 0),
                    crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                    crossterm::style::Print(render_screen(&app))
                )?;
                out.flush()?;
                needs_redraw = false;
            }

            if !crossterm::event::poll(std::time::Duration::from_millis(250))? {
                continue;
            }

            match crossterm::event::read()? {
                crossterm::event::Event::Key(key)
                    if key.kind == crossterm::event::KeyEventKind::Press =>
                {
                    match key.code {
                        crossterm::event::KeyCode::Esc => break,
                        crossterm::event::KeyCode::Char(ch) => {
                            if handle_key(&mut app, ch.to_ascii_lowercase()) {
                                break;
                            }
                            needs_redraw = true;
                        }
                        _ => {}
                    }
                }
                crossterm::event::Event::Resize(_, _) => needs_redraw = true,
                _ => {}
            }
        }
        Ok(())
    })();

    let _ = crossterm::execute!(
        out,
        crossterm::cursor::Show,
        crossterm::terminal::LeaveAlternateScreen
    );
    //let _ = crossterm::terminal::disable_raw_mode();

    if let Err(err) = run_result {
        eprintln!("qsim_tui runtime error: {err}");
    }
}
