pub enum Command {
    Quit,
    ApplyH(usize),
    ApplyX(usize),
    MeasureAll,
    Reset,
}

pub fn map_input_to_command(key: char) -> Option<Command> {
    match key {
        'q' => Some(Command::Quit),
        'h' => Some(Command::ApplyH(0)),
        'x' => Some(Command::ApplyX(0)),
        'm' => Some(Command::MeasureAll),
        'r' => Some(Command::Reset),
        _ => None,
    }
}
