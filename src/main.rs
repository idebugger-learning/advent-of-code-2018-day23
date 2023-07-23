use petgraph::graph::UnGraph;

mod parser;

pub type Point = (isize, isize, isize);

#[derive(Debug, Clone, Copy)]
pub struct Nanobot {
    position: Point,
    radius: usize,
}

fn main() {
    // let input = include_str!("./data/example.txt");
    let input = include_str!("./data/input.txt");
    let (leftover, nanobots) = parser::parse_input(input).unwrap();
    assert!(leftover.is_empty());

    println!("Nanobots: {:?}", nanobots);
    let graph = into_graph(&nanobots);
    println!("Graph: {:?}", graph);

    let largest_radius = nanobots.iter().max_by_key(|nanobot| nanobot.radius).unwrap();
    println!("Largest radius: {:?}", largest_radius);

    let larges_radius_index = graph.node_indices().find(|&index| graph[index] == largest_radius.position).unwrap();
    let largest_radius_neighbors_positions = graph.neighbors(larges_radius_index).map(|index| graph[index]).collect::<Vec<Point>>();
    println!("Largest radius neighbors positions: {:?}", largest_radius_neighbors_positions);
    println!("Largest radius neighbors positions count: {:?}", largest_radius_neighbors_positions.len());
}

fn into_graph(nanobots: &Vec<Nanobot>) -> UnGraph<Point, usize> {
    let mut graph = UnGraph::new_undirected();

    for nanobot in nanobots {
        graph.add_node(nanobot.position);
    }

    for nanobot in nanobots.clone() {
        for other_nanobot in nanobots.clone() {
            let distance = manhattan_distance(nanobot.position, other_nanobot.position);
            if distance <= nanobot.radius {
                let nanobot_index = graph.node_indices().find(|&index| graph[index] == nanobot.position).unwrap();
                let other_nanobot_index = graph.node_indices().find(|&index| graph[index] == other_nanobot.position).unwrap();
                if !graph.contains_edge(nanobot_index, other_nanobot_index) {
                    graph.add_edge(nanobot_index, other_nanobot_index, distance);
                }
            }
        }
    }

    graph
}

fn manhattan_distance(a: Point, b: Point) -> usize {
    let (x1, y1, z1) = a;
    let (x2, y2, z2) = b;

    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as usize
}