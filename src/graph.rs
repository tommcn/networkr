#[derive(Debug)]
pub struct Graph {
    /**
     The version of the graph
    */
    pub version: String,
    /**
     The type of the graph, NV or MATRIX
    */
    pub graph_type: String,
    /**
     The flags for the graph
    */
    pub flags: Vec<String>,

    /*
     The names of the nodes
    */
    pub nodes: Vec<String>,
    /*
     The weights for each of the vertices as an u32, as a NxN matrix
    */
    pub weights: Vec<Vec<i32>>,
    /*
     Whether or not both nodes are connected as a NxN matrix
    */
    pub connectivity: Vec<Vec<bool>>,
    /*
     The name of the vertices
    */
    pub vertex_names: Vec<Vec<String>>,

    /*
     Default start node for the algorithms
    */
    pub default_start: String,
    /*
     Default end node for the algorithms
    */
    pub default_end: String,
    /*
     Recommended algorithm to parse the map
    */
    pub recommended_algo: String,
    /*
     Description of the map
    */
    pub description: String,
}

#[derive(Debug)]
pub struct Vertex {
    pub from: String,
    pub to: String,
    pub weight: i32,
    pub directed: bool,
    pub name: Option<String>,
}
