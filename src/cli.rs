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

#[derive(Debug, clap::Parser)]
pub struct BingraphArgs {
    /// Output location for constructed graph file.
    #[arg(short, long, default_value_t = String::from("graph.json"))]
    pub output: String,

    /// Specify a path string to search through for acquiring binaries.
    #[arg(short, long, default_value_t = env!("PATH").to_string())]
    pub bin_path: String,

    /// Specify a path string to search through for acquiring binaries.
    #[arg(short, long, default_value_t = String::from("/usr/x86_64-pc-linux-gnu/lib64:/usr/lib:/usr/local/lib:/usr/x86_64-pc-linux-gnu/lib"))]
    pub lib_path: String,
}
