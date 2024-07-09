use core::fmt;

pub struct IndentFormatter<'a, 'b: 'a> {
    current_indent: String,
    add_indent: usize,
    last_was_newline: bool,
    formatter: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b: 'a> IndentFormatter<'a, 'b> {
    /// Constructs a new formatter managing indents of `indent` spaces in the
    /// wrapped formatter `formatter`.
    pub fn new(formatter: &'a mut fmt::Formatter<'b>, indent: usize) -> Self {
        Self {
            current_indent: String::new(),
            add_indent: indent,
            last_was_newline: false,
            formatter,
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

impl fmt::Write for IndentFormatter<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if self.last_was_newline {
                self.formatter.write_str(&self.current_indent)?;
            }
            self.formatter.write_char(c)?;
            self.last_was_newline = c == '\n';
        }
        Ok(())
    }
}
