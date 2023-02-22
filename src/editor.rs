use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}
impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        // loop
        loop {
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
        }
    }
    pub fn default() -> Self {
        Self { should_quit: false }
    }
    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = read_key()?;
        if let Key::Ctrl('q') = pressed_key {
            self.should_quit = true;
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, io::Error> {
    let stdin = io::stdin();
    let mut keys = stdin.keys();
    match keys.next() {
        Some(key) => key,
        None => Err(io::Error::new(io::ErrorKind::Other, "Failed to read key")),
    }
}

fn die(e: &std::io::Error) {
    panic!("{}", e)
}
