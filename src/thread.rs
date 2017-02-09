use std::thread;
// use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let aa = Mutex::new(0);
    //let aa = 0;
    let b = Arc::new(aa);
    let mut vec = Vec::new();
    for _ in 0..1000 {
        let e = b.clone();
        let handle = thread::spawn(move || {
            let g = e.as_ref();
            // let h = g.get_mut();
            // *h += 1;
            let mut data = g.lock().unwrap();
            *data += 1;
        });
        vec.push(handle);
//        let _ = handle.join();
    }

    for pat in vec {
        pat.join();
    }

    let c = b.as_ref();
    println!("After a = {:?}", c);


}