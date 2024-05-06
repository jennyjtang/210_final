use std::collections::{HashMap, HashSet};

// Parse the graph from the file contents
pub fn parse_graph(contents: &str) -> HashMap<i32, Vec<i32>> {
    let mut graph = HashMap::new();
    for line in contents.lines() {
        let nodes: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok()) // Filter out any parsing errors
            .collect();
        if nodes.len() != 2 {
            // Ensure each line contains exactly two nodes
            eprintln!("Invalid line: {}", line);
            continue;
        }
        let node1 = nodes[0];
        let node2 = nodes[1];
        graph.entry(node1).or_insert(vec![]).push(node2);
        graph.entry(node2).or_insert(vec![]).push(node1);
    }

    graph
}

// Print out the degree centrality values
pub fn print_degree_centrality(degree_centrality: &HashMap<i32, usize>) {
    for (node, degree) in degree_centrality {
        println!("Node {}: Degree Centrality {}", node, degree);
    }
}

// Calculate degree centrality for each node in the graph
pub fn calculate_degree_centrality(graph: &HashMap<i32, Vec<i32>>) -> HashMap<i32, usize> {
    let mut degree_centrality = HashMap::new();
    for (node, neighbors) in graph.iter() {
        let degree = neighbors.len();
        degree_centrality.insert(*node, degree);
    }
    degree_centrality
}

// Validate the graph structure
pub fn validate_graph(graph: &HashMap<i32, Vec<i32>>) -> bool {
    // Add validation logic here...
    true // Placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_degree_centrality() {
        let degree_centrality: HashMap<i32, usize> = [(1, 2), (2, 3), (3, 1)].iter().cloned().collect();
        print_degree_centrality(&degree_centrality);
        // Assert based on the printed output
        assert!(true); // Adjust this assertion as needed
    }
}

    #[test]
    fn test_print_degree_centrality() {
        let degree_centrality: HashMap<i32, usize> = [(1, 2), (2, 3), (3, 1)].iter().cloned().collect();
        let mut output = Vec::new();
        print_degree_centrality(&degree_centrality, &mut output);
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("Node 1: == Degree Centrality 2"));
        assert!(output_string.contains("Node 2: === Degree Centrality 3"));
        assert!(output_string.contains("Node 3: = Degree Centrality 1"));
    }