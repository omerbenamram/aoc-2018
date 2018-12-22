#[macro_use]
extern crate lazy_static;
use log::debug;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

use env_logger;
use failure::{format_err, Error};
use petgraph::graph::EdgeReference;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;

use colored::Colorize;
use regex::Regex;

type Result<T> = ::std::result::Result<T, Error>;
type Step = char;

struct Dependency {
    pub name: Step,
    pub depends_on: Step,
}

type DependencyGraph = Graph<Step, Step>;

impl FromStr for Dependency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            pub static ref RE: Regex =
                Regex::new(r#"Step ([A-Z]) must be finished before step ([A-Z]) can begin\."#)
                    .expect("This should be a valid regex");
        }

        if let Some(matches) = RE.captures(s) {
            return Ok(Dependency {
                name: matches[2].as_bytes()[0] as Step,
                depends_on: matches[1].as_bytes()[0] as Step,
            });
        };

        Err(format_err!("Failed to match text"))
    }
}

macro_rules! print_edges {
    ($prefix: expr, $edges: expr, $graph: expr) => {
        debug!(
            "{}: {:?}",
            $prefix,
            $edges
                .iter()
                .map(|e| format!("{} -> {}", $graph[e.source()], $graph[e.target()]))
                .collect::<Vec<_>>()
        );
    };
}

fn build_dependency_graph(input: &str) -> Result<DependencyGraph> {
    let mut graph = DependencyGraph::new();
    let mut nodes = HashMap::new();

    for c in b'A'..=b'Z' {
        let idx = graph.add_node(c as char);
        nodes.insert(c as char, idx);
    }

    for line in input.lines() {
        let edge = Dependency::from_str(line)?;
        graph.add_edge(
            *nodes.get(&edge.depends_on).expect("Indexed all letters"),
            *nodes.get(&edge.name).expect("Indexed all letters"),
            edge.name,
        );
    }

    graph.retain_nodes(|g, node| g.neighbors_undirected(node).next().is_some());

    let nodes_without_deps = find_nodes_without_dependencies(&graph);

    if nodes_without_deps.is_empty() {
        return Err(format_err!("Failed to find nodes to start"));
    }

    let dummy_head = graph.add_node('h');
    let mut dummy_edges = vec![];

    for node in nodes_without_deps.iter() {
        dummy_edges.push(graph.add_edge(dummy_head, *node, graph[*node]));
    }

    Ok(graph)
}

fn step_is_possible(
    step_as_edge: EdgeReference<Step, u32>,
    graph: &DependencyGraph,
    visited: &HashSet<Step>,
) -> bool {
    let target = graph[step_as_edge.target()];

    let deps: Vec<_> = graph
        .edges_directed(step_as_edge.target(), Direction::Incoming)
        .collect();

    debug!("  Checking {}", target);
    let can_do = deps.iter().all(|dep| {
        let dep_name = graph[dep.source()];
        let ok = visited.contains(&dep_name);
        debug!(
            "  Checking dep {:?} -> ({})",
            dep_name,
            if ok { "Found" } else { "Not found" }
        );
        ok
    });

    if can_do {
        debug!("{}", format!("{} is possible", target).green());
    } else {
        debug!("{}", format!("{} is not possible", target).red());
    }

    can_do
}

fn find_nodes_without_dependencies(graph: &DependencyGraph) -> Vec<NodeIndex> {
    let mut nodes_without_deps = vec![];
    // Find nodes without dependencies
    for idx in graph.node_indices() {
        let dependencies: Vec<_> = graph.edges_directed(idx, Direction::Incoming).collect();

        if dependencies.is_empty() {
            nodes_without_deps.push(idx);
        }
    }

    nodes_without_deps
}

fn part1(graph: &DependencyGraph) -> Result<String> {
    let mut available_steps = Vec::new();
    let mut result = String::new();
    let mut visited = HashSet::new();

    visited.insert('h');
    let head = graph
        .node_references()
        .find(|n| *n.weight() == 'h')
        .expect("Head should exist");

    available_steps.extend(graph.edges(head.0));

    // Account for the dummy node ('h');
    while result.len() < graph.node_count() - 1 {
        while !available_steps.is_empty() {
            debug!("So far (ordered): {}", result);
            debug!("So far done: {:?}", visited);

            // Sort steps lexicographically
            available_steps.sort_by_key(|f| f.weight());

            // Reverse the stack so the cheapest step will be on top
            available_steps.reverse();
            print_edges!("Steps", available_steps, graph);

            let step = available_steps
                .pop()
                .expect("We've checked above that stack is non-empty");

            let target = graph[step.target()];

            // Redundant step, ignore it
            if visited.contains(&target) {
                debug!(
                    "{} Already done, Skipping `{} -> {}`",
                    graph[step.target()],
                    graph[step.source()],
                    graph[step.target()]
                );
            } else {
                if step_is_possible(step, &graph, &visited) {
                    debug!("{} Done", target);
                    let next: Vec<_> = graph
                        .edges_directed(step.target(), Direction::Outgoing)
                        .collect();

                    available_steps.extend(next);

                    visited.insert(target);
                    result.push(target);
                }
            }

            print_edges!("With added steps", available_steps, graph);
            debug!("---------------------------------");
        }
    }

    Ok(result)
}

#[test]
fn test_part1() {
    env_logger::init();
    let test_input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";
    let graph = build_dependency_graph(test_input).unwrap();

    assert_eq!(part1(&graph).unwrap(), "CABDFE".to_owned());
}

fn main() -> Result<()> {
    env_logger::init();
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day7/input/tasks");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;
    let graph = build_dependency_graph(&input)?;

    println!("Steps: {}", part1(&graph)?);

    Ok(())
}
