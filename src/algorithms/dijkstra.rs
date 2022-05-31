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
    let mut distances: Vec<i32> = vec![(f64::INFINITY) as i32; graph.nodes.len()];
    distances[num_from] = 0;

    while unvisited.contains(&num_to) {
        /*
         * 6: Select unvisited node with smallest tentative value
         */

        let mut current = num_to;
        current = get_smallest_dist(&unvisited, &distances, current);

        /*
         * 3: For each unvisited neighbor, check the distance
         */
        let neighbors: Vec<usize> = get_neighbors(&graph.connectivity, current);

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

/**
 * Get smallest distance from a list of distance
 */
fn get_smallest_dist(unvisited: &HashSet<usize>, distances: &Vec<i32>, current: usize) -> usize {
    let mut ret = current;
    unvisited.iter().for_each(|&f| {
        if distances[f] < distances[ret] {
            ret = f
        }
    });
    return ret;
}

fn get_neighbors(connectivity: &Vec<Vec<bool>>, node: usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = vec![];
    connectivity[node]
        .iter()
        .enumerate()
        .for_each(|(idx, connected)| {
            if *connected {
                neighbors.push(idx);
            }
        });
    return neighbors;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_smallest_dist() {
        let mut unvisited: HashSet<usize> = HashSet::new();
        unvisited.insert(1);
        unvisited.insert(2);
        unvisited.insert(4);
        unvisited.insert(5);
        let distances = vec![0, 20, 10, 30, 40, (f64::INFINITY as i32)];
        let current = 5;
        assert_eq!(get_smallest_dist(&unvisited, &distances, current), 2);
    }

    #[test]
    fn test_get_neighbors() {
        let mut connectivity = vec![vec![false; 7]; 7];
        connectivity[2][3] = true;
        connectivity[2][5] = true;
        connectivity[2][6] = true;
        assert_eq!(get_neighbors(&connectivity, 2), vec![3, 5, 6]);
    }
}
