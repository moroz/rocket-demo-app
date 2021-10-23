use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    id: u64,
    title: String,
    description: Option<String>,
    done: bool,
}

impl Task {
    pub fn new(id: u64, title: &str, description: Option<&str>) -> Self {
        let description = match description {
            Some(desc) => Some(String::from(desc)),
            None => None,
        };
        Self {
            id,
            description,
            title: String::from(title),
            done: false,
        }
    }
}
