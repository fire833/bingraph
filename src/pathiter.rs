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

use std::fs::{self, DirEntry, ReadDir};

/// Implements an iterator to iterate through all files
/// that are found within the provided system path.
pub struct PathIterator {
    directories: Vec<ReadDir>,
    curr_iter: Option<ReadDir>,
}

impl PathIterator {
    pub fn new(path: &str) -> Self {
        let mut s = Self {
            directories: vec![],
            curr_iter: None,
        };

        for subpath in path.split(":") {
            if let Ok(dir) = fs::read_dir(subpath) {
                s.directories.push(dir);
            }
        }

        if let Some(curr) = s.directories.pop() {
            s.curr_iter = Some(curr);
        }

        s
    }
}

impl Iterator for PathIterator {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.curr_iter {
            Some(i) => match i.next() {
                Some(dir) => match dir {
                    Ok(entry) => match entry.file_type() {
                        Ok(info) => {
                            if info.is_dir() {
                                self.next()
                            } else {
                                Some(entry)
                            }
                        }
                        Err(e) => {
                            println!("unable to get file type for path {:?}: {}", entry, e);
                            self.next()
                        }
                    },
                    Err(_) => self.next(),
                },
                None => {
                    if let Some(next_dir) = self.directories.pop() {
                        self.curr_iter = Some(next_dir);
                        self.next()
                    } else {
                        self.curr_iter = None;
                        self.next()
                    }
                }
            },
            None => None,
        }
    }
}
