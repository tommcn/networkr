use crate::graph::Graph;

pub fn bellmanford(graph: Graph, from: String, to: String) -> i64 {
    let num_from: usize = graph.nodes.iter().position(|x| x == &from).unwrap();
    let num_to: usize = graph.nodes.iter().position(|x| x == &to).unwrap();

    let mut distances = vec![f64::INFINITY as i64; graph.nodes.len()];
    let mut predecessors = vec![None as Option<usize>; graph.nodes.len()];
    distances[num_from] = 0;

    let len = graph.nodes.len();

    for _ in 0..len {
        graph.edges.clone().into_iter().for_each(|e| {
            let f: usize = graph.nodes.iter().position(|x| x == &e.from).unwrap();
            let t: usize = graph.nodes.iter().position(|x| x == &e.to).unwrap();

            let tempdist = distances[f] + (graph.weights[f][t] as i64);
            if tempdist < distances[t] {
                distances[t] = tempdist;
                predecessors[t] = Some(f);
            }
        })
    }

    graph.edges.clone().into_iter().for_each(|e| {
        let f: usize = graph.nodes.iter().position(|x| x == &e.from).unwrap();
        let t: usize = graph.nodes.iter().position(|x| x == &e.to).unwrap();
        if distances[f] + (graph.weights[f][t] as i64) < distances[t] {
            distances[t] = f64::NEG_INFINITY as i64;
        }
    });

    return distances[num_to];
}
