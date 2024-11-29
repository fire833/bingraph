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

use std::{fs, io::Write};

use clap::Parser;
use cli::BingraphArgs;
use errors::BingraphError;
use graph::BinGraph;

mod cli;
mod errors;
mod graph;
mod node;
mod pathiter;

fn main() -> Result<(), BingraphError> {
    let args = BingraphArgs::parse();
    let g = match BinGraph::new(args.bin_path, args.lib_path) {
        Ok(g) => g,
        Err(e) => return Err(e),
    };

    let data = match serde_json::to_string_pretty(&g) {
        Ok(d) => d,
        Err(e) => return Err(e.into()),
    };

    match fs::File::create(args.output) {
        Ok(mut f) => f.write(data.as_bytes()).unwrap(),
        Err(e) => return Err(e.into()),
    };

    if args.output_graphviz != "" {
        match fs::File::create(args.output_graphviz) {
            Ok(mut f) => f.write(g.serialize_graphviz().as_bytes()).unwrap(),
            Err(e) => return Err(e.into()),
        };
    }

    Ok(())
}
