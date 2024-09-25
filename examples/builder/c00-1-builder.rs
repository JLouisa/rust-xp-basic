fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task = Task::default();
    println!("{task:#?}");

    // Using struct literals
    let task2 = Task {
        done: true,
        ..Task::new("Cool Task")
    };
    println!("{task2:#?}");

    // While Implementing Default can be beneficial
    let task3: Option<Task> = None;
    let task3 = task3.unwrap_or_default();
    println!("{task3:#?}");

    // Also with struct literals
    let task4 = Task {
        done: true,
        ..Default::default()
    };
    println!("{task4:#?}");

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

// Implement Default of Task
impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            done: false,
            desc: None,
        }
    }
}
