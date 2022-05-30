use networkr::algorithms::dijkstra::dijkstra;
use networkr::parser;
fn main() {
    let graph =
        parser::parse_graph("data/graph_matrix.map".to_string()).expect("Error parsing file");
    let from = String::from("A");
    let to = String::from("D");
    dijkstra(graph, from, to);
}
