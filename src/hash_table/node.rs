/*
Name: node.rs
Version: 1.0
Author: Alex Ho
Date: 2024-03-28
Description: Defines a Node struct and an impl block 
*/

// import course for Node through super
use super::course::Course;

/// Node struct that stores a course, key, and next node
/// Derives Debug trait so that it can be printed
/// Derives Clone trait so that it can be cloned
#[derive(Debug, Clone)]
pub struct Node {
    pub course: Course,
    pub key: usize,
    // Option<> of Node. Can be Some or None
    // Box<Node> is a pointer to a Node - a smart pointer to heap memory
    pub next: Option<Box<Node>>, 
}

/// Node implements the Node struct
impl Node {
    /// Constructor for Node struct
    pub fn new(course: Course, key: usize) -> Self {
        Node {
            course,
            key,
            // Set to None by default
            next: None,
        }
    }
}