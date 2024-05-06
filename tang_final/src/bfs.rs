use std::collections::{HashMap, VecDeque};

// Function to perform breadth-first search on the graph
pub fn breadth_first_search(graph: &HashMap<i32, Vec<i32>>, start_node: i32) -> HashMap<i32, i32> {

    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start_node, 0);
    queue.push_back(start_node);

    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];

        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    queue.push_back(neighbor);
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadth_first_search() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1, 3]);
        graph.insert(3, vec![1, 2]);

        let bfs_result = breadth_first_search(&graph, 1);

        let expected_result: HashMap<i32, i32> = [(1, 0), (2, 1), (3, 1)].iter().cloned().collect();

        assert_eq!(bfs_result, expected_result);
    }
}

