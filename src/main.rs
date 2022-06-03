use networkr::algorithms::bellmanford::bellmanford;
use networkr::algorithms::dijkstra::dijkstra;
use networkr::create::create_graph;
use networkr::graph::Edge;
use networkr::parser;
fn main() {
    let graph =
        parser::parse_graph("data/graph_matrix.map".to_string()).expect("Error parsing file");
    let from = String::from("A");
    let to = String::from("D");
    let res = dijkstra(graph, from, to);

    let nodes = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
    ];
    let vertices = vec![
        Edge {
            from: String::from("A"),
            to: String::from("B"),
            directed: false,
            weight: 20,
            name: Some(String::from("A->B")),
        },
        Edge {
            from: String::from("A"),
            to: String::from("C"),
            directed: false,
            weight: 30,
            name: Some(String::from("A->C")),
        },
        Edge {
            from: String::from("B"),
            to: String::from("B"),
            directed: false,
            weight: 10,
            name: Some(String::from("B->C")),
        },
        Edge {
            from: String::from("B"),
            to: String::from("D"),
            directed: false,
            weight: 10,
            name: Some(String::from("B->D")),
        },
        Edge {
            from: String::from("C"),
            to: String::from("D"),
            directed: false,
            weight: 10,
            name: Some(String::from("C->D")),
        },
    ];

    let graph = create_graph(&nodes, vertices);
    let res_dji = dijkstra(graph.clone(), "A".to_string(), "D".to_string());
    dbg!(res_dji);
    let res_bmf = bellmanford(graph.clone(), "A".to_string(), "D".to_string());
    dbg!(res_bmf);
}
