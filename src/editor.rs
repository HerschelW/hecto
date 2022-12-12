use std::fs::DirEntry;
use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}
impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        // loop
        loop {
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }
    pub fn default() -> Self {
        Self {}
    }
    fn process_keypress(&self) -> Result<(), io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program End"),
            _ => (),
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
