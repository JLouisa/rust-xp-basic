fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task = Task {
        title: "Task1".to_string(),
        done: false,
        desc: Some("This is task 1".to_string()),
    };

    let task = Task::new("Task 01");

    println!("{task:#?}");

    Ok(())
}

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool,
    pub desc: Option<String>,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            done: false,
            desc: None,
        }
    }
}
