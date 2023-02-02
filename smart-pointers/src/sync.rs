use std::rc::Rc;

/*
A type is Sync if it is safe to share between threads
"Almost all primitives are Send and Sync, and as a consequence pretty much all types you'll ever interact with are Send and Sync."

Some common exceptions to the above rule are Rc or UnsafeCell. If a type contains a type that is not Sync, then the broader type will also not be safe to share between threads. Non-Sync types can be made Sync by wrapping them with a lock such as RwLock or Mutex.
*/

#[allow(unused)]
struct CanSync {
    a: u64,
    b: String,
}

#[allow(unused)]
struct CantSync {
    a: Rc<u64>,
    b: Rc<String>,
}