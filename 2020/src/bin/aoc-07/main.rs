use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use std::convert::TryInto;
use core::str::FromStr;

#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone)]
struct Edge {
    to: String,
    from: String,
    count: u64,
}
type Graph = HashMap<String, Vec<Edge>>;

fn has_path(graph: &Graph, initial_node: &str, target_node: &str) -> bool {
    graph[initial_node].iter()
        .any(|edge| {
            edge.to == target_node
                || has_path(&graph, &edge.to, target_node)
        })
}

fn count_paths(graph: &Graph, initial_node: &str) -> u64 {
    let children: u64 = graph[initial_node].iter()
        .map(|edge| {
            let result: u64 = edge.count * count_paths(&graph, &edge.to);
            result
        })
        .sum();
    return 1 + children;
}

fn make_graph(input: &str) -> Graph {
    let mut results = Graph::new();

    for line in input.lines() {
        let re = Regex::new(r"(.*) bags contain (.*).").unwrap();
        let caps = re.captures(line).unwrap();
        let outer: String = caps[1].to_string();
        let inners: String = caps[2].to_string();

        if !results.contains_key(&outer) {
            results.insert(outer.clone(), vec![]);
        }

        if inners == "no other bags" {
            continue;
        }

        for inner in inners.split(", ") {
            let inner_re = Regex::new(r"(\d+) (.*) bags?").unwrap();
            let inner_caps = inner_re.captures(inner).unwrap();
            let count = inner_caps[1].to_string();
            let colour = inner_caps[2].to_string();
            results.get_mut(&outer)
                .unwrap()
                .push(Edge {
                    to: colour,
                    from: outer.clone(),
                    count: u64::from_str(&count).unwrap(),
                });
        }
    }

    return results;
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let graph = make_graph(&input);
    let part_1_ans = graph.iter()
        .filter(|(key,_)| has_path(&graph, &key, "shiny gold"))
        .count();
    let part_1_time = SystemTime::now();
    let part_2_ans = count_paths(&graph, "shiny gold") - 1;
    let part_2_time = SystemTime::now();

    println!("Part 1: {}", part_1_ans);
    println!("Part 2: {}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::make_graph;
    use super::has_path;
    use super::count_paths;

    fn example1() -> String {
        String::from(
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.")
    }

    #[test]
    fn example1a() {
        let graph = make_graph(&example1());
        let result = graph.iter()
            .filter(|(key,_)| has_path(&graph, &key, "shiny gold"))
            .count();
        assert_eq!(result, 4);
    }

    #[test]
    fn example1b() {
        let graph = make_graph(&example1());
        let result = count_paths(&graph, "shiny gold") - 1;
        assert_eq!(result, 32);
    }

    fn example2() -> String {
        String::from(
"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.")
    }

    #[test]
    fn example2a() {
        let graph = make_graph(&example2());
        let result = count_paths(&graph, "shiny gold") - 1;
        assert_eq!(result, 126);
    }
}
