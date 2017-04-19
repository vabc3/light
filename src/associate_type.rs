// trait A { type B; }
// struct C<T: A> {
//     inner: A::B,
// }

trait C {}

trait AsSlice {
    type B;
}

struct Foo<List: AsSlice> {
    elems: List,
}

struct EI {

}
impl AsSlice for EI {
    type B=  u32;
}



fn main() {
    let a = Foo { elems: EI {} };
}