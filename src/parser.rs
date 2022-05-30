use core::fmt;
use std::fs;

use crate::graph::Graph;

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

    let _num_vertices = lines[4].parse::<u64>().unwrap();

    let _enable_vert_name = !(lines[5].parse::<u8>().unwrap() == 0);

    let mut nodes: Vec<String> = Vec::new();

    for i in 0..num_nodes {
        nodes.push(lines[(5 + i) as usize].to_string())
    }

    let file_weights = lines[(6 + num_nodes) as usize..(6 + num_nodes * 2) as usize].to_vec();
    let mut weights = vec![vec![0; num_nodes as usize]; num_nodes as usize];

    for (i, node) in file_weights.iter().enumerate() {
        let split = node.split(' ');
        for (j, w) in split.enumerate() {
            weights[i][j] = w.parse::<u32>().unwrap()
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

    let file_vname = lines[(6 + num_nodes * 3) as usize..(6 + (4 * num_nodes)) as usize].to_vec();
    let mut vertex_names = vec![vec![String::new(); num_nodes as usize]; num_nodes as usize];

    for (i, node) in file_vname.iter().enumerate() {
        let split = node.split(' ');
        for (j, w) in split.enumerate() {
            vertex_names[i][j] = w.to_string();
        }
    }

    let default_start = lines[(6 + 4 * num_nodes) as usize]
        .split(' ')
        .collect::<Vec<&str>>()[0]
        .to_string();
    let default_end = lines[(6 + 4 * num_nodes) as usize]
        .split(' ')
        .collect::<Vec<&str>>()[1]
        .to_string();

    let recommended_algo = lines[(6 + 4 * num_nodes + 1) as usize].to_string();
    let description = lines[(6 + 4 * num_nodes + 2) as usize].to_string();

    let graph = Graph {
        version,
        graph_type,
        flags,
        weights,
        connectivity,
        vertex_names,
        default_start,
        default_end,
        recommended_algo,
        description,
    };

    println!("{:?}", graph);

    return Ok(graph);
}
