/*
The Sized trait is often elided, but when explicitly added it asserts that the type has a size known at compile time.
This can be useful when abstracting over many types that don't necessarily even share a common trait,
such as in an HList
*/

use std::fmt::{Display, Formatter};

pub trait HList: Sized {}

pub struct HNil;
impl HList for HNil {}

pub struct HCons<H,T> {
    pub head: H,
    pub tail: T,
}

impl<H, T: HList> HList for HCons<H, T> {}
impl<H, T> HCons<H, T> {
    pub fn pop(self) -> (H, T) {
        (self.head, self.tail)
    }
}

/*
When to use the ?Sized trait
The ?Sized trait indicates to the compiler that the type does not have a size known at compile-time.
Variants of this trait have apparently been available to the internal compiler,
but now it has been stabilized and made available for general development.
*/

struct Cell<T: ?Sized> {
    pub block: T
}

impl<T: ?Sized + Display> Display for Cell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", &self.block)
    }
}

// associated types

trait Assoc {
    type L: Display;
    type R: Display;
    fn left(&self) -> &Self::L;
    fn right(&self) -> &Self::R;
}

struct Pair(u64,u64);
impl Assoc for Pair {
    type L = u64;
    type R = u64;
    fn left(&self) -> &Self::L { &self.0 }
    fn right(&self) -> &Self::R { &self.1 }
}

fn main() {
    println!("Templates");
    println!("left: {}", Pair(2,3).left());
    let cell = Cell{ block: 1 };
    println!("Cell: {}", &cell);
}
