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

//! Utility module for enabling an implementation-independent indent writer
//! backend.

/// A marker for a stream supporting indent formatting.
pub trait IndentWriteMarker {}

/// Marks an IO stream.
#[cfg(feature = "io")]
pub struct IO;

#[cfg(feature = "io")]
impl IndentWriteMarker for IO {}

/// Marks a format stream.
#[cfg(feature = "fmt")]
pub struct Format;

#[cfg(feature = "fmt")]
impl IndentWriteMarker for Format {}
