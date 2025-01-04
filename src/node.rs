/*
*	Copyright (C) 2025 Kendall Tauser
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
#[derive(Debug, Serialize, Clone)]
pub struct BinNode {
    name: String,
    absolute_path: String,

    node_type: NodeType,

    #[serde(skip)]
    dependencies: Vec<String>,

    in_degree: u32,
    out_degree: u32,

    betweenness_centrality: Option<f64>,
    katz_centrality: Option<f64>,
    eigen_centrality: Option<f64>,
    closeness_centrality: Option<f64>,
}

impl BinNode {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn format_graphviz(&self) -> String {
        let color: &str = match self.node_type {
            NodeType::ELFBinary => "blue",
            NodeType::ELFLibrary => "green",
            NodeType::PortableExecutable => "pink",
            NodeType::InterpretedExecutable => "red",
        };

        return format!(
            "  \"{}\" [style=filled, color=\"{}\", tooltip=\"Absolute Path: {}\\nOutdegree: {}\\nIndegree: {}\\nBetweeness: {}\\nKatz: {}\\nEigen: {}\\nCloseness: {}\"];\n",
            self.name(),
            color,
            self.absolute_path,
            self.dependencies.len(),
            self.in_degree,
            self.betweenness_centrality.unwrap_or_default(),
            self.katz_centrality.unwrap_or_default(),
            self.eigen_centrality.unwrap_or_default(),
            self.closeness_centrality.unwrap_or_default(),
        );
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

    pub fn set_in_degree(&mut self, v: u32) {
        self.in_degree = v;
    }

    pub fn set_out_degree(&mut self, v: u32) {
        self.out_degree = v;
    }

    pub fn get_dependencies(&self) -> &Vec<String> {
        &self.dependencies
    }

    pub fn get_in_degree(&self) -> u32 {
        self.in_degree
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
            NodeType::ELFBinary => serializer.serialize_str("elf_binary"),
            NodeType::ELFLibrary => serializer.serialize_str("elf_library"),
            NodeType::PortableExecutable => serializer.serialize_str("pe"),
            NodeType::InterpretedExecutable => serializer.serialize_str("interp"),
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
                        in_degree: 0,
                        out_degree: 0,
                        dependencies: elf.libraries.into_iter().map(String::from).collect(),
                        betweenness_centrality: None,
                        katz_centrality: None,
                        eigen_centrality: None,
                        closeness_centrality: None,
                    })
                }

                Object::PE(pe) => Ok(Self {
                    name: value.file_name().into_string().unwrap_or_default(),
                    absolute_path: value
                        .path()
                        .into_os_string()
                        .into_string()
                        .unwrap_or_default(),
                    node_type: NodeType::PortableExecutable,
                    in_degree: 0,
                    out_degree: 0,
                    dependencies: pe.libraries.into_iter().map(String::from).collect(),
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
