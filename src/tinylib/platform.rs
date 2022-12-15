extern crate termios;

use std::io;
use std::io::Read;
use std::io::Write;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

const STDIN: i32 = 0;
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub struct Platform {
    stdout: io::Stdout,
    stdin: io::Stdin,
    termios: Termios,
}

impl Platform {
    pub fn new() -> Self {
        let termios = Termios::from_fd(STDIN).unwrap();
        let mut new_termios = termios;

        new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(STDIN, TCSANOW, &new_termios).unwrap();

        Platform {
            stdout: io::stdout(),
            stdin: io::stdin(),
            termios,
        }
    }

    pub fn read_char(&mut self) -> char {
        self.stdout.lock().flush().unwrap();

        let mut buffer = [0; 1];
        self.stdin.read_exact(&mut buffer).unwrap();

        *buffer.first().unwrap() as char
    }

    pub fn clear_display() {
        print!("{CLEAR}");
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        tcsetattr(STDIN, TCSANOW, &self.termios).unwrap();
    }
}
