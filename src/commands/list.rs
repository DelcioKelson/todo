use crate::storage;

pub fn run () {
    let tasks = match storage::load_tasks() {
        Ok(tasks) => tasks,
        Err(_) => {
            eprintln!("Failed to load tasks");
            return;
        }
    };

    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            println!("ID: {}, Name: {}, Description: {}, Status: {:?}, Created At: {}", 
                task.id, task.name, task.description, task.status, task.created_at);
        }
    }
}