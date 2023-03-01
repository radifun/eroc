// =================================================================================================
// Copyright (c) 2023 Viet-Hoa Do <doviethoa@doviethoa.com>
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// =================================================================================================

use core::{fmt, result};

// =================================================================================================
// Result
// =================================================================================================

/// An alternative to [std::io::Result].
pub type Result<T> = result::Result<T, Error>;

// =================================================================================================
// Error
// =================================================================================================

/// An alternative to [std::io::Error].
///
/// Incompatibility list:
///   - [Error::new] always take `&'static str` as the error. No other type is permitted.
///   - [core::fmt::Display] trait is implemented but [ErrorKind] is displayed using
///     [core::fmt::Debug] trait.
///   - The following functions are not available:
///     - [std::io::Error::other]
///     - [std::io::Error::last_os_error]
///     - [std::io::Error::from_raw_os_error]
///     - [std::io::Error::raw_os_error]
///     - [std::io::Error::get_ref]
///     - [std::io::Error::get_mut]
///     - [std::io::Error::into_inner]
///     - [std::io::Error::downcast]
///   - The following traits are not implemented:
///     - [core::error::Error]
///     - [From]<[std::io::IntoInnerError]>
///     - [From]<[std::ffi::NulError]>
pub struct Error {
    data: ErrorData,
}

// Constructors ------------------------------------------------------------------------------------

impl Error {
    pub fn new(kind: ErrorKind, error: &'static str) -> Self {
        return Self { data: ErrorData::SimpleMessage(SimpleMessage::new(kind, error)) };
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        return Self { data: ErrorData::Simple(value) };
    }
}

// Properties --------------------------------------------------------------------------------------

impl Error {
    #[inline]
    pub fn kind(&self) -> ErrorKind {
        match &self.data {
            ErrorData::Simple(k) => *k,
            ErrorData::SimpleMessage(m) => m.kind,
        }
    }
}

// Debug trait implementation ----------------------------------------------------------------------

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            ErrorData::Simple(k) => f.debug_tuple("Kind").field(k).finish(),
            ErrorData::SimpleMessage(m) => {
                f.debug_struct("Error").field("kind", &m.kind).field("message", &m.message).finish()
            }
        }
    }
}

// Display trait implementation --------------------------------------------------------------------

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            ErrorData::Simple(k) => write!(f, "{:?}", k),
            ErrorData::SimpleMessage(m) => m.message.fmt(f),
        }
    }
}

// =================================================================================================
// Error data
// =================================================================================================

enum ErrorData {
    Simple(ErrorKind),
    SimpleMessage(SimpleMessage),
}

// =================================================================================================
// Simple message
// =================================================================================================

struct SimpleMessage {
    kind: ErrorKind,
    message: &'static str,
}

// Constructors ------------------------------------------------------------------------------------

impl SimpleMessage {
    fn new(kind: ErrorKind, message: &'static str) -> Self {
        return Self { kind, message };
    }
}

// =================================================================================================
// Error kind
// =================================================================================================

/// An alternative to [std::io::ErrorKind].
///
/// Incompatibility list:
///   - The list of choices is incomplete.
///   - The following traits are not implemented:
///     - [core::fmt::Display]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ErrorKind {}
