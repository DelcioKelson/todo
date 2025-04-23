use crate::storage;

pub fn run( id: usize, name: Option<String>, description: Option<String>, status: Option<String>) {
    // Check if the status is valid
    if let Some(status) = status {
        if let Some(task_status) = crate::task::TaskStatus::from_str(&status) {
            // edit the task status
            println!("Task status editd to: {:?}", task_status);
        } else {
            eprintln!("Invalid task status: {}", status);
            return;
        }
    }
    
    let mut tasks = match storage::load_tasks() {
        Ok(tasks) => tasks,
        Err(_) => {
            eprintln!("Failed to load tasks");
            return;
        }
    };

    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        if let Some(name) = name {
            task.name = name;
        }
        if let Some(description) = description {
            task.description = description;
        }

        if let Err(e) = storage::save_tasks(&tasks) {
            eprintln!("Failed to save tasks: {}", e);
        } else {
            println!("Task editd successfully");
        }
    } else {
        eprintln!("Task with ID {} not found", id);
    }
}