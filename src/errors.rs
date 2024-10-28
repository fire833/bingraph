/*
*	Copyright (C) 2024 Kendall Tauser
*
*	This program is free software; you can redistribute it and/or modify
*	it under the terms of the GNU General Public License as published by
*	the Free Software Foundation; either version 2 of the License, or
*	(at your option) any later version.
*
*	This program is distributed in the hope that it will be useful,
*	but WITHOUT ANY WARRANTY; without even the implied warranty of
*	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*	GNU General Public License for more details.
*
*	You should have received a copy of the GNU General Public License along
*	with this program; if not, write to the Free Software Foundation, Inc.,
*	51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

use std::{fmt::Display, io};

use goblin::error::Error;

pub enum BingraphError {
    GeneralError(String),
    IOError(io::Error),
    GoblinError(Error),
}

impl Display for BingraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::GeneralError(e) => write!(f, "{}", e),
            Self::IOError(e) => write!(f, "io: {}", e),
            Self::GoblinError(e) => write!(f, "goblin: {}", e),
        }
    }
}

impl From<io::Error> for BingraphError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<Error> for BingraphError {
    fn from(value: Error) -> Self {
        Self::GoblinError(value)
    }
}

impl From<String> for BingraphError {
    fn from(value: String) -> Self {
        Self::GeneralError(value)
    }
}
