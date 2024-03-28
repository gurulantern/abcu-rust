/*
Name: hash_table.rs
Version: 1.0
Author: Alex Ho
Date: 2024-03-28
Description: Defines a HashTable struct
*/

// Declare usage of course and node
pub mod course;
pub mod node;
use course::Course;
use node::Node;

// Hash table size
const DEFAULT_SIZE: usize = 8;

/// Hash Table struct that stores an array of nodes and a table size
#[derive(Debug)]
pub struct HashTable {
    nodes: Vec<Option<Node>>,
    table_size: usize,
}

impl HashTable {
    /// Constructor for HashTable struct
    pub fn new() -> Self {
        HashTable {
            // Initialize nodes to None
            nodes: vec![None; DEFAULT_SIZE],
            table_size: DEFAULT_SIZE,
        }
    }

    /// Constructor for HashTable struct with a size parameter
    /* Currently not in use so it is commented out
    pub fn with_size(size: usize) -> Self {
        HashTable {
            nodes: vec![None; size],
            table_size: size,
        }
    }
    */

    /// Returns the hash value of a course number
    fn hash(&self, course_number: &str) -> usize {
        // Sum the ASCII values of the characters in the course number
        // Modulo the sum by the table size
        // Return the hash value
        course_number.chars().map(|c| c as usize).sum::<usize>() % self.table_size
    }

    /// Inserts a course into the hash table
    pub fn insert(&mut self, course: Course) {
        // Get the hash value of the course number
        let key = self.hash(&course.course_number);
        // Create a new node with the course and key
        let node = Node::new(course, key);
        // Print the key value
        match node.key {
            key_value => {
                // Use the key value here
                println!("Node key: {}", key_value.to_string());
            }
        }

        // Insert the node into the hash table
        // Time complexity: O(n) in the worst case (n: length of linked list)
        match &mut self.nodes[key] {
            // If the node is already in the hash table, insert it after the last node
            Some(n) => {
                let mut current = n;
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(Box::new(node));
            }
            // If the node is not in the hash table, insert it at the head of the linked list
            None => {
                self.nodes[key] = Some(node);
            }
        }
    }

    /// Prints all courses in the hash table and their prerequisites
    pub fn print_all(&self) {
        // Initialize a vector to store all courses
        let mut all_courses = Vec::new();
        // Iterate through the nodes and add all courses to the vector
        // Time complexity: O(n) where n is the number of nodes in the hash table
        for node in &self.nodes {
            if let Some(n) = node {
                all_courses.push(&n.course);
                let mut current = &n.next;
                // Iterate through the linked list and add all courses to the vector
                while let Some(ref next_node) = current {
                    all_courses.push(&next_node.course);
                    current = &next_node.next;
                }
            }
        }

        // Sort the vector by course number
        // Time complexity: O(n log n) where n is the number of courses
        all_courses.sort_by(|a, b| a.course_number.cmp(&b.course_number));

        // Print all courses and their prerequisites
        // Time complexity: O(n) where n is the number of courses
        for course in all_courses {
            println!(
                "{} | {} | Prerequisites: {}",
                course.course_number,
                course.name,
                course
                    .prerequisites
                    .iter()
                    .map(|prereq| prereq.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    /// Searches for a course in the hash table
    pub fn search(&self, course_number: &str) -> Option<Course> {
        let key = self.hash(course_number);

        // If the node is in the hash table, search for the course
        if let Some(node) = &self.nodes[key] {
            let mut current = node;
            // Iterate through the linked list and search for the course
            // Time complexity: O(n) where n is the number of courses in linked list
            while let Some(ref next_node) = &current.next {
                // If the course is found, return it
                if next_node.course.course_number == course_number {
                    return Some(next_node.course.clone());
                }
                // If the course is not found, move to the next node in the linked list
                current = next_node;
            }
        }
        // If the course is not in the hash table, return None
        None
    }
}