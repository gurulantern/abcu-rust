mod parser;

use parser::Parser;

const DEFAULT_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct Course {
    course_number: String,
    name: String,
    prerequisites: Vec<String>,
}

impl Course {
    fn new(course_number: String, name: String, prerequisites: Vec<String>) -> Self {
        Course {
            course_number,
            name,
            prerequisites,
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    course: Course,
    key: usize,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(course: Course, key: usize) -> Self {
        Node {
            course,
            key,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct HashTable {
    nodes: Vec<Option<Node>>,
    table_size: usize,
}

impl HashTable {
    pub fn new() -> Self {
        HashTable {
            nodes: vec![None; DEFAULT_SIZE],
            table_size: DEFAULT_SIZE,
        }
    }

    pub fn with_size(size: usize) -> Self {
        HashTable {
            nodes: vec![None; size],
            table_size: size,
        }
    }

    fn hash(&self, course_number: &str) -> usize {
        course_number.chars().map(|c| c as usize).sum::<usize>() % self.table_size
    }

    pub fn insert(&mut self, course: Course) {
        let key = self.hash(&course.course_number);
        let node = Node::new(course, key);

        match &mut self.nodes[key] {
            Some(n) => {
                let mut current = n;
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(Box::new(node));
            }
            None => {
                self.nodes[key] = Some(node);
            }
        }
    }

    pub fn print_all(&self) {
        let mut all_courses = Vec::new();
        for node in &self.nodes {
            if let Some(n) = node {
                all_courses.push(&n.course);
                let mut current = &n.next;
                while let Some(ref next_node) = current {
                    all_courses.push(&next_node.course);
                    current = &next_node.next;
                }
            }
        }

        all_courses.sort_by(|a, b| a.course_number.cmp(&b.course_number));

        for course in all_courses {
            println!(
                "{} | {} | Pregrequisites: {}",
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

    pub fn search(&self, course_number: &str) -> Option<Course> {
        let key = self.hash(course_number);

        if let Some(node) = &self.nodes[key] {
            let mut current = node;
            while let Some(ref next_node) = &current.next {
                if next_node.course.course_number == course_number {
                    return Some(next_node.course.clone());
                }
                current = next_node;
            }
            if current.course.course_number == course_number {
                return Some(current.course.clone());
            }
        }

        None
    }
}

fn load_courses(path: &str, hash_table: &mut HashTable) {
    println!("Loading input file {}", path);

    let parser = Parser::new(path);
    let data = parser.get_file_as_array();

    for row in data {
        let course_number = row[0].clone();
        let name = row[1].clone();
        let prerequisites = row[2..].to_vec();
        let course = Course::new(course_number, name, prerequisites);
        hash_table.insert(course);
    }
}

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
        choice.clear();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");

        match choice.trim() {
            "1" => load_courses(path, &mut course_table),
            "2" => course_table.print_all(),
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
            "9" => {
                println!("Thank you for using the course planner!");
                break;
            }
            _ => println!("{} is not a valid option. Please refer back to a menu.", choice.trim()),
        }
    }
}