use std::collections::{HashMap, HashSet, VecDeque};

/// Map (node) -> (child, cost)
type Graph<'a> = HashMap<&'a str, HashSet<(&'a str, u32)>>;

/// Find if a node is reachable from another node via BFS.
fn is_reachable(graph: &Graph, src: &str, dst: &str) -> bool {
    let mut visited = HashSet::new();
    visited.insert(src);
    let mut queue = VecDeque::new();
    queue.push_back(src);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if node == dst {
            return true;
        }
        for child in graph.get(node).unwrap() {
            if !visited.contains(child.0) {
                queue.push_back(child.0);
                visited.insert(child.0);
            }
        }
    }
    false
}

fn get_inner_cost(graph: &Graph, src: &str) -> u32 {
    let children = graph.get(src).unwrap();
    if children.is_empty() {
        return 0;
    }
    children.iter().fold(0, |acc, child| {
        acc + child.1 + child.1 * get_inner_cost(graph, child.0)
    })
}

fn build_input_graph() -> Graph<'static> {
    let mut graph = Graph::new();

    for line in include_str!("../input/day07.txt").lines() {
        let (source, children) = line.split_once(" bags contain ").unwrap();
        let child = children.strip_suffix('.').unwrap();
        if child == "no other bags" {
            graph.entry(source).or_default();
        } else {
            child.split(", ").for_each(|x| {
                let (cost, node) = x.split_once(' ').unwrap();
                let cost = cost.parse::<u32>().unwrap();
                let node = node.rsplit_once(' ').unwrap().0;
                graph.entry(source).or_default().insert((node, cost));
            });
        }
    }

    graph
}

pub fn solve_1() -> usize {
    let graph = build_input_graph();
    graph
        .keys()
        .filter(|&x| *x != "shiny gold" && is_reachable(&graph, x, "shiny gold"))
        .count()
}

pub fn solve_2() -> u32 {
    let graph = build_input_graph();
    get_inner_cost(&graph, "shiny gold")
}
