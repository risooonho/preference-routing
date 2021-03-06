use std::env;

mod config;
mod graph;
mod helpers;
mod lp;
mod server;
mod user;

const EDGE_COST_DIMENSION: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide exactly one parameter, which is the path to the graph file");
    }
    let graph = graph::parse_graph_file(&args[1]).unwrap();
    server::start_server(graph);
}
