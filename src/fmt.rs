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

//! Indent formatting in format streams.

use core::fmt;

use crate::{
    common::{IndentWrite, IndentWriterCommon, IndentWriterImpl},
    marker::{self},
};

impl<W: fmt::Write> IndentWrite<marker::Format> for W {
    type Error = fmt::Error;

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write_char(c)
    }

    #[inline]
    fn write_str(&mut self, str: &str) -> Result<(), Self::Error> {
        self.write_str(str)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Format-level indent writer.
pub struct IndentWriter<'fmt, W: fmt::Write> {
    inner: IndentWriterImpl<'fmt, marker::Format, W>,
}

impl<'fmt, W: fmt::Write> IndentWriter<'fmt, W> {
    /// Constructs a new indent writer managing indents of `indent` spaces in
    /// the wrapped format stream `w`.
    pub fn new(w: &'fmt mut W, indent: usize) -> Self {
        Self {
            inner: IndentWriterImpl::new(w, indent),
        }
    }
}

impl<W: fmt::Write> IndentWriterCommon for IndentWriter<'_, W> {
    fn increase_indent(&mut self) {
        self.inner.increase_indent();
    }

    fn decrease_indent(&mut self) {
        self.inner.decrease_indent();
    }

    fn indent_if_needed(&mut self) {
        let _ = self.inner.indent_if_needed();
    }
}

impl<W: fmt::Write> fmt::Write for IndentWriter<'_, W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.inner.write_str(s)
    }
}

pub type IndentFormatter<'fmt, 'buffer> =
    IndentWriter<'fmt, fmt::Formatter<'buffer>>;
