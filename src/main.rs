use std::{
    fmt::Debug,
    fs::{self},
    path::Path,
};

use chrono::{DateTime, Local};
use json::{JsonValue, object};

#[derive(Debug)]
enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Task {
    fn new(id: u32, desc: String) -> Self {
        Task {
            id: id,
            description: desc,
            status: TaskStatus::ToDo,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }
    fn updated(&mut self) {
        self.updated_at = Local::now();
    }

    fn update(&mut self, desc: String) {
        self.description = desc;
        self.updated();
    }
    fn mark_in_progress(&mut self) {
        self.status = TaskStatus::InProgress;
        self.updated();
    }
    fn mark_done(&mut self) {
        self.status = TaskStatus::Done;
        self.updated();
    }

    fn print(&self) {
        println!(
            "TASK: {}\t\"{}\"\t{:?}\t{}\t{}",
            self.id, self.description, self.status, self.created_at, self.updated_at,
        )
    }
    fn to_json(&self) -> JsonValue {
        object! {
            "id" => self.id,
            "description" => self.description.clone(),
            "status" => format!("{:?}", self.status),
            "created_at" => self.created_at.to_rfc3339(),
            "updated_at" => self.updated_at.to_rfc3339(),
        }
    }

    fn from_json(json: &JsonValue) -> Option<Self> {
        Some(Task {
            id: json["id"].as_u32()?,
            description: json["description"].as_str()?.to_string(),
            status: match json["status"].as_str()? {
                "ToDo" => TaskStatus::ToDo,
                "InProgress" => TaskStatus::InProgress,
                "Done" => TaskStatus::Done,
                _ => return None,
            },
            created_at: DateTime::parse_from_rfc3339(json["created_at"].as_str()?)
                .ok()?
                .with_timezone(&Local),
            updated_at: DateTime::parse_from_rfc3339(json["updated_at"].as_str()?)
                .ok()?
                .with_timezone(&Local),
        })
    }
}
struct ID {
    current_id: u32,
}
impl ID {
    fn new(start_value: Option<u32>) -> Self {
        match start_value {
            Some(current_id) => ID {
                current_id: current_id,
            },
            None => ID { current_id: 0 },
        }
    }
    fn generate(&mut self) -> u32 {
        (*self).current_id += 1;
        self.current_id
    }
}

struct FileStorage {
    path: String,
}

impl FileStorage {
    fn new(path: Option<String>) -> Self {
        match path {
            Some(_path) => FileStorage { path: _path },
            None => FileStorage {
                path: format!("temp/{}", Local::now().timestamp_millis().to_string()),
            },
        }
    }

    fn save(&self, data: String) {
        match fs::exists(self.path.clone()) {
            Ok(exist) => {
                if !exist {
                    let path = Path::new(&self.path);
                    match fs::create_dir_all(path.parent().expect("Enable to get parent dir.")) {
                        Ok(_) => println!("Dir create successfully."),
                        Err(error) => println!("Erreur : {}", error),
                    }
                }
            }
            Err(error) => println!("Erreur : {}", error),
        }

        match fs::write(self.path.clone(), data) {
            Ok(_) => println!("Saved successfully."),
            Err(error) => println!("Erreur : {}", error),
        }
    }

    fn get(&self) -> Option<String> {
        match fs::read_to_string(self.path.clone()) {
            Ok(content) => Some(content),
            Err(error) => {
                println!("Erreur : {}", error);
                None
            }
        }
    }
}
struct TaskManager {
    tasks: Vec<Task>,
    id_generator: ID,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            id_generator: ID::new(None),
        }
    }

    fn add_task(&mut self, desc: String) {
        let task = Task::new(self.id_generator.generate(), desc);
        self.tasks.push(task);
    }

    fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|&task| task.id == id)
    }

    fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }

    fn to_json_string(&self) -> (String, u32) {
        let json_array = self.tasks.iter().map(|t| t.to_json()).collect::<Vec<_>>();
        (
            json::JsonValue::Array(json_array).to_string(),
            self.id_generator.current_id,
        )
    }

    fn from_json_string(json_str: &str, current_id: u32) -> Option<Self> {
        let parsed = json::parse(json_str).ok()?;
        let mut tasks = Vec::new();
        for item in parsed.members() {
            if let Some(task) = Task::from_json(item) {
                tasks.push(task);
            }
        }
        Some(TaskManager {
            tasks,
            id_generator: ID::new(Some(current_id)),
        })
    }
}
enum TaskManagerOperation {
    Add,
    Delete,
    MarkInProgress,
    MarkDone,
    UpdateDesc,
    Get,
    List,
}

fn task_manager_factory(path: Option<String>) -> (TaskManager, Box<dyn Fn(&TaskManager)>) {
    let id_store = FileStorage::new(Some("temp/id".to_string()));
    let store = FileStorage::new(path);
    let store_content = store.get().unwrap_or("[]".to_string());
    let task = TaskManager::from_json_string(
        &store_content,
        id_store
            .get()
            .unwrap_or("0".to_string())
            .parse::<u32>()
            .unwrap_or(0),
    )
    .unwrap_or(TaskManager::new());

    let save = move |taskmanger: &TaskManager| {
        let (task_data, id) = taskmanger.to_json_string();
        id_store.save(id.to_string());
        store.save(task_data);
    };

    (task, Box::new(save))
}

fn exec(
    op: TaskManagerOperation,
    path: Option<String>,
    task_id: Option<u32>,
    description: Option<String>,
) {
    let (mut taskmanager, save) = task_manager_factory(path);

    match op {
        TaskManagerOperation::Add => {
            if let Some(desc) = description {
                taskmanager.add_task(desc.clone());
                println!("✓ Task added: \"{}\"", desc);
            } else {
                println!("✗ Error: Description required for adding a task");
            }
        }
        TaskManagerOperation::Delete => {
            if let Some(id) = task_id {
                taskmanager.remove_task(id);
                println!("✓ Task {} deleted", id);
            } else {
                println!("✗ Error: Task ID required for deletion");
            }
        }
        TaskManagerOperation::MarkInProgress => {
            if let Some(id) = task_id {
                if let Some(task) = taskmanager.get_task_mut(id) {
                    task.mark_in_progress();
                    println!("✓ Task {} marked as In Progress", id);
                } else {
                    println!("✗ Error: Task {} not found", id);
                }
            } else {
                println!("✗ Error: Task ID required");
            }
        }
        TaskManagerOperation::MarkDone => {
            if let Some(id) = task_id {
                if let Some(task) = taskmanager.get_task_mut(id) {
                    task.mark_done();
                    println!("✓ Task {} marked as Done", id);
                } else {
                    println!("✗ Error: Task {} not found", id);
                }
            } else {
                println!("✗ Error: Task ID required");
            }
        }
        TaskManagerOperation::UpdateDesc => {
            if let Some(id) = task_id {
                if let Some(desc) = description {
                    if let Some(task) = taskmanager.get_task_mut(id) {
                        task.update(desc.clone());
                        println!("✓ Task {} updated: \"{}\"", id, desc);
                    } else {
                        println!("✗ Error: Task {} not found", id);
                    }
                } else {
                    println!("✗ Error: Description required for update");
                }
            } else {
                println!("✗ Error: Task ID required");
            }
        }
        TaskManagerOperation::Get => {
            if let Some(id) = task_id {
                if let Some(task) = taskmanager.get_task(id) {
                    task.print();
                } else {
                    println!("✗ Error: Task {} not found", id);
                }
            } else {
                println!("✗ Error: Task ID required");
            }
        }
        TaskManagerOperation::List => {
            if taskmanager.tasks.is_empty() {
                println!("No tasks found.");
            } else {
                println!("Tasks:");
                for task in &taskmanager.tasks {
                    task.print();
                }
            }
        }
    }

    save(&taskmanager);
}
// fn interact() {
//     println!("===== WELCOME TO TASK TRACKER INTERACT MODE ====");
//     print_help(true);
//     loop {
//         let mut args: String = String::new();
//         println!("Enter the command: ");
//         std::io::stdin().read_line(&mut args).expect("Log");
//         let args: Vec<String> = args
//             .trim()
//             .split_whitespace()
//             .map(|arg| arg.to_string())
//             .collect();
//         println!("Provided Arguments: {:?}", args);
//     }
// }

fn main() {
    let path = std::env::var("TASKS_STORE_PATH").unwrap_or("temp/tasks".to_string());
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help(false);
        return;
    }

    let command = &args[1];
    let path = Some(path);

    match command.as_str() {
        "add" => {
            let description = args.get(2).map(|s| s.clone());
            exec(TaskManagerOperation::Add, path, None, description);
        }
        "remove" => {
            if let Some(id_str) = args.get(2) {
                let task_id = id_str.parse::<u32>().ok();
                exec(TaskManagerOperation::Delete, path, task_id, None);
            }
        }
        "mark-in-progress" => {
            if let Some(id_str) = args.get(2) {
                let task_id = id_str.parse::<u32>().ok();
                exec(TaskManagerOperation::MarkInProgress, path, task_id, None);
            }
        }
        "done" => {
            if let Some(id_str) = args.get(2) {
                let task_id = id_str.parse::<u32>().ok();
                exec(TaskManagerOperation::MarkDone, path, task_id, None);
            }
        }
        "edit" => {
            if let Some(id_str) = args.get(2) {
                let task_id = id_str.parse::<u32>().ok();
                let description = args.get(3).map(|s| s.clone());
                exec(TaskManagerOperation::UpdateDesc, path, task_id, description);
            }
        }
        "get" => {
            if let Some(id_str) = args.get(2) {
                let task_id = id_str.parse::<u32>().ok();
                exec(TaskManagerOperation::Get, path, task_id, None);
            }
        }
        "list" => {
            exec(TaskManagerOperation::List, path, None, None);
        }
        "help" => print_help(false),
        // "interact" => interact(),
        _ => println!(
            "Unknown command: {}. Use 'help' for usage instructions.",
            command
        ),
    }
}

fn print_help(is_interact: bool) {
    println!(
        "Usage: {} <command> [options]",
        if is_interact { "task-tracker-cli" } else { "" }
    );
    println!("Commands:");
    println!("  add <description>           - Add a new task");
    println!("  remove <id>                 - remove a task");
    println!("  mark-in-progress <id>       - Mark task as in progress");
    println!("  done <id>                   - Mark task as done");
    println!("  edit <id> <description>     - Edit task description");
    println!("  get <id>                    - Get task details");
    println!("  list                        - List all task");
    println!("  help                        - Show this help message");
    if !is_interact {
      //  println!("  interact                    - Start interact mode");
    }
}
