#[derive(Debug)]
enum Body<'a> {
    BufBody(&'a [u8], usize)
}
#[derive(Debug)]
struct Client {
}

impl Client {
    fn new() -> Client {
        Client{}
    }
    // fn get(self: &Client) -> Build {
    fn get(&self) -> Build {
        Build { client: self, body: None }
    }
}

struct Build<'a> {
    client: &'a Client,
    body: Option<Body<'a>>,
}

impl<'a> Build<'a> {
    fn body(mut self, b: Body<'a>) -> Build<'a> {
        self.body = Some(b);
        // Build {body:None}
        self
    }

    fn send(self) -> Result<i32,i32> {
        println!("{:?}", self.client);
        println!("{:?}", self.body);
        Ok(5)
    }
}

fn run () -> Result<i32,i32> {
    let a = vec![1];
    let b = Body::BufBody(&a,1);
    let client = Client::new();
    let mut builder = client.get();
    builder = builder.body(b);
    builder.send()
}

fn main() {
    let x = run().unwrap();
    println!("{:?}",x);
}

