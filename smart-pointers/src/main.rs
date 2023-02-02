mod sync;

use std::fmt::Display;
use crate::List::{Cons, Nil};
use std::borrow::{Cow};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct Pair<T> {
    x: T,
    y: T,
}

#[allow(unused)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(unused)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// function which creates a vector of i32 and returns it as slice
// https://stackoverflow.com/questions/32682876/is-there-any-way-to-return-a-reference-to-a-variable-created-in-a-function
// This returns a vector and returns a copy of it - not a reference
fn create_vec() -> Vec<i32> {
    vec![1, 2, 3, 4]
}

fn create_vector_box() -> Box<Vec<i32>> {
    Box::new(vec![1, 2, 3, 4])
}

fn create_vector_cow<'a>() -> Cow<'a, Vec<i32>> {
    Cow::Owned(vec![1, 2, 3, 4])
}

fn main() {
    let b = Box::new(5);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("b = {}", b);
    println!("b = {:?}", list);

    // Cow examples
    let o: Cow<[i32]> = Cow::Owned(vec![1, 2, 3]);
    let b: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
    println!("o = {:?}", o);
    println!("b = {:?}", b);

    // Cow vs Box
    let mut v_box = create_vector_box();
    println!("Data from v_box = {:?}", v_box);

    let mut v_cow = create_vector_cow();
    println!("Data from v_cow = {:?}", v_cow);

    // Modify the vector is possible because the vector is owned and mutable
    v_box[1] = 5;
    println!("Data from v_box (modified) = {:?}", v_box);

    // Modify data in the Cow is also possible because it will make a copy of the data
    let owned = v_cow.to_mut();
    owned[1] = 5;
    println!("Data from v_cow (modified) = {:?}", owned);
    println!("Data from v_cow (modified) = {:?}", v_cow);
}
