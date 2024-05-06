mod graph;
mod bfs;
mod degree_centrality;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Error};


//    let mut file = File::open("twitter_combined.txt").expect("Can't open file!");
//
//    let mut contents = String::new();
//    file.read_to_string(&mut contents)
//        .expect("Cannot read file!");
//
//    print!("{}", contents);


fn main() {
    // Read the contents of the file into a string
    let contents = match read_file("twitter_combined.txt") {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    // Parse the contents into a graph
    let graph = graph::parse_graph(&contents);

    // Calculate degree centrality for a maximum of 6 nodes
    let max_nodes = 6;
    let degree_centrality = degree_centrality::calculate(&graph, max_nodes);

    // Print out the degree centrality values
    graph::print_degree_centrality(&degree_centrality);

    // Perform BFS from a source node
    let source_node = 1; // Example source node
    let bfs_result = bfs::breadth_first_search(&graph, source_node);
    println!("BFS result: {:?}", bfs_result);

}

// Function to read contents from a file
fn read_file(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}