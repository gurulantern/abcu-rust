/*
Name: parser.rs
Version: 2.0
Author: Alex Ho
Date: 2024-03-27
Description: Defines a Parser struct and an impl block that reads data 
from a txt file. impl Parser returns a 2D vector of lines from the txt.
*/

// Import File for opening and reading files
use std::fs::File;
// Import BufRead and BufReader for a buffered reader to read line-by-line
// BufReader wraps a reader with buffering to implement BufRead
use std::io::{BufRead, BufReader};
// Imports the Read trait to use .read_exact
use std::io::Read;

/// Parser struct that uses a filename string to open the file for impl
pub struct Parser {
    filename: String,
}

/// Parser implements the Parser struct
impl Parser {
    /// Constructor for Parser struct
    pub fn new(filename: &str) -> Self {
        Parser {
            // Ensures filename is a string
            filename: filename.to_string(),
        }
    }

    /// Returns a 2D vector of lines from the txt file
    pub fn get_file_as_array(&self) -> Vec<Vec<String>> {
        // Opens file and stores in file variable
        let file = match File::open(&self.filename) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Error opening file."); // Prints error message
                return Vec::new(); // Returns empty vector
            }
        };

        // Skip first 3 bytes BOM
        let mut file = BufReader::new(file);
        let mut bom = [0; 3];
        // Read 3 bytes into bom
        file.read_exact(&mut bom).unwrap();

        // Initialize data_array
        let mut data_array = Vec::new();

        // Iterate through each line in file
        for line in file.lines() {
            if let Ok(line) = line {
                /*  Split line by commas and push to data_array
                .trim() to remove whitespace from line
                .map(|s| s.to_string()) converts each string to a string
                .collect() converts the iterator to a vector
                */
                let row: Vec<String> = line.trim().split(',').map(|s| s.to_string()).collect();
                // Push row to data_array
                data_array.push(row);
            }
        }
        
        data_array
    }
}

