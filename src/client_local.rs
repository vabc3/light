use std::io::Read;
use std::fs::File;

enum Body<'a> {
    // SizedBody(&'a mut (Read + 'a)),
    BufBody(&'a mut i32),
}
struct Client {}
struct Build<'a> {
    client: &'a Client,
    body: Option<Body<'a>>,
}
impl<'a> Build<'a> {
    fn body(&mut self, a1: Body<'a>) {
        self.body = Some(a1);
    }
}

fn main() {
    // let mut f = File::open("foo.txt").unwrap();
    // let b = Body::SizedBody(&mut f);
    let mut x = 7i32;
    let b = Body::BufBody(&mut x);
    let client = Client {};
    let mut builder = Build {
        client: &client,
        body: None,
    };
    {
        builder.body(b);
    }
}
