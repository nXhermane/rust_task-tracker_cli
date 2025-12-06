use std::fmt::Debug;

use chrono::{DateTime, Local};

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
    createdAt: DateTime<Local>,
    updatedAt: DateTime<Local>,
}

impl Task {
    fn new(desc: String) -> Self {
        Task {
            id: 1,
            description: desc,
            status: TaskStatus::ToDo,
            createdAt: Local::now(),
            updatedAt: Local::now(),
        }
    }
        fn updated(&mut self) {
        self.updatedAt = Local::now();
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


    
}


fn main() {
    println!("Init Rust task tracker CLI");
    let mut task = Task {
        id: 1,
        description: "Implemented task".to_string(),
        status: TaskStatus::ToDo,
        createdAt: Local::now(),
        updatedAt: Local::now(),
    };
    task.update(String::from("hello"));
    println!("Task ID: {}", task.id);
    println!("Description: {}", task.description);
    println!("Status: {:?}", task.status);
    println!("Created At: {}", task.createdAt);
    println!("Updated At: {}", task.updatedAt);
}
