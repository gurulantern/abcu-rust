/*
Name: main.rs
Version: 2.0
Author: Alex Ho
Date: 2024-03-27
Description: Defines a Course, Node, and Hash Table struct and impl blocks
Contains the main function that loads data from a txt file, prints course data
from the Hash Table with all prerequisites, and looks for specific courses to print
at the user's request.
*/

mod hash_table;
use hash_table::HashTable;
use hash_table::course::Course; 

mod parser;
use parser::Parser;

/// Loads the courses from a file into the hash table
fn load_courses(path: &str, hash_table: &mut HashTable) {
    println!("Loading input file {}", path);

    // Initialize a parser for the input file
    let parser = Parser::new(path);
    let data = parser.get_file_as_array();

    // Iterate through the rows in the data array from the Parser and insert each course into the hash table
    // Time complexity: O(n) where n is the number of rows in the input file
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