#![allow(dead_code)]
// enum Body<'a> {
//     SizedBody(&'a mut (T1 + 'a)), //    BufBody,
// }
// struct Client {}
// struct Builder<'a> {
//     client: &'a Client,
//     c2: &'a Client,
// }
// impl<'a> Builder<'a> {
//     fn body(&self, _: Body<'a>) {}
// }
// fn run(){
// let c2 = Client {};
// let c1 = Client {};
// let builder = Builder {
//     client: &c1,
//     c2: &c2,
// };
// builder.body(b);
// let d = Body::SizedBody(&mut ic);
//
trait Trait1 {}
struct S1 {}
struct A1<'a> {
    a: &'a mut Trait1,
}
struct B1<'c> {
    q: Option<A1<'c>>,
}
impl Trait1 for S1 {}
fn b1<'b>(b: &'b mut B1<'b>, e: A1<'b>) {
    // b.q = Some(e);
}
fn b2(_: &B1, _: A1) {}

fn main() {
    let mut b = B1 { q: None };
    let mut a1 = S1 {};
    let d = A1 { a: &mut a1 };
    b1(&mut b, d);
}
