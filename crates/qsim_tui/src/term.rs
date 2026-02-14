pub enum Command {
    Quit,
    ApplyH(usize),
    ApplyX(usize),
    MeasureAll,
    Reset,
}

/*
mplement Command and input mapping in crates/qsim_tui/src/term.rs:
enum Command { Quit, ApplyH(usize), ApplyX(usize), MeasureAll, Reset }
map:
'q' -> Quit
'h' -> ApplyH(0)
'x' -> ApplyX(0)
'm' -> MeasureAll
'r' -> Reset
anything else -> None
*/
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
