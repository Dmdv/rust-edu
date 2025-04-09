use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;

// Example 1: Basic Send and Sync types
fn basic_types_example() {
    // These types are Send and Sync
    let x: i32 = 5;
    let y: String = "hello".to_string();
    let z: Vec<i32> = vec![1, 2, 3];

    // We can move these to another thread
    thread::spawn(move || {
        println!("x: {}, y: {}, z: {:?}", x, y, z);
    }).join().unwrap();
}

// Example 2: Non-Send types (Rc)
fn non_send_example() {
    let rc = Rc::new(5);
    // Uncommenting this would cause a compile error:
    // thread::spawn(move || {
    //     println!("{}", rc);
    // });
}

// Example 3: Non-Sync types (RefCell)
fn non_sync_example() {
    let cell = RefCell::new(5);
    // Uncommenting this would cause a compile error:
    // thread::spawn(move || {
    //     println!("{}", *cell.borrow());
    // });
}

// Example 4: Using Arc for shared ownership
fn arc_example() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            println!("Thread {}: {:?}", i, data);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// Example 5: Using Mutex for shared mutable state
fn mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

// Example 6: Using RwLock for multiple readers
fn rwlock_example() {
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // Spawn readers
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let value = data.read().unwrap();
            println!("Reader {}: {}", i, *value);
        }));
    }

    // Spawn a writer
    let data = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        let mut value = data.write().unwrap();
        *value += 1;
        println!("Writer: incremented value to {}", *value);
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}

// Example 7: Custom type with Send and Sync
struct MyThreadSafeType {
    data: Arc<Mutex<i32>>,
}

impl MyThreadSafeType {
    fn new(value: i32) -> Self {
        Self {
            data: Arc::new(Mutex::new(value)),
        }
    }

    fn increment(&self) {
        let mut data = self.data.lock().unwrap();
        *data += 1;
    }

    fn get_value(&self) -> i32 {
        *self.data.lock().unwrap()
    }
}

fn custom_type_example() {
    let shared_data = Arc::new(MyThreadSafeType::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let shared_data = Arc::clone(&shared_data);
        handles.push(thread::spawn(move || {
            shared_data.increment();
            println!("Thread incremented value to {}", shared_data.get_value());
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("=== Basic Types Example ===");
    basic_types_example();

    println!("\n=== Arc Example ===");
    arc_example();

    println!("\n=== Mutex Example ===");
    mutex_example();

    println!("\n=== RwLock Example ===");
    rwlock_example();

    println!("\n=== Custom Type Example ===");
    custom_type_example();

    // Uncomment to see compile errors
    // println!("\n=== Non-Send Example ===");
    // non_send_example();
    
    // println!("\n=== Non-Sync Example ===");
    // non_sync_example();
} 