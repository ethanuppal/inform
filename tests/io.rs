// Copyright (C) 2024 Ethan Uppal. All rights reserved.
//
// This file is part of inform. inform is free software: you can redistribute it
// and/or modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version. inform is distributed in the hope that
// it will be useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details. You should have received a
// copy of the GNU Lesser General Public License along with inform. If not, see
// <https://www.gnu.org/licenses/>.

#![cfg(feature = "io")]

use core::str;
use std::io::{self, Write};

use inform::{common::IndentWriterCommon, io::IndentWriter};

#[derive(Default)]
struct FakeIOBuffer {
    string: String,
}

impl io::Write for FakeIOBuffer {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.string
            .push_str(str::from_utf8(buffer).map_err(io::Error::other)?);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn write_text(buffer: &mut FakeIOBuffer) -> io::Result<()> {
    let mut f = IndentWriter::new(buffer, 2);
    writeln!(f, "hello\ngoodbye")?;
    f.increase_indent();
    writeln!(f, "hello\ngoodbye")?;
    f.decrease_indent();
    writeln!(f, "hello\ngoodbye")
}

#[test]
fn test_indent_writer() {
    let mut buffer = FakeIOBuffer::default();
    write_text(&mut buffer).expect("failed to format");
    assert_eq!(
        "hello\ngoodbye\n  hello\n  goodbye\nhello\ngoodbye\n",
        buffer.string
    );
}
