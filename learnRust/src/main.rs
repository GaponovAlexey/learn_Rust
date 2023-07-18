use std::io::Write;

use chrono::{ DateTime, Local };

enum Priority {
    Low,
    Medium,
    High,
}
impl Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::High => "High".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::Low => "Low".to_owned(),
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
    fn new_from_console() -> Self {
        let name = ConsoleManager::input("Enter your task name").expect("err");
        let description = ConsoleManager::input("Enter your task description").expect("err");

        let priority = match
            ConsoleManager::input("Enter new task priority").expect("err").as_str()
        {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => {
                println!("i don't understand");
                Priority::Low
            }
        };
        Self { name, description, priority, add_time: Local::now() }
    }
    fn print_task(&self) {
        println!(
            "\n****************************\n{} | {} | {:?}\n\"{}\"\n****************************\n",
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
        Self { tasks: Vec::new() }
    }

    fn print_task(&self) {
        for task in &self.tasks {
            println!("from TasksManager");
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task)
    }

    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|el| el.name.as_str() == name)
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(i) = self.find_task(&name) {
            self.tasks.remove(i);
            Ok(format!("Task {} removed", name))
        } else {
            Err(format!("Task {} doesn't removed", name))
        }
    }

    fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String> {
        if let Some(i) = self.find_task(&name) {
            match self.tasks.get_mut(i) {
                Some(task) => {
                    task.name = updated_task.name;
                    task.description = updated_task.description;
                    task.priority = updated_task.priority;
                    Ok(format!("Task {} updated successfully", name))
                }
                None => Err(format!("Task \"{}\" doesn't exist", name)),
            }
        } else {
            Err(format!("Task \"{}\" doesn't exist", name))
        }
    }
}

struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(),
            menu_options: vec![
                "Add Task".to_owned(),
                "Find task".to_owned(),
                "Edit task".to_owned(),
                "Remove task".to_owned(),
                "Print task".to_owned(),
                "Store task to file".to_owned(),
                "Read task from file".to_owned()
            ],
        }
    }
    fn input(query: &str) -> std::io::Result<String> {
        print!("{:?}", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn print_menu(&self) {
        for (index, menu_options) in self.menu_options.iter().enumerate() {
            println!("{:?}\nOptions{}", index + 1, menu_options);
        }
    }

    fn process_command(&mut self) {
        match Self::input("Enter command index") {
            Ok(command) => {
                match command.as_str() {
                    "1" => { self.tasks_manager.add_task(Task::new_from_console()) }
                    "2" => {
                        let name = Self::input("enter task name to find").expect("err");
                        match self.tasks_manager.find_task(name.as_str()) {
                            Some(i) => {
                                self.tasks_manager.tasks.get(i).expect("not find").print_task()
                            }
                            None => println!("Nothing found"),
                        }
                    }
                    "3" => {
                        let name = Self::input("enter task name to find").expect("err");
                        match self.tasks_manager.edit_task(name.as_str(), Task::new_from_console()) {
                            Ok(msg) => println!("edited ok {:?}", msg),
                            Err(msg) => println!("{:?}", msg),
                        }
                    }
                    "4" => {
                        let name = Self::input("enter task name to remove").expect("err");
                        match self.tasks_manager.remove_task(name.as_str()) {
                            Ok(msg) => println!("{:?}", msg),
                            Err(msg) => println!("{:?}", msg),
                        }
                    }
                    "5" => { self.tasks_manager.print_task() }
                    "6" => {}
                    "7" => {}
                    _ => { println!("i don't understand this command") }
                }
            }
            Err(e) => println!("Err {:?}", e),
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
        print!("");
    }
}
