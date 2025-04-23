use crate::storage;

pub fn run(id: usize) {
    let mut tasks = match storage::load_tasks() {
        Ok(tasks) => tasks,
        Err(_) => {
            eprintln!("Failed to load tasks");
            return;
        }
    };

    tasks.retain(|task| task.id != id);

    if let Err(e) = storage::save_tasks(&tasks) {
        eprintln!("Failed to save tasks: {}", e);
    } else {
        println!("Task deleted successfully");
    }
}