/*
Name: course.rs
Version: 1.0
Author: Alex Ho
Date: 2024-03-28
Description: Defines a Course struct and an impl block 
*/

/// Course struct that stores a course number, name, and prerequisites
/// Derives Debug trait so that it can be printed
/// Derives Clone trait so that it can be cloned
#[derive(Debug, Clone)]
pub struct Course {
    pub course_number: String,
    pub name: String,
    pub prerequisites: Vec<String>,
}

/// Course implements the Course struct
impl Course {
    /// Constructor for Course struct
    pub fn new(course_number: String, name: String, prerequisites: Vec<String>) -> Self {
        Course {
            course_number,
            name,
            prerequisites,
        }
    }
}