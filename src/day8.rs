use std::{
    collections::HashMap,
    iter::{Cycle, FusedIterator},
    str::FromStr,
};

use anyhow::Context;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct NodeName([char; 3]);

impl FromStr for NodeName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().fuse();
        match (chars.next(), chars.next(), chars.next()) {
            (Some(first), Some(second), Some(third)) => Ok(Self([first, second, third])),
            _ => anyhow::bail!("invalid node name: {s}"),
        }
    }
}

impl NodeName {
    fn ends_with(&self) -> char {
        self.0[2]
    }
}

#[derive(Debug)]
struct NodeDescription {
    current: NodeName,
    left: NodeName,
    right: NodeName,
}

impl FromStr for NodeDescription {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // AAA = (BBB, CCC)
        let mut it = s.split(" = ");
        let current = it
            .next()
            .context("missing current node")?
            .parse()
            .context("failed to parse current")?;

        let second_part = it.next().context("missing left and right nodes")?;

        // (BBB, CCC)
        anyhow::ensure!(second_part.len() > 1);
        let second_part = &second_part[1..second_part.len() - 1];
        // BBB, CCC
        let mut it = second_part.split(", ");
        let left = it
            .next()
            .context("missing second node")?
            .parse()
            .context("failed to parse second node")?;
        let right = it
            .next()
            .context("missing third node")?
            .parse()
            .context("failed to parse third node")?;

        Ok(Self {
            current,
            left,
            right,
        })
    }
}

struct NetworkBuilder {
    nodes: Vec<NetworkNode>,
    node_mapping: HashMap<NodeName, usize>,
    heads: Vec<usize>,
}

impl NetworkBuilder {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            node_mapping: HashMap::with_capacity(capacity),
            heads: Vec::new(),
        }
    }

    fn insert_new_node(&mut self, node_name: NodeName) -> usize {
        let new_node = NetworkNode::new(node_name);
        let is_start_node = new_node.is_start_node();
        self.nodes.push(new_node);
        let index = self.nodes.len() - 1; // no underflow because we pushed at least one
        if is_start_node {
            self.heads.push(index)
        }
        index
    }

    fn get_or_insert_node_index(&mut self, node_name: NodeName) -> usize {
        match self.node_mapping.get(&node_name) {
            Some(index) => *index,
            None => {
                let index = self.insert_new_node(node_name);
                self.node_mapping.insert(node_name, index);
                index
            }
        }
    }

    fn get_or_insert_node(&mut self, node_name: NodeName) -> &mut NetworkNode {
        let index = match self.node_mapping.get(&node_name) {
            Some(index) => *index,
            None => {
                let index = self.insert_new_node(node_name);
                self.node_mapping.insert(node_name, index);
                index
            }
        };
        self.nodes
            .get_mut(index)
            .expect("we just inserted the node")
    }

    fn finish(self, start_node: NodeName) -> anyhow::Result<Network> {
        let head = self.node_mapping.get(&start_node).copied();
        Ok(Network {
            nodes: self.nodes,
            head,
            heads: self.heads,
        })
    }
}

fn create_network_from_node_description(
    node_descriptions: &[NodeDescription],
) -> anyhow::Result<Network> {
    let mut network_builder = NetworkBuilder::with_capacity(node_descriptions.len());
    for node_description in node_descriptions {
        let left_node_index = network_builder.get_or_insert_node_index(node_description.left);
        let right_node_index = network_builder.get_or_insert_node_index(node_description.right);
        let node = network_builder.get_or_insert_node(node_description.current);

        // Don't add node that lead to infinite recursion
        if node_description.left != node_description.current {
            node.left.replace(left_node_index);
        }

        if node_description.right != node_description.current {
            node.right.replace(right_node_index);
        }
    }

    network_builder
        .finish(NodeName(['A', 'A', 'A']))
        .context("failed to build network")
}

fn create_network_from_node_description_str(node_descriptions: &[&str]) -> anyhow::Result<Network> {
    let nodes: anyhow::Result<Vec<NodeDescription>> = node_descriptions
        .iter()
        .map(|x| {
            x.parse()
                .with_context(|| format!("failed to parse node description: {x}"))
        })
        .collect();

    let nodes = nodes.context("failed to parse node descriptions")?;

    create_network_from_node_description(&nodes).context("failed to create network")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Self::Right),
            'L' => Ok(Self::Left),
            _ => anyhow::bail!("invalid value for instruction: {value}"),
        }
    }
}

#[derive(Debug)]
pub struct NetworkNode {
    name: NodeName,
    left: Option<usize>,
    right: Option<usize>,
}

// We can probably compare the left and right but nodes are unique
impl PartialEq for NetworkNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for NetworkNode {}

impl NetworkNode {
    fn new(name: NodeName) -> Self {
        Self {
            name,
            left: None,
            right: None,
        }
    }

    fn is_start_node(&self) -> bool {
        self.name.ends_with() == 'A'
    }

    fn is_end_node(&self) -> bool {
        self.name.ends_with() == 'Z'
    }
}

// pretty much a binary graph (because contains cycles)
#[derive(Debug)]
pub struct Network {
    nodes: Vec<NetworkNode>,
    head: Option<usize>, // optional for part2
    heads: Vec<usize>,   // part 2
}

impl Network {
    fn get_head(&self) -> Option<&NetworkNode> {
        self.nodes.get(self.head?)
    }

    fn get_left(&self, node: &NetworkNode) -> Option<&NetworkNode> {
        node.left.map(|left| self.nodes.get(left))?
    }

    fn get_right(&self, node: &NetworkNode) -> Option<&NetworkNode> {
        node.right.map(|right| self.nodes.get(right))?
    }

    fn get_next(&self, node: &NetworkNode, instruction: &Instruction) -> Option<&NetworkNode> {
        match instruction {
            Instruction::Left => self.get_left(node),
            Instruction::Right => self.get_right(node),
        }
    }

    fn get_heads(&self) -> Vec<&NetworkNode> {
        self.heads
            .iter()
            .map(|index| {
                self.nodes
                    .get(*index)
                    .expect("the index must point to a valid node in the graph")
            })
            .collect()
    }
}

struct InfiniteNetworkIterator<'a, T> {
    network: &'a Network,
    instructions_iter: Cycle<T>,
    current: &'a NetworkNode,
}

impl<'a, T> InfiniteNetworkIterator<'a, T>
where
    T: Iterator<Item = &'a Instruction> + FusedIterator + Clone,
{
    fn new(network: &'a Network, instructions_iter: T, head: &'a NetworkNode) -> Self {
        Self {
            network,
            instructions_iter: instructions_iter.cycle(),
            current: head,
        }
    }
}

impl<'a, T> Iterator for InfiniteNetworkIterator<'a, T>
where
    T: Iterator<Item = &'a Instruction> + FusedIterator + Clone,
{
    type Item = &'a NetworkNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current_instruction = self.instructions_iter.next()?;
        self.current = self.network.get_next(self.current, current_instruction)?;

        Some(self.current)
    }
}

impl<'a, T> FusedIterator for InfiniteNetworkIterator<'a, T> where
    T: Iterator<Item = &'a Instruction> + FusedIterator + Clone
{
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a.max(b), b.min(a));

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

pub struct Map {
    instructions: Vec<Instruction>,
    network: Network,
}

impl Map {
    fn get_num_steps(&self, target_node: NodeName) -> anyhow::Result<u32> {
        let mut current_node = self.network.get_head().context("missing head")?;
        let mut num_steps = 0;
        loop {
            for instruction in &self.instructions {
                if current_node.name == target_node {
                    return Ok(num_steps);
                }

                current_node = self
                    .network
                    .get_next(&current_node, instruction)
                    .context("reached infinite loop")?;
                num_steps += 1;
            }
        }
    }

    fn network_iter<'a>(
        &'a self,
        head: &'a NetworkNode,
    ) -> InfiniteNetworkIterator<'_, std::slice::Iter<'_, Instruction>> {
        InfiniteNetworkIterator::new(&self.network, self.instructions.iter(), head)
    }

    fn get_num_steps_to_reach_end(&self, node: &NetworkNode) -> u32 {
        let mut num_steps = 0;

        for node in self.network_iter(node) {
            num_steps += 1;
            if node.is_end_node() {
                break;
            }
        }

        num_steps
    }

    fn get_num_steps_for_all_heads(&self) -> u64 {
        let heads = self.network.get_heads();
        let mut steps_to_reach_end = Vec::with_capacity(heads.len());

        for node in heads.iter() {
            steps_to_reach_end.push(self.get_num_steps_to_reach_end(node));
        }

        steps_to_reach_end
            .into_iter()
            .fold(1, |acc, e| lcm(acc, e as u64))
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // RL
        //
        // AAA = (BBB, CCC)

        let mut lines = s.lines();
        let instructions: anyhow::Result<Vec<Instruction>> = lines
            .next()
            .context("missing instructions line")?
            .chars()
            .map(|x| x.try_into().context("failed to parse instruction"))
            .collect();
        let instructions = instructions.context("failed to parse instructions")?;
        lines.next().context("missing blank line")?;

        let nodes = lines.collect_vec();
        let network =
            create_network_from_node_description_str(&nodes).context("failed to create network")?;

        Ok(Self {
            instructions,
            network,
        })
    }
}

pub fn part1(map: &Map) -> u32 {
    map.get_num_steps(NodeName(['Z', 'Z', 'Z'])).unwrap()
}

pub fn part2(map: &Map) -> u64 {
    map.get_num_steps_for_all_heads()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{get_day_extra_test_input, get_day_test_input, parse_input};

    #[test]
    fn test_part1() {
        let map = parse_input(get_day_test_input("day8"));
        assert_eq!(part1(&map), 2);
    }

    #[test]
    fn test_part1_extra() {
        let map = parse_input(get_day_extra_test_input("day8", 2));
        assert_eq!(part1(&map), 6);
    }

    #[test]
    fn test_part2() {
        let map = parse_input(get_day_extra_test_input("day8", 3));
        assert_eq!(part2(&map), 6);
    }
}
