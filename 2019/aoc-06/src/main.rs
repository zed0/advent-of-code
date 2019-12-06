use std::fs;
use std::env;
use std::collections::HashMap;
use petgraph::Graph;
use petgraph::Undirected;
use petgraph::algo::dijkstra;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut orbit_map = Graph::<i64, i64>::new().into_edge_type::<Undirected>();
    let mut bodies: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

    fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|i| {
            let mut parts = i.trim().split(")").clone();
            (String::from(parts.next().unwrap()), String::from(parts.next().unwrap()))
        })
        .for_each(|edge| {
            // This should be possible to move to a function, but rust.
            let a = match bodies.get(&edge.0) {
                Some(&edge) => edge,
                _ => {let n = orbit_map.add_node(0); bodies.insert(edge.0, n); n},
            };
            let b = match bodies.get(&edge.1) {
                Some(&edge) => edge,
                _ => {let n = orbit_map.add_node(0); bodies.insert(edge.1, n); n},
            };
            orbit_map.add_edge(a, b, 1);
        });

    // Part 1
    let total_distance: i64 = dijkstra(
        &orbit_map,
        *bodies.get("COM").unwrap(),
        None,
        |e| *e.weight(),
    )
    .iter()
    .map(|d| d.1)
    .sum();
    println!("total_distance: {:?}", total_distance);

    // Part 2
    let distance = dijkstra(
        &orbit_map,
        *bodies.get("YOU").unwrap(),
        Some(*bodies.get("SAN").unwrap()),
        |e| *e.weight(),
    )
    .get(bodies.get("SAN").unwrap()).unwrap() - 2;
    println!("distance: {:?}", distance);
}
