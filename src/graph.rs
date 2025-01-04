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

use std::collections::HashMap;

use serde::Serialize;

use rustworkx_core::{
    centrality::{
        betweenness_centrality, closeness_centrality, eigenvector_centrality, katz_centrality,
    },
    petgraph::{adj::NodeIndex, graph::DiGraph, visit::IntoNodeReferences},
};

use crate::{errors::BingraphError, node::BinNode, pathiter::PathIterator};

#[derive(Debug, Serialize)]
pub struct BinGraph {
    nodes: Vec<BinNode>,

    edges: Vec<(String, String)>,
    degree_distribution: HashMap<u32, u32>,

    average_degree: f64,
    num_nodes: u32,
    num_edges: u32,
}

impl BinGraph {
    pub fn new(bin_path: String, lib_path: String) -> Result<Self, BingraphError> {
        let mut edges: Vec<((NodeIndex, String), (NodeIndex, String))> = vec![];
        let mut ext_nodes: Vec<BinNode> = vec![];

        // Mapping of BinNode names and their corresponding node index.
        let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
        let mut node_indegree: HashMap<String, u32> = HashMap::new();
        let mut graph: DiGraph<BinNode, u32> = rustworkx_core::petgraph::Graph::new();

        let mut total_path = bin_path;
        total_path.push(':');
        total_path.push_str(&lib_path);

        println!("searching through {} for things", total_path);

        // Go through every file and try to add it as a node, and add a reference
        // from the name to the nodeindex.
        for path in PathIterator::new(&total_path) {
            let s = path.path();
            match BinNode::try_from(path) {
                Ok(node) => {
                    // println!("created node at {:?}", s);
                    let nnode = node.clone();
                    let idx = graph.add_node(node);
                    nodes.insert(nnode.name(), idx.index() as u32);
                    node_indegree.insert(nnode.name(), 0);
                }
                Err(e) => println!("unable to create node at {:?}: {}", s, e),
            }
        }

        // Iterate through every node and find it's dependencies, add the links.
        for (sidx, node) in graph.node_references() {
            for neigh in node.get_dependencies() {
                if let Some((name, didx)) = nodes.get_key_value(neigh) {
                    edges.push(((sidx.index() as u32, node.name()), (*didx, name.clone())));

                    // Keep track of the indegree for each node as well.
                    if let Some(v) = node_indegree.get(name) {
                        node_indegree.insert(name.clone(), v + 1);
                    }
                }
            }
        }

        // Add the edges to the main graph structure too.
        for (src, dst) in edges.iter() {
            graph.add_edge(NodeIndex::from(src.0), NodeIndex::from(dst.0), 0);
        }

        // Compute our centralities
        println!("computing betweeness centrality for graph");
        let betweenness = betweenness_centrality(&graph, true, true, 4);
        println!("computing katz centrality for graph");
        let katz = match katz_centrality(
            &graph,
            |_| Ok::<f64, BingraphError>(1.),
            None,
            None,
            None,
            Some(100),
            None,
        ) {
            Ok(k) => k,
            Err(e) => {
                println!("unable to compute katz centrality: {}", e);
                None
            }
        };
        println!("computing eigenvector centrality for graph");
        let eigen =
            match eigenvector_centrality(&graph, |_| Ok::<f64, BingraphError>(1.), Some(100), None)
            {
                Ok(e) => e,
                Err(e) => {
                    println!("unable to compute eigenvector_centrality: {}", e);
                    None
                }
            };
        println!("computing closeness centrality for graph");
        let closeness = closeness_centrality(&graph, true);

        // Assign centralities to the new nodes and append to
        for (idx, node) in graph.node_references() {
            let mut new_node = node.clone();
            if let Some(Some(value)) = betweenness.get(idx.index()) {
                new_node.set_betweeness_centrality(*value);
            }
            if let Some(ref vec) = katz {
                if let Some(value) = vec.get(idx.index()) {
                    new_node.set_katz_centrality(*value);
                }
            }

            if let Some(ref vec) = eigen {
                if let Some(value) = vec.get(idx.index()) {
                    new_node.set_eigen_centrality(*value);
                }
            }
            if let Some(Some(value)) = closeness.get(idx.index()) {
                new_node.set_closeness_centrality(*value);
            }

            // Specify the outdegree of the node.
            new_node.set_out_degree(new_node.get_dependencies().len() as u32);

            // Specify the indegree of the node.
            if let Some(v) = node_indegree.get(&new_node.name()) {
                new_node.set_in_degree(*v);
            }

            ext_nodes.push(new_node);
        }

        let ext_edges: Vec<(String, String)> = edges
            .iter()
            .map(|x| (x.0 .1.clone(), x.1 .1.clone()))
            .collect();

        let num_nodes = graph.node_count() as u32;
        let num_edges = graph.edge_count() as u32;
        let avg_degree = num_nodes as f64 / num_edges as f64;

        // Compute our degree distribution
        println!("computing degree distribution for graph");
        let mut deg_dist: HashMap<u32, u32> = HashMap::new();
        for node in ext_nodes.iter() {
            let deg = node.get_in_degree();

            // If this degree has been found, increment
            if let Some(freq) = deg_dist.get(&deg) {
                deg_dist.insert(deg, freq + 1);
            } else {
                deg_dist.insert(deg, 1);
            }
        }

        return Ok(Self {
            nodes: ext_nodes,
            edges: ext_edges,
            num_nodes: num_nodes,
            num_edges: num_edges,
            average_degree: avg_degree,
            degree_distribution: deg_dist,
        });
    }

    pub fn serialize_graphviz(&self) -> String {
        let mut graph: String = "".to_string();

        graph.push_str("digraph bingraph {\n\n");

        for node in self.nodes.iter() {
            let n = node.format_graphviz();
            graph.push_str(&n);
        }

        graph.push_str("\n\n");

        for edge in self.edges.iter() {
            let e = format!("  \"{}\" -> \"{}\"\n", edge.0, edge.1);
            graph.push_str(&e);
        }

        graph.push_str("\n}");
        return graph;
    }
}
