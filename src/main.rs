use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const GREETING: &str = "Hello from Rust in Docker! 🦀";

fn write_greeting(mut output: impl Write) -> io::Result<()> {
    writeln!(output, "{GREETING}")?;
    output.flush()
}

fn main() {
    write_greeting(io::stdout().lock()).expect("failed to write the greeting");
    thread::sleep(Duration::from_millis(200));
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FailingWriter {
        fail_on_write: bool,
    }

    impl Write for FailingWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_on_write {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "write failed"))
            } else {
                Ok(bytes.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("flush failed"))
        }
    }

    #[test]
    fn writes_exact_utf8_greeting() {
        let mut buffer = Vec::new();
        write_greeting(&mut buffer).unwrap();
        assert_eq!(buffer, "Hello from Rust in Docker! 🦀\n".as_bytes());
    }

    #[test]
    fn propagates_write_failure() {
        let error = write_greeting(FailingWriter {
            fail_on_write: true,
        })
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
    }

    #[test]
    fn propagates_flush_failure() {
        let error = write_greeting(FailingWriter {
            fail_on_write: false,
        })
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::Other);
    }
}
