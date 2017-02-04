trait T1 {}
enum Body<'a> {
    SizedBody(&'a mut (T1 + 'a)),
    BufBody,
}
struct Client {}
struct Builder<'a> {
    client: &'a Client,
}
impl<'a> Builder<'a> {
    fn body(&mut self, _: Body<'a>) {}
}
fn main() {
    let b = Body::BufBody;
    let client = Client {};
    let mut builder = Builder { client: &client };
    builder.body(b);
}
