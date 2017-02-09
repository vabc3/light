// playground
#![allow(dead_code)]
#![allow(unused_variables)]
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
            Q { a: Unique::new(3 as *mut T) }
        }
    }
}

fn main1() {
    // let mut a = Vec::new();
    // a.push(3i32);
    // a.push(5u16);
    // a=3;
    // a=7;
    // run(&a);
    //  run2(&a);
    // let b = Q::new();
}

struct A {}
struct Cat : A {}

trait B {
    fn b1(&self);
}
impl A {
    fn b1(&self) {
        println!("A_b1");
    }
}

impl B for A {
    fn b1(&self) {
        println!("B_for_A_b1");
    }
}

fn main() {
    let a = A {};
    a.b1();
    let b: & B = &a;
    b.b1();
}