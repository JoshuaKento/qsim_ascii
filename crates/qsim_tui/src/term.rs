use std::{
    io::{self, Write},
    time::Duration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Quit,
    ApplyH(usize),
    ApplyX(usize),
    ApplyCnot,
    MeasureAll,
    Reset,
    SelectTarget(usize),
    ToggleRenderMode,
    ToggleRecord,
    PlayRecording,
    ClearRecording,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalEvent {
    Key(char),
    Esc,
    Resize,
}

pub fn enter_terminal(out: &mut io::Stdout) -> io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    if let Err(err) = crossterm::execute!(
        out,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    ) {
        let _ = crossterm::terminal::disable_raw_mode();
        return Err(err);
    }
    Ok(())
}

pub fn leave_terminal(out: &mut io::Stdout) -> io::Result<()> {
    let leave_result = crossterm::execute!(
        out,
        crossterm::cursor::Show,
        crossterm::terminal::LeaveAlternateScreen
    );
    let raw_mode_result = crossterm::terminal::disable_raw_mode();
    leave_result?;
    raw_mode_result?;
    Ok(())
}

pub fn terminal_height() -> usize {
    crossterm::terminal::size()
        .map(|(_, h)| h as usize)
        .unwrap_or(40)
        .saturating_sub(1)
        .max(1)
}

pub fn draw_screen(out: &mut io::Stdout, screen: &str) -> io::Result<()> {
    crossterm::queue!(
        out,
        crossterm::cursor::MoveTo(0, 0),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::style::Print(screen)
    )?;
    out.flush()
}

pub fn poll_event(timeout: Duration) -> io::Result<Option<TerminalEvent>> {
    if !crossterm::event::poll(timeout)? {
        return Ok(None);
    }

    match crossterm::event::read()? {
        crossterm::event::Event::Key(key) if key.kind == crossterm::event::KeyEventKind::Press => {
            match key.code {
                crossterm::event::KeyCode::Esc => Ok(Some(TerminalEvent::Esc)),
                crossterm::event::KeyCode::Char(ch) => {
                    Ok(Some(TerminalEvent::Key(ch.to_ascii_lowercase())))
                }
                _ => Ok(None),
            }
        }
        crossterm::event::Event::Resize(_, _) => Ok(Some(TerminalEvent::Resize)),
        _ => Ok(None),
    }
}

pub fn map_input_to_command(key: char) -> Option<Command> {
    match key {
        'q' => Some(Command::Quit),
        'h' => Some(Command::ApplyH(0)),
        'x' => Some(Command::ApplyX(0)),
        'c' => Some(Command::ApplyCnot),
        'm' => Some(Command::MeasureAll),
        'r' => Some(Command::Reset),
        'a' => Some(Command::ToggleRenderMode),
        'v' => Some(Command::ToggleRecord),
        'p' => Some(Command::PlayRecording),
        'k' => Some(Command::ClearRecording),
        '0'..='9' => Some(Command::SelectTarget((key as u8 - b'0') as usize)),
        _ => None,
    }
}
