use std::{io, str::SplitWhitespace};

// Valid Stdin API to update the app is:
// -------------------------------------
// data <minutes> <seconds> <SA-status>
// size <size>

pub enum Command {
    Time(String, String, bool),
    Size(f32),
    Color(String),
    Skip,
}

/// Read the next line from Stdin and decode it into a Command
pub fn get_next_command() -> io::Result<Command> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut decoded = input.trim().split_whitespace();

    Ok(decode_input(&mut decoded).unwrap_or(Command::Skip))
}

/// Decodes a SplitWhitspace iterator into a Command
fn decode_input(input: &mut SplitWhitespace) -> Option<Command> {
    match input.next() {
        Some("data") => {
            let minutes = input.next()?;
            let seconds = input.next()?;
            let sa_status = input.next()?;
            println!("{} {} {}", minutes, seconds, sa_status);
            Some(Command::Time(
                minutes.to_string(),
                seconds.to_string(),
                sa_status == "true",
            ))
        }
        Some("size") => {
            let size = input.next()?.parse::<f32>().ok()?;
            Some(Command::Size(size))
        }
        Some("cmap") => {
            let color = input.next()?;
            Some(Command::Color(color.to_string()))
        }
        _ => None,
    }
}
