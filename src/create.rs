use crate::graph::{Graph, Vertex};
use std::collections::HashMap;

pub fn create_graph(nodes: &Vec<String>, vertices: Vec<Vertex>) -> Graph {
    let version = String::from("v3");
    let graph_type = String::from("MATRIX");
    let flags = vec![String::from("F"), String::from("A")];

    let mut node_map = HashMap::new();
    nodes.iter().enumerate().for_each(|(idx, n)| {
        node_map.insert(n, idx);
    });

    let nodes_len = nodes.len();
    let mut weights: Vec<Vec<i32>> = vec![vec![0; nodes_len]; nodes_len];
    let mut connectivity: Vec<Vec<bool>> = vec![vec![false; nodes_len]; nodes_len];
    let mut vertex_names: Vec<Vec<String>> = vec![vec![String::new(); nodes_len]; nodes_len];

    vertices.iter().for_each(|v| {
        let from = node_map.get(&v.from).unwrap().to_owned();
        let to = node_map.get(&v.to).unwrap().to_owned();

        weights[from][to] = v.weight;
        connectivity[from][to] = true;
        vertex_names[from][to] = v.name.as_ref().unwrap_or(&"0".to_string()).to_string();
        if !v.directed {
            weights[to][from] = v.weight;
            connectivity[to][from] = true;
            vertex_names[to][from] = v.name.as_ref().unwrap_or(&"0".to_string()).to_string();
        }
    });

    return Graph {
        version,
        graph_type,
        flags,
        nodes: nodes.to_vec(),
        weights,
        connectivity,
        vertex_names,
        default_start: nodes.first().unwrap().to_owned(),
        default_end: nodes.last().unwrap().to_owned(),
        recommended_algo: String::new(),
        description: String::from("Generated automatically"),
    };
}
