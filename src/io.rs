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

//! Indent formatting in I/O buffers.

use core::str;
use std::io;

use crate::{
    common::{IndentWrite, IndentWriterCommon, IndentWriterImpl},
    marker,
};

impl<W: io::Write> IndentWrite<marker::IO> for W {
    type Error = io::Error;

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    fn write_str(&mut self, str: &str) -> Result<(), Self::Error> {
        <Self as io::Write>::write(self, str.as_bytes()).and(Ok(()))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush()
    }
}

/// I/O-level indent writer.
pub struct IndentWriter<'buffer, W: io::Write> {
    inner: IndentWriterImpl<'buffer, marker::IO, W>,
}

impl<'buffer, W: io::Write> IndentWriter<'buffer, W> {
    /// Constructs a new indent writer managing indents of `indent` spaces in
    /// the wrapped IO buffer `w`.
    pub fn new(w: &'buffer mut W, indent: usize) -> Self {
        Self {
            inner: IndentWriterImpl::new(w, indent),
        }
    }

    /// Provides access to the wrapped I/O buffer in `f`.
    pub fn with_raw_buffer<R, F: FnOnce(&mut W) -> R>(&mut self, f: F) -> R {
        f(self.inner.wrapped)
    }
}

impl<W: io::Write> IndentWriterCommon for IndentWriter<'_, W> {
    fn increase_indent(&mut self) {
        self.inner.increase_indent();
    }

    fn decrease_indent(&mut self) {
        self.inner.decrease_indent();
    }
}

impl<W: io::Write> io::Write for IndentWriter<'_, W> {
    fn write(&mut self, str: &[u8]) -> io::Result<usize> {
        self.inner
            .write_str(str::from_utf8(str).map_err(io::Error::other)?)?;
        Ok(str.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
