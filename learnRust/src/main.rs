extern crate chrono;
use chrono::prelude::*;

enum Priority {
    Low,
    Medium,
    High,
}
impl Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned(),
        }
    }
}

struct Task {
    name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>,
}

impl Task {
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self { name, description, priority, add_time: Local::now() }
    }
    fn print_task(&self) {
        println!(
            "{} | {} | {:?}\n\"{}\"\n",
            self.name,
            self.priority.to_string(),
            self.add_time.format("%d-%m-%Y %H:%M:%S %z").to_string(),
            self.description
        )
    }
}

struct TasksManager {
    tasks: Vec<Task>,
}

impl TasksManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn print_task(&self) {
        for task in &self.tasks {
            println!("trrrr");
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task)
    }

    fn find_task(&self, name: String) -> Option<usize> {
        self.tasks.iter().position(|el| el.name == name)
    }

    fn remove_task(&mut self, name: String) -> Result<String, String> {
        if let Some(i) = self.find_task(name) {
            self.tasks.remove(i);
            Ok(format!("Task {} removed", name))
        } else {
            Err(format!("Task \"{}\" doesn't exist", name))
        }
    }
    fn edit_task(&mut self, name: String, update_task: Task) -> Result<String, String> {
        if let Some(i) = self.find_task(name) {
            if let Some(task) = self.tasks.get_mut(i) {
                task.name = update_task.name;
                Ok(format!("Task {} removed", name))
            } else {
                Ok(format!("Task {} removed", name))
            }
        } else {
            Err(format!("Task \"{}\" doesn't exist", name))
        }
    }
}

fn main() {
    let mut task = Task::new("my_task".to_string(), "i Need it...".to_string(), Priority::High);
    // task.print_task();
}
