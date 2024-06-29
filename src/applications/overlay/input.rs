use std::sync::{Arc, Mutex};
use std::{io, str::SplitWhitespace};

pub enum Command {
    Time(String, String),
    Size(f32),
    Skip,
}

/// Polls for new commands from Stdin and updates shared variables
/// Allows the blocking call to Stdin to be handled in a separate thread
pub fn thread_reader(timer: Arc<Mutex<String>>, size: Arc<Mutex<f32>>) {
    loop {
        match get_next_command() {
            Ok(Command::Time(minutes, seconds)) => {
                *timer.lock().unwrap() = format!("{}:{}", minutes, seconds);
            }
            Ok(Command::Size(timer_size)) => {
                *size.lock().unwrap() = timer_size;
            }
            _ => {}
        }
    }
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
        Some("time") => {
            let minutes = input.next()?;
            let seconds = input.next()?;
            Some(Command::Time(minutes.to_string(), seconds.to_string()))
        }
        Some("size") => {
            let size = input.next()?.parse::<f32>().ok()?;
            Some(Command::Size(size))
        }
        _ => None,
    }
}
