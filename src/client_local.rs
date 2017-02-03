use std::io::Read;
use std::fs::File;

enum Body<'a> {
    SizedBody(&'a mut (Read + 'a)),
    BufBody,
}
struct Client {}
struct Build<'a> {
    client: &'a Client,
}
impl<'a> Build<'a> {
    fn body(self, _: Body<'a>) {}
}

fn main() {
    // let mut f = File::open("foo.txt").unwrap();
    // let b = Body::SizedBody(&mut f);
    let client = Client {};
    let b = Body::BufBody;
    let builder = Build { client: &client };
    builder.body(b);
}

