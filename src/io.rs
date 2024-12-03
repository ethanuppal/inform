use core::str;
use std::io;

pub struct IndentFormatter<'buffer> {
    current_indent: String,
    add_indent: usize,
    last_was_newline: bool,
    buffer: &'buffer mut dyn io::Write,
}

impl<'buffer> IndentFormatter<'buffer> {
    /// Constructs a new formatter managing indents of `indent` spaces in the
    /// wrapped formatter `formatter`.
    pub fn new(buffer: &'buffer mut dyn io::Write, indent: usize) -> Self {
        Self {
            current_indent: String::new(),
            add_indent: indent,
            last_was_newline: false,
            buffer,
        }
    }

    /// Adds a level of indentation.
    pub fn increase_indent(&mut self) {
        for _ in 0..self.add_indent {
            self.current_indent.push(' ');
        }
    }

    /// Removes a level of indentation.
    pub fn decrease_indent(&mut self) {
        for _ in 0..self.add_indent {
            self.current_indent.pop();
        }
    }
}

impl io::Write for IndentFormatter<'_> {
    fn write(&mut self, str: &[u8]) -> io::Result<usize> {
        for c in str::from_utf8(str).map_err(io::Error::other)?.chars() {
            if self.last_was_newline {
                self.buffer.write_all(self.current_indent.as_bytes())?;
            }
            self.buffer.write_all(c.to_string().as_bytes())?;
            self.last_was_newline = c == '\n';
        }
        Ok(str.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}
