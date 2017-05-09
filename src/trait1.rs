// http://stackoverflow.com/questions/43493129/how-does-asref-work-with-str/43493257#43493257
// good edit and answer.

use std::path::Path;

trait Trait1 {}

impl Trait1 for str {}
// impl<'a> Trait1 for &'a str {}
// impl<'a, T> Trait1 for &'a T where T: ?Sized + Trait1 {}

fn run<T: ?Sized + Trait1>(_: &T) {}
fn run1<T: AsRef<Path>>(_: T) {}

fn main() {
    // E0277: the trait bound `&str: Trait1` is not satisfied
    run::<str>("sf");
    run1::<&str>("sf");
}