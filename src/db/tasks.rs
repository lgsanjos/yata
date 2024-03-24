#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: i32,
    pub project: String,
    pub status: String,
    pub title: String,
}

impl Task {
    pub fn new(id: i32, project: &str, status: &str, title: &str) -> Self {
        Self {
            id,
            project: project.to_string(),
            status: status.to_string(),
            title: title.to_string(),
        }
    }
}


