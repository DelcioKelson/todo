use crate::task::Task;
use crate::task::TaskStatus;
use crate::storage;

pub fn run (name: String, description: String) {

    let mut tasks = match storage::load_tasks() {
        Ok(tasks) => tasks,
        Err(_) => {
            eprintln!("Failed to load tasks, creating a new list");
            vec![]
        }
    };

    let id = tasks.len() + 1; // Simple ID generation

    let task = Task {
        id,
        name,
        description,
        status: TaskStatus::Pending,
        created_at: chrono::Local::now().to_string(),
    };

    tasks.push(task);

    if let Err(e) = storage::save_tasks(&tasks) {
        eprintln!("Failed to save tasks: {}", e);
    } else {
        println!("Task added successfully");
    }
}