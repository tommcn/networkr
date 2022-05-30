use networkr::parser;
fn main() {
    parser::parse_graph("data/graph_matrix.map".to_string()).expect("Error parsing file");
}
