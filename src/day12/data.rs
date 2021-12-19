/*
 * MIT License
 *
 * Copyright (c) 2021 Luiz Ferraz
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cave {
    big: bool,
    edges: Vec<String>,
}

#[derive(Default, Clone, Debug)]
pub struct CaveSystem {
    caves: HashMap<String, Cave>,
}

pub struct CaveHandle<'a> {
    system: &'a CaveSystem,
    cave: &'a Cave,
    name: &'a str,
}

impl CaveSystem {
    pub fn add_tunnel(&mut self, a: &str, b: &str) {
        self.get_or_insert_cave(a).edges.push(b.to_string());

        self.get_or_insert_cave(b).edges.push(a.to_string());
    }

    pub fn get_cave<'a>(&'a self, name: &'a str) -> Option<CaveHandle<'a>> {
        self.caves.get(name).map(move |cave| CaveHandle {
            system: self,
            name,
            cave,
        })
    }

    pub fn get_or_insert_cave(&mut self, name: &'_ str) -> &mut Cave {
        self.caves.entry(name.to_string()).or_insert_with(|| Cave {
            big: name.to_ascii_uppercase() == name,
            edges: Vec::new(),
        })
    }
}

impl<'a> CaveHandle<'a> {
    pub fn seek_no_double_small<'b>(
        &'b self,
        target: &'b str,
    ) -> impl Iterator<Item = Vec<&'a str>> + 'b {
        self.seek_no_double_small_inner(target, vec![self.name])
    }

    fn seek_no_double_small_inner<'t>(
        &'t self,
        target: &'t str,
        current_path: Vec<&'a str>,
    ) -> impl Iterator<Item = Vec<&'a str>> + 't {
        self.cave.edges.iter().flat_map(move |edge| {
            let mut path = current_path.to_vec();

            if edge == target {
                path.push(edge);
                vec![path]
            } else {
                let next_handle = self
                    .system
                    .get_cave(edge)
                    .expect("Cave with an edge not found");

                if !next_handle.cave.big && current_path.contains(&edge.as_ref()) {
                    return vec![];
                }

                path.push(edge);

                next_handle
                    .seek_no_double_small_inner(target, path)
                    .collect_vec()
            }
        })
    }

    pub fn seek_single_double_small<'b>(
        &'b self,
        target: &'b str,
    ) -> impl Iterator<Item = Vec<&'a str>> + 'b {
        self.seek_single_double_small_inner(target, vec![self.name], false)
    }

    fn seek_single_double_small_inner<'t>(
        &'t self,
        target: &'t str,
        current_path: Vec<&'a str>,
        double_small_seen: bool,
    ) -> impl Iterator<Item = Vec<&'a str>> + 't {
        self.cave.edges.iter().flat_map(move |edge| {
            let mut path = current_path.to_vec();

            if edge == target {
                path.push(edge);
                vec![path]
            } else {
                let next_handle = self
                    .system
                    .get_cave(edge)
                    .expect("Cave with an edge not found");

                let is_double_small =
                    !next_handle.cave.big && current_path.contains(&edge.as_ref());

                if is_double_small && (double_small_seen || edge == "start" || edge == "end") {
                    return vec![];
                }

                path.push(edge);

                next_handle
                    .seek_single_double_small_inner(
                        target,
                        path,
                        double_small_seen || is_double_small,
                    )
                    .collect_vec()
            }
        })
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for CaveSystem {
    fn from_iter<T: IntoIterator<Item = (&'a str, &'a str)>>(iter: T) -> Self {
        let mut caves = Self::default();

        for (a, b) in iter {
            caves.add_tunnel(a, b);
        }

        caves
    }
}

pub struct Parser;

impl Parser {
    pub fn parse_input(input: &str) -> anyhow::Result<CaveSystem> {
        input
            .trim()
            .split('\n')
            .map(|line| line.split_once('-').ok_or(anyhow::anyhow!("Invalid line")))
            .try_collect()
    }
}
