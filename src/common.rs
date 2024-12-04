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

use std::marker::PhantomData;

use crate::marker::IndentWriteMarker;

pub(crate) trait IndentWrite<M: IndentWriteMarker> {
    type Error;

    fn write_char(&mut self, c: char) -> Result<(), Self::Error>;
    fn write_str(&mut self, str: &str) -> Result<(), Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}

pub(crate) struct IndentWriterImpl<'w, M: IndentWriteMarker, W: IndentWrite<M>>
{
    add_indent: usize,
    current_indent: String,
    last_was_newline: bool,
    pub(crate) wrapped: &'w mut W,
    _marker: PhantomData<M>,
}

impl<'w, M: IndentWriteMarker, W: IndentWrite<M>> IndentWriterImpl<'w, M, W> {
    pub(crate) fn new(wrapped: &'w mut W, add_indent: usize) -> Self {
        Self {
            add_indent,
            current_indent: String::new(),
            last_was_newline: false,
            wrapped,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn increase_indent(&mut self) {
        for _ in 0..self.add_indent {
            self.current_indent.push(' ');
        }
    }

    #[inline]
    pub(crate) fn decrease_indent(&mut self) {
        let new_length = self.current_indent.len() - self.add_indent;
        self.current_indent.truncate(new_length);
    }
}

impl<M: IndentWriteMarker, W: IndentWrite<M>> IndentWrite<M>
    for IndentWriterImpl<'_, M, W>
{
    type Error = W::Error;

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        if self.last_was_newline {
            self.wrapped.write_str(&self.current_indent)?;
        }
        self.wrapped.write_char(c)?;
        self.last_was_newline = c == '\n';
        Ok(())
    }

    #[inline]
    fn write_str(&mut self, str: &str) -> Result<(), Self::Error> {
        for c in str.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.wrapped.flush()
    }
}

pub trait IndentWriterCommon {
    /// Adds a level of indentation. The new indent takes effect upon the next
    /// newline.
    fn increase_indent(&mut self);

    /// Removes a level of indentation. The new indent takes effect upon the
    /// next newline.
    fn decrease_indent(&mut self);
}
