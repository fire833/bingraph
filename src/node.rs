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

use std::fs::{self, DirEntry};

use goblin::Object;
use serde::Serialize;

use crate::errors::BingraphError;

/// A BinNode is a wrapper around a filesystem node on the searched system. This
/// includes shared libraries, ELF binaries, and interpreted executables.
#[derive(Debug, Serialize)]
pub struct BinNode {
    name: String,
    absolute_path: String,

    node_type: NodeType,

    #[serde(skip)]
    dependencies: Vec<String>,

    id: Option<usize>,
    betweenness_centrality: Option<f64>,
    katz_centrality: Option<f64>,
    eigen_centrality: Option<f64>,
    closeness_centrality: Option<f64>,
}

impl BinNode {
    pub fn id(&self) -> String {
        self.absolute_path.clone()
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    pub fn set_betweeness_centrality(&mut self, c: f64) {
        self.betweenness_centrality = Some(c);
    }

    pub fn set_katz_centrality(&mut self, c: f64) {
        self.katz_centrality = Some(c);
    }

    pub fn set_eigen_centrality(&mut self, c: f64) {
        self.eigen_centrality = Some(c);
    }

    pub fn set_closeness_centrality(&mut self, c: f64) {
        self.closeness_centrality = Some(c);
    }

    pub fn get_type(&self) -> NodeType {
        self.node_type.clone()
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    ELFBinary,
    ELFLibrary,
    PortableExecutable,
    #[allow(unused)]
    InterpretedExecutable,
}

impl Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NodeType::ELFBinary => serializer.serialize_bytes("elf_binary".as_bytes()),
            NodeType::ELFLibrary => serializer.serialize_bytes("elb_library".as_bytes()),
            NodeType::PortableExecutable => serializer.serialize_bytes("pe".as_bytes()),
            NodeType::InterpretedExecutable => serializer.serialize_bytes("interp".as_bytes()),
        }
    }
}

impl TryFrom<DirEntry> for BinNode {
    type Error = BingraphError;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let file = match fs::read(value.path()) {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
        };

        match Object::parse(&file) {
            Ok(obj) => match obj {
                Object::Elf(elf) => {
                    let t: NodeType;
                    if elf.is_lib {
                        t = NodeType::ELFLibrary;
                    } else {
                        t = NodeType::ELFBinary;
                    }

                    Ok(Self {
                        name: value.file_name().into_string().unwrap_or_default(),
                        absolute_path: value
                            .path()
                            .into_os_string()
                            .into_string()
                            .unwrap_or_default(),
                        node_type: t,
                        dependencies: vec![],
                        id: None,
                        betweenness_centrality: None,
                        katz_centrality: None,
                        eigen_centrality: None,
                        closeness_centrality: None,
                    })
                }

                Object::PE(_) => Ok(Self {
                    name: value.file_name().into_string().unwrap_or_default(),
                    absolute_path: value
                        .path()
                        .into_os_string()
                        .into_string()
                        .unwrap_or_default(),
                    node_type: NodeType::PortableExecutable,
                    dependencies: vec![],
                    id: None,
                    betweenness_centrality: None,
                    katz_centrality: None,
                    eigen_centrality: None,
                    closeness_centrality: None,
                }),

                _ => return Err(format!("{:?} is of unknown file type", value.path()).into()),
            },
            Err(e) => return Err(e.into()),
        }
    }
}
