// use std::io::{self, Write};
// use nix::unistd;
use std::process;
use std::io::{stdout, stderr, Write};
use std::time::Duration;
use crossterm::{
    execute,
    ExecutableCommand,
    terminal,
    terminal::{ScrollUp, SetSize, size, Clear, ClearType},
    style::{Print},
    event::{read, Event::*, KeyCode, KeyModifiers, poll}
};

fn die(s: &str) {
    eprintln!("Error: {}", s);
    stderr().flush().unwrap();
    process::exit(1);
}

fn editor_refresh_screen() -> Result<(), std::io::Error> {
    stdout().execute(Clear(ClearType::All))?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut _count = 0;

    terminal::enable_raw_mode()?;

    let (cols, rows) = size()?;

    execute!(
        stdout(),
        SetSize(10, 10),
        ScrollUp(5)
    )?;

    execute!(stdout(), SetSize(cols, rows))?;
    // Be a good citizen, cleanup
    execute!(
        stdout(),
        Print("Styled text here. \n"),
    )?;

    loop {
        _count += 1;
        if let Ok(true) = poll(Duration::from_millis(100)) {
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    if (key_event.code == KeyCode::Char('q')) &&
                        (key_event.modifiers == KeyModifiers::CONTROL) {
                        break;
                    } if (key_event.code == KeyCode::Char('r')) &&
                        (key_event.modifiers == KeyModifiers::CONTROL) {
                            editor_refresh_screen();
                        } else {
                        println!("{:?}\r", key_event);
                    }
                }
            } else {
                die("error");
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}

