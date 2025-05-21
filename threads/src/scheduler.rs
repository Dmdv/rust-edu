use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, Ordering}};
use std::thread;

#[derive(Clone)]
pub struct Task {
    id: u32,
    name: String,
    priority: u8,
    action: Arc<dyn Fn() + Send + Sync + 'static>,
    interval: Duration
}

impl Task {
    pub fn new<F>(id: u32, name: String, priority: u8, action: F, interval: Duration) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Task {
            id,
            name,
            priority,
            action: Arc::new(action),
            interval,
        }
    }

    pub fn run(&self) {
        println!("Running task: {} (ID: {})", self.name, self.id);
        (self.action)();
    }
}

pub struct TaskScheduler {
    pending_tasks: Arc<Mutex<Vec<Task>>>,
    running_tasks: Arc<Mutex<Vec<Task>>>,
    task_available: Arc<Condvar>,
    shutdown: Arc<AtomicBool>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        TaskScheduler {
            pending_tasks: Arc::new(Mutex::new(Vec::new())),
            running_tasks: Arc::new(Mutex::new(Vec::new())),
            task_available: Arc::new(Condvar::new()),
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn add_task(&self, task: Task) {
        if let Ok(mut tasks) = self.pending_tasks.lock() {
            tasks.push(task);
            self.task_available.notify_one();
        }
    }

    pub fn start(&self) {}
    pub fn stop(&self) {}
} 