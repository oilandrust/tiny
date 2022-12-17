extern crate libc;
extern crate termios;

use std::io;

use std::io::Read;

use termios::VMIN;
use termios::VTIME;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

const STDIN: i32 = 0;
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub struct Platform {
    stdin: io::Stdin,
    termios: Termios,
}

#[derive(PartialEq, Eq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Unknown,
}

impl Platform {
    pub fn new() -> Self {
        let termios = Termios::from_fd(STDIN).unwrap();
        let mut new_termios = termios;

        // no echo and canonical mode
        new_termios.c_lflag &= !(ICANON | ECHO);

        // set VMIN and VTIME to 0 not block on read when the buffer is empty.
        new_termios.c_cc[VMIN] = 0;
        new_termios.c_cc[VTIME] = 0;

        tcsetattr(STDIN, TCSANOW, &new_termios).unwrap();

        Platform {
            stdin: io::stdin(),
            termios,
        }
    }

    pub fn poll_input(&mut self) -> Option<char> {
        let mut buffer = [0; 1];
        self.stdin.read_exact(&mut buffer).ok()?;

        buffer.first().map(|c| *c as char)
    }

    pub fn clear_display() {
        print!("{CLEAR}");
    }

    pub fn translate_input(c: char) -> Key {
        match c {
            'a' | 'A' => Key::A,
            'b' | 'B' => Key::B,
            'c' | 'C' => Key::C,
            'd' | 'D' => Key::D,
            'e' | 'E' => Key::E,
            'f' | 'F' => Key::F,
            'g' | 'G' => Key::G,
            'h' | 'H' => Key::H,
            'i' | 'I' => Key::I,
            'j' | 'J' => Key::J,
            'k' | 'K' => Key::K,
            'l' | 'L' => Key::L,
            'm' | 'M' => Key::M,
            'n' | 'N' => Key::N,
            'o' | 'O' => Key::O,
            'p' | 'P' => Key::P,
            'q' | 'Q' => Key::Q,
            'r' | 'R' => Key::R,
            's' | 'S' => Key::S,
            't' | 'T' => Key::T,
            'u' | 'U' => Key::U,
            'v' | 'V' => Key::V,
            'w' | 'W' => Key::W,
            'x' | 'X' => Key::X,
            'y' | 'Y' => Key::Y,
            'z' | 'Z' => Key::Z,
            _ => todo!(),
        }
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
