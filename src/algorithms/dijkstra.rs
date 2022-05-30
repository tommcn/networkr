use crate::graph::Graph;
use std::collections::HashSet;

pub fn dijkstra(graph: Graph, from: String, to: String) -> u64 {
    let num_from: usize = graph.nodes.iter().position(|x| x == &from).unwrap();
    let num_to: usize = graph.nodes.iter().position(|x| x == &to).unwrap();

    /*
     * 1: Mark all nodes as unvisited
     */
    let mut unvisited = HashSet::new();

    (0..graph.nodes.len()).for_each(|i| {
        unvisited.insert(i);
    });

    /*
     * 2: Assign to all nodes a tentative distance
     * Set the initial node as the current
     */
    let mut distances: Vec<u32> = vec![(f64::INFINITY) as u32; graph.nodes.len()];
    distances[num_from] = 0;

    while unvisited.contains(&num_to) {
        /*
         * 6: Select unvisited node with smallest tentative value
         */
        let mut current = num_to;
        unvisited.iter().for_each(|&f| {
            if distances[f] < distances[current] {
                current = f
            }
        });

        /*
         * 3: For each unvisited neighbor, check the distance
         */
        let mut neighbors: Vec<usize> = vec![];
        graph.connectivity[current]
            .iter()
            .enumerate()
            .for_each(|(idx, connected)| {
                if *connected {
                    neighbors.push(idx);
                }
            });
        neighbors
            .iter()
            .filter(|&n| unvisited.contains(n))
            .for_each(|&n| {
                if (distances[current] + graph.weights[current][n]) < distances[n] {
                    distances[n] = distances[current] + graph.weights[current][n];
                }
            });
        /*
         * 4: Mark current node as visited
         */
        unvisited.remove(&current);
    }

    return distances[num_to] as u64;
}
