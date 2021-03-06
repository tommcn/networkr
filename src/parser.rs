use core::fmt;
use std::fs;

use crate::graph::{Edge, Graph};

type Result<T> = std::result::Result<T, InvalidGraph>;

#[derive(Debug, Clone)]
pub struct InvalidGraph;

impl fmt::Display for InvalidGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Graph File")
    }
}

pub fn parse_graph(filename: String) -> Result<Graph> {
    let content = fs::read_to_string(filename).expect("Error reading file contents");
    let lines: Vec<&str> = content.split("\n").map(|l| l).collect();

    let version = lines[0].to_string();
    if version != "v3" {
        println!("Invalid graph version: expected v3");
        return Err(InvalidGraph);
    }

    let graph_type = lines[1].to_string();
    if graph_type != "MATRIX" {
        if graph_type == "NV" {
            println!("NV file type not yet implemented")
        } else {
            println!("File type not yet implemented")
        }
        return Err(InvalidGraph);
    }

    let flags: Vec<String> = lines[2].split(' ').into_iter().map(|s| s.into()).collect();

    let num_nodes = lines[3].parse::<u64>().unwrap();

    let _num_edges = lines[4].parse::<u64>().unwrap();

    let enable_edge_name = !(lines[5].parse::<u8>().unwrap() == 0);

    let mut nodes: Vec<String> = Vec::new();

    for i in 0..num_nodes {
        nodes.push(lines[(6 + i) as usize].to_string())
    }

    let file_weights = lines[(6 + num_nodes) as usize..(6 + num_nodes * 2) as usize].to_vec();
    let mut weights = vec![vec![0; num_nodes as usize]; num_nodes as usize];

    for (i, node) in file_weights.iter().enumerate() {
        let split = node.split(' ');
        for (j, w) in split.enumerate() {
            weights[i][j] = w.parse::<i32>().unwrap()
        }
    }

    let file_connectivities =
        lines[(6 + num_nodes * 2) as usize..(6 + (3 * num_nodes)) as usize].to_vec();
    let mut connectivity = vec![vec![false; num_nodes as usize]; num_nodes as usize];

    for (i, node) in file_connectivities.iter().enumerate() {
        let split = node.split(' ');
        for (j, w) in split.enumerate() {
            connectivity[i][j] = !(w.parse::<u32>().unwrap() == 0)
        }
    }

    let mut edge_names = vec![vec![String::new(); num_nodes as usize]; num_nodes as usize];

    if enable_edge_name {
        let file_vname =
            lines[(6 + num_nodes * 3) as usize..(6 + (4 * num_nodes)) as usize].to_vec();

        for (i, node) in file_vname.iter().enumerate() {
            let split = node.split(' ');
            for (j, w) in split.enumerate() {
                edge_names[i][j] = w.to_string();
            }
        }
    }
    let mut edges: Vec<Edge> = vec![];
    for y in 0..connectivity.len() {
        for x in 0..connectivity[y].len() {
            if connectivity[y][x] {
                let name = if enable_edge_name {
                    Some(edge_names[y][x].to_owned())
                } else {
                    Some(format!("{} -> {}", nodes[y], nodes[x]))
                };
                if enable_edge_name {
                    edges.push(Edge {
                        from: nodes[y].to_owned(),
                        to: nodes[x].to_owned(),
                        weight: weights[x][y],
                        directed: true,
                        name,
                    })
                }
            }
        }
    }

    let offset = 6 + (if enable_edge_name { 4 } else { 3 }) * num_nodes;

    let default_start = lines[(offset) as usize].split(' ').collect::<Vec<&str>>()[0].to_string();
    let default_end = lines[(offset) as usize].split(' ').collect::<Vec<&str>>()[1].to_string();

    let recommended_algo = lines[(offset + 1) as usize].to_string();
    let description = lines[(offset + 2) as usize].to_string();

    let graph = Graph {
        version,
        graph_type,
        flags,
        nodes,
        weights,
        connectivity,
        edges,
        edge_names,
        default_start,
        default_end,
        recommended_algo,
        description,
    };

    return Ok(graph);
}
