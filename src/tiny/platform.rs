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

struct NonblockingBufReader {
    buffered: BufReader<RawFd2>,
}

struct RawFd2 {
    fd: RawFd,
}

impl NonblockingBufReader {
    /// Takes ownership of the underlying FD
    fn new<R: IntoRawFd>(underlying: R) -> NonblockingBufReader {
        let buffered = BufReader::new(RawFd2 {
            fd: underlying.into_raw_fd(),
        });
        return NonblockingBufReader { buffered };
        // Here's a private implementation of 'Read' for raw file descriptors, for use in BufReader...
        impl std::io::Read for RawFd2 {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                assert!(buf.len() <= isize::max_value() as usize);
                match unsafe { libc::read(self.fd, buf.as_mut_ptr() as _, buf.len()) } {
                    x if x < 0 => Err(std::io::Error::last_os_error()),
                    x => Ok(x as usize),
                }
            }
        }
    }

    /// Wraps BufReader::read_line.
    /// Possible outcomes: (1) data, (2) EOF, (3) Error
    fn read_line(&mut self) -> std::io::Result<String> {
        let mut line = String::new();
        match self.buffered.read_line(&mut line) {
            Ok(_) => Ok(line), // EOF/data
            Err(e) => Err(e),  // Error
        }
    }

    /// Does BufReader::read_line but only if there's already at least one byte
    /// available on the FD. In case of EOF, returns an empty string.
    /// Possible outcomes: (0) no-data-yet, (1) data, (2) EOF, (3) Error
    fn read_line_only_if_data(&mut self) -> std::io::Result<Option<String>> {
        let r = unsafe {
            // The reason this is safe is we know 'inner' wraps a valid FD,
            // and we're not doing any reads on it such as would disturb BufReader.
            let fd = self.buffered.get_ref().fd;
            let flags = libc::fcntl(fd, libc::F_GETFL);
            libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
            let r = self.buffered.fill_buf();
            libc::fcntl(fd, libc::F_SETFL, flags);
            r
        };
        // Behavior of fill_buf is "Returns the contents of the internal buffer, filling it with more
        // data from the inner reader if it is empty." If there were no bytes available, then (1) the
        // internal buffer is empty, (2) it'll call inner.read(), (3) that call will error WouldBlock.
        match r {
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None), // (0) no-data-yet
            Ok(buf) if !buf.is_empty() => {
                let mut line = String::new();
                self.buffered.read_line(&mut line)?;
                Ok(Some(line)) // (1) data, or further error
            }
            Ok(_) => Ok(Some(String::new())), // (2) EOF
            Err(e) => Err(e),                 // (3) Error
        }
    }
}
