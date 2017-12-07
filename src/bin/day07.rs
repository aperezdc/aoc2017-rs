//
// day07.rs
// Copyright (C) 2017 Adrian Perez <aperez@igalia.com>
// Distributed under terms of the MIT license.
//

extern crate ego_tree;

#[macro_use]
extern crate failure;

use ego_tree::{ NodeMut, NodeRef, Tree };
use ego_tree::iter::Edge;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Debug)]
struct Tower
{
    name: String,
    weight: u32,
}


impl FromStr for Tower
{
    type Err = ::failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let pos = if let Some(p) = s.find(' ') { p } else {
            bail!("Input '{}' does not contain a space", s)
        };

        let (name, weight_str) = s.split_at(pos);
        let weight = weight_str.trim()
            .trim_left_matches('(')
            .trim_right_matches(')')
            .parse()?;

        Ok(Tower { name: name.to_string(), weight: weight })
    }
}


fn tree_node_fill(mut node: NodeMut<Tower>,
                  towers: &mut HashMap<String, Tower>,
                  parent: &HashMap<String, String>)
{
    let node_name = node.value().name.clone();
    for (name, parent_name) in parent {
        if *parent_name == node_name {
            let mut child_node = node.append(towers.remove(name).unwrap());
            tree_node_fill(child_node, towers, parent);
        }
    }
}


fn build_tree(mut towers: HashMap<String, Tower>,
              parent: HashMap<String, String>) -> Tree<Tower>
{
    let root_name = towers.keys()
        .filter(|&key| !parent.contains_key(key))
        .next().unwrap().to_string();

    let mut tree = Tree::new(towers.remove(&root_name).unwrap());
    tree_node_fill(tree.root_mut(), &mut towers, &parent);
    tree
}


fn subtree_weight(node: NodeRef<Tower>) -> u32
{
    node.value().weight + node.children().fold(0, |sum, n| sum + subtree_weight(n))
}


fn find_unbalanced_node(node: NodeRef<Tower>) -> Option<(NodeRef<Tower>, u32)>
{
    if node.has_children() {
        let mut max_index = 0;
        let mut min_weight = u32::max_value();
        let mut max_weight = u32::min_value();

        for (index, weight) in node.children().map(subtree_weight).enumerate() {
            if weight > max_weight {
                max_weight = weight;
                max_index = index;
            }
            if weight < min_weight {
                min_weight = weight;
            }
        }

        if min_weight != max_weight {
            let node = node.children().nth(max_index).unwrap();
            return Some((node, max_weight - min_weight));
        }
    } 

    None
}


fn main()
{
    let mut parent = HashMap::new();
    let mut towers = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(io::Result::ok) {
        // Each line of input is:  <name> (<weight>) [-> child1[, child2, ...]]
        let (tower_str, child_str) = if let Some(arrow_pos) = line.find("->") {
            let (left, right) = line.split_at(arrow_pos);
            (left.trim(), right.trim_left_matches("->").trim())
        } else {
            (line.as_str(), "")
        };

        let tower = tower_str.parse::<Tower>().unwrap();

        if child_str.len() > 0 {
            for child in child_str.split(',') {
                parent.insert(child.trim().to_string(), tower.name.clone());
            }
        }

        towers.insert(tower.name.clone(), tower);
    }

    let tree = build_tree(towers, parent);
    println!("Root: {}", tree.root().value().name);

    for node in tree.root().traverse().filter_map(|item| match item {
        Edge::Close(node) => Some(node),
        Edge::Open(_) => None
    }) {
        if let Some((n, weight_diff)) = find_unbalanced_node(node) {
            println!("Diff: {}: {} -> {}",
                     n.value().name, n.value().weight,
                     n.value().weight - weight_diff);
            break;
        }
    }
}
