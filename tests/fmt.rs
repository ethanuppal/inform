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

#![cfg(feature = "fmt")]

use std::fmt::{self, Write};

use inform::{
    common::IndentWriterCommon,
    fmt::{IndentFormatter, IndentWriter},
};

fn write_text(str: &mut String) -> fmt::Result {
    let mut f = IndentWriter::new(str, 2);
    writeln!(f, "hello\ngoodbye")?;
    f.increase_indent();
    writeln!(f, "hello\ngoodbye")?;
    f.decrease_indent();
    writeln!(f, "hello\ngoodbye")
}

#[test]
fn test_indent_writer() {
    let mut buffer = String::new();
    write_text(&mut buffer).expect("failed to format");
    assert_eq!(
        "hello\ngoodbye\n  hello\n  goodbye\nhello\ngoodbye\n",
        buffer
    );
}

struct TestIndentFormatter;

impl fmt::Display for TestIndentFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = IndentFormatter::new(f, 2);
        writeln!(f, "hello\ngoodbye")?;
        f.increase_indent();
        writeln!(f, "hello\ngoodbye")?;
        f.decrease_indent();
        writeln!(f, "hello\ngoodbye")
    }
}

#[test]
fn test_indent_formatter() {
    assert_eq!(
        "hello\ngoodbye\n  hello\n  goodbye\nhello\ngoodbye\n",
        TestIndentFormatter.to_string()
    );
}
