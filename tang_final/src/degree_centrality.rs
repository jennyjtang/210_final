use std::collections::HashMap;

// Function to calculate degree centrality for a maximum of n nodes
pub fn calculate(graph: &HashMap<i32, Vec<i32>>, max_nodes: usize) -> HashMap<i32, usize> {
    let mut degree_centrality = HashMap::new();
    for (node, neighbors) in graph.iter() {
        let degree = neighbors.len();
        degree_centrality.insert(*node, degree);
    }
    // Sort degree centrality by value and take top max_nodes
    let mut sorted_degree_centrality: Vec<_> = degree_centrality.into_iter().collect();
    sorted_degree_centrality.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_degree_centrality.truncate(max_nodes);
    sorted_degree_centrality.into_iter().collect()
}

// Function to print out the degree centrality values as a histogram graph
pub fn print_degree_centrality(degree_centrality: &HashMap<i32, usize>) {
    // Find the maximum degree centrality
    let max_degree = *degree_centrality.values().max().unwrap_or(&0);
    
    for (node, &degree) in degree_centrality {
        let bar_length = (degree as f64 / max_degree as f64 * 10.0) as usize; // Scale the bar length
        
        // Print node and its corresponding histogram
        println!("Node {}: {}{}", node, "=".repeat(bar_length), degree);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_degree_centrality() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1, 3]);
        graph.insert(3, vec![1, 2]);

        let degree_centrality = calculate(&graph, 2);

        let expected_degree_centrality: HashMap<i32, usize> = [(1, 2), (2, 2), (3, 2)].iter().cloned().collect();

        assert_eq!(degree_centrality, expected_degree_centrality);
    }
}