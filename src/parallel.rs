use std::sync::{Mutex,Arc};
use std::rc::Rc;

struct S {}

fn main(){
    let t = S{};
    let t = Rc::new(t);
    let a: Mutex<Rc<S>> = Mutex::new(t);
    a.lock();
}