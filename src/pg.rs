// playground
#![feature(unique)]
use std::ptr::Unique;
trait T1 {}
trait T2 {}
// trait T6{}
// impl<T: T2> T1 for T {}
// impl<T: T6> T2 for T {}

// impl T6 for S1 {}
// impl T2 for S1 {}
// struct S1 {}

use std::io::Write;
fn run(x: &Write) {}
fn run2(x: &Write) {}

// impl<T: Write> T1 for T {}

struct Q<T> {
    a: Unique<T>,
}

impl<T> Q<T> {
    pub fn new() -> Q<T> {
        unsafe {
            // !0 is usize::MAX. This branch should be stripped at compile time.

            // heap::EMPTY doubles as "unallocated" and "zero-sized allocation"
            Q {
                a: Unique::new(3 as *mut T),
            }
        }
    }
}

fn main() {
    let a = Vec::new();
    // a=7;
    // run(&a);
     run2(&a);
   // let b = Q::new();

}