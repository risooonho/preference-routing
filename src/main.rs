use std::env;

mod graph;
mod helpers;
mod lp;
mod server;
mod user;

const EDGE_COST_DIMENSION: usize = 3;
const EDGE_COST_TAGS: [&str; EDGE_COST_DIMENSION] = ["Distance", "Height", "UnsuitDist"];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide exactly one parameter, which is the path to the graph file");
    }
    let graph_file = &args[1];
    let graph = graph::parse_graph_file(graph_file).unwrap();
    server::start_server(graph);
}
