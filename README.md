# CS-300 Hash Table Implementation for ABCU Advising

This is a simple course planner program written in Rust. It allows users to load course data from a file, print the list of courses, and search for specific courses by their course numbers.

## How to Use
### Prerequisites
Make sure you have Rust installed on your system. If not, you can download and install it from here.

### Installation
Clone this repository to your local machine:

```
git clone <repository_url>
```
### Usage
Navigate to the project directory:

```
cd abcu-rust
```

Compile the Rust code:

```
cargo build
```

Run the program:

```
cargo run
```

### Options
Load Data Structure (1): Loads course data from a file specified in the program.
Print Course list (2): Prints the list of all courses loaded into the system.
Print Course (3): Allows you to search for a specific course by its course number.
Exit Program (9): Quits program

### Input Data Format
The input file should be a txt file where each row represents a course and contains the course number, name, and optional prerequisites.

Example input file format:

```
CSCI200,Introduction to Computer Science,
CSCI300,Data Structures and Algorithms,CSCI200
```