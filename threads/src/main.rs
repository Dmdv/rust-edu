mod scheduler;

use scheduler::{Task, TaskScheduler};
use std::time::Duration;

fn main() {
    let scheduler = TaskScheduler::new();
    
    // Add some example tasks
    scheduler.add_task(Task::new(
        1,
        "Task 1".to_string(),
        1,
        || println!("Running Task 1"),
        Duration::from_secs(2)
    ));

    scheduler.add_task(Task::new(
        2,
        "Task 2".to_string(),
        2,
        || println!("Running Task 2"),
        Duration::from_secs(3)
    ));

    scheduler.add_task(Task::new(
        3,
        "Task 3".to_string(),
        3,
        || println!("Running Task 3"),
        Duration::from_secs(1)
    ));

    println!("Scheduler started. Press Ctrl+C to stop.");
    scheduler.start();
}
