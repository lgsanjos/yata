#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: i32,
    pub project: String,
    pub status: String,
    pub title: String,
    pub line_number: u32,

    pub weight: u32,
    pub duration: u32,
}

impl Task {
    pub fn new(id: i32, project: &str, status: &str, title: &str, line_number: u32) -> Self {
        Self {
            id,
            project: project.to_string(),
            status: status.to_string(),
            title: title.to_string(),
            line_number,
            weight: 0,
            duration: 0,
        }
    }
}
