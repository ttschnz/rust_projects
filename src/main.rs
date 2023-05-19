use rand::Rng;
use ttschnz::algorythms::graph_theory::{dijkstra::Vertices, Solver};

fn generate_graph(n: usize, x: usize) -> Vec<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

    for node in 0..n {
        let outgoing_connections = rng.gen_range(1..=x);
        for _ in 0..outgoing_connections {
            let target = rng.gen_range(0..n);
            graph[node].push(target);
        }
    }

    graph
}
fn main() {
    let config = generate_graph(500, 3);
    let vertices = Vertices::from_config(config).expect("could not create vertices from config");

    println!("{:?}", vertices.shortest_path(&0, &2));
}
