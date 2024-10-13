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

use std::fs::DirEntry;

use crate::errors::BingraphError;

/// A BinNode is a wrapper around a filesystem node on the searched system. This
/// includes shared libraries, ELF binaries, and interpreted executables.
pub struct BinNode {
    path: String,

    node_type: NodeType,
}

impl BinNode {
    pub fn path(&self) -> String {
        self.path.clone()
    }
}

pub enum NodeType {
    ELFBinary,
    ELFLibrary,
    InterpretedExecutable,
}

impl TryFrom<DirEntry> for BinNode {
    type Error = BingraphError;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        Err(BingraphError::GeneralError(
            "not implemented yet".to_string(),
        ))
    }
}
