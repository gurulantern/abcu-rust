/*
Name: main.rs
Version: 2.0
Author: Alex Ho
Date: 2024-03-27
Description: Defines a Course, Node, and Hasht Table struct and impl blocks
Contains main function that loads data from a txt file and prints the data for 
the user.
*/
mod parser;
use parser::Parser;


//============================================================================
// Global definitions visible to all methods and classes
//============================================================================

// Hash table size
const DEFAULT_SIZE: usize = 8;

/// Course struct that stores a course number, name, and prerequisites
/// Derives Debug trait so that it can be printed
/// Derives Clone trait so that it can be cloned
#[derive(Debug, Clone)]
pub struct Course {
    course_number: String,
    name: String,
    prerequisites: Vec<String>,
}

/// Course implements the Course struct
impl Course {
    /// Constructor for Course struct
    fn new(course_number: String, name: String, prerequisites: Vec<String>) -> Self {
        Course {
            course_number,
            name,
            prerequisites,
        }
    }
}

/// Node struct that stores a course, key, and next node
/// Derives Debug trait so that it can be printed
/// Derives Clone trait so that it can be cloned
#[derive(Debug, Clone)]
struct Node {
    course: Course,
    key: usize,
    // Option<> of Node. Can be Some or None
    // Box<Node> is a pointer to a Node - a smart pointer to heap memory
    next: Option<Box<Node>>, 
}

/// Node implements the Node struct
impl Node {
    /// Constructor for Node struct
    fn new(course: Course, key: usize) -> Self {
        Node {
            course,
            key,
            // Set to None by default
            next: None,
        }
    }
}

//============================================================================
// Hash Table class definition
//============================================================================

/// Hash Table struct that stores an array of nodes and a table size
#[derive(Debug)]
pub struct HashTable {
    nodes: Vec<Option<Node>>,
    table_size: usize,
}

/// HashTable implements the HashTable struct
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
    pub fn with_size(size: usize) -> Self {
        HashTable {
            nodes: vec![None; size],
            table_size: size,
        }
    }

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
        all_courses.sort_by(|a, b| a.course_number.cmp(&b.course_number));

        // Print all courses and their prerequisites
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

/// Loads the courses from a file into the hash table
fn load_courses(path: &str, hash_table: &mut HashTable) {
    println!("Loading input file {}", path);

    // Initialize a parser for the input file
    let parser = Parser::new(path);
    let data = parser.get_file_as_array();

    // Iterate through the rows in the data array from the Parser and insert each course into the hash table
    for row in data {
        let course_number = row[0].clone();
        let name = row[1].clone();
        let prerequisites = row[2..].to_vec();
        let course = Course::new(course_number, name, prerequisites);
        hash_table.insert(course);
    }
}

/// Main function
fn main() {
    let path = "ABCU_Advising_Program_Input.txt";
    let mut course_table = HashTable::new();
    let mut course_number = String::new();
    let mut choice = String::new();

    loop {
        println!("Welcome to the course planner.\n");
        println!("  1. Load Data Structure");
        println!("  2. Print Course list");
        println!("  3. Print Course");
        println!("  9. Exit");

        println!("What would you like to do?");
        // Clear the choice variable
        choice.clear();
        // Read the choice from the user
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");
        
        // Check the choice and perform the corresponding action
        match choice.trim() {
            // Load the data structure
            "1" => load_courses(path, &mut course_table),
            // Print the course list
            "2" => course_table.print_all(),
            // Print a specific course
            "3" => {
                println!("Please enter the course number of the course you are looking for.");
                println!("Example: CSCI200 or math201");
                course_number.clear();
                std::io::stdin()
                    .read_line(&mut course_number)
                    .expect("Failed to read input.");
                if let Some(course) = course_table.search(course_number.trim()) {
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
                } else {
                    println!("Course Number {} not found. Please try again.", course_number);
                }
            }
            // Exit the program
            "9" => {
                println!("Thank you for using the course planner!");
                break;
            }
            // Invalid option
            _ => println!("{} is not a valid option. Please refer back to the menu.", choice.trim()),
        }
    }
}