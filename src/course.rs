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

