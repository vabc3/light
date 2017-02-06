// #![feature(custom_derive)]
// #[derive(copy)]
struct A(i32);

// impl Drop for A {
//     fn drop(&mut self) {}
// }

fn main() {
    let mut a = A(1);
    let b = a;
    a.0 = 6;
    println!("{}", b.0);
}