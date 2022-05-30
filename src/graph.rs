#[derive(Debug)]
pub struct Graph {
    pub version: String,
    pub graph_type: String,
    pub flags: Vec<String>,

    pub weights: Vec<Vec<u32>>,
    pub connectivity: Vec<Vec<bool>>,
    pub vertex_names: Vec<Vec<String>>,

    pub default_start: String,
    pub default_end: String,
    pub recommended_algo: String,
    pub description: String,
}
