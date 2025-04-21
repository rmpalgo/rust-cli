use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

pub fn read_line() -> io::Result<String> {
    let mut line = String::new();
    loop {
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event::read()?
        {
            match code {
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char(c) => {
                    line.push(c);
                }
                _ => {}
            }
        }
    }

    Ok(line)
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    
    println!("Enter text (type 'q' on a line by itself to quit):\r");
    
    loop {
        let input = read_line()?;
        println!("You typed: {}\r", input);
        
        if input == "q" {
            break;
        }
    }

    terminal::disable_raw_mode()
}