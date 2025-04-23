use crate::task::Task;
use std::{fs, io::Result};

const STORAGE_FILE: &str = "tasks.json";

pub fn load_tasks() -> Result<Vec<Task>> {
    if let Ok(content) = fs::read_to_string(STORAGE_FILE){
        let tasks: Vec<Task> = serde_json::from_str(&content)?;
        return Ok(tasks);
    }
    Ok(vec![])
}

pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    let content = serde_json::to_string(tasks)?;
    fs::write(STORAGE_FILE, content)?;
    Ok(())
}