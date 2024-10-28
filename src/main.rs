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

use node::{BinNode, NodeType};
use pathiter::PathIterator;
use rustworkx_core::{
    centrality::{
        betweenness_centrality, closeness_centrality, eigenvector_centrality, katz_centrality,
    },
    petgraph::{
        visit::{IntoNodeIdentifiers, IntoNodeReferences},
        Graph,
    },
};

mod errors;
mod graph;
mod node;
mod pathiter;

fn main() {
    let mut graph: Graph<BinNode, (String, String)> = rustworkx_core::petgraph::Graph::new();

    for path in PathIterator::new(env!("PATH")) {
        let s = path.path();
        match BinNode::try_from(path) {
            Ok(node) => {
                println!("created node at {:?}", s);
                graph.add_node(node);
            }
            Err(e) => println!("unable to create node at {:?}: {}", s, e),
        }
    }
    // Iterate through every node and find it's dependencies, add the links.
    for node in graph.node_references() {
        match node.1.get_type() {
            NodeType::ELFBinary | NodeType::ELFLibrary => {}
            _ => {}
        }
    }

    let betweenness = betweenness_centrality(&graph, true, true, 4);
    let katz = katz_centrality(&graph, |g| Err(()), None, None, None, Some(1000), None);
    let eigen = eigenvector_centrality(&graph, |g| Err(()), Some(1000), None);
    let closeness = closeness_centrality(&graph, true);

    for node in graph.node_identifiers() {
        println!("{:?}", node);
    }
}
