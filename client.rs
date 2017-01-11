// enum Body {
//     A,B
// }

struct Client {
}

impl Client {
    fn new() -> Client {
        Client{}
    }
    fn get(self: &Client) -> Build {
        Build {}
    }
}

struct Build {
}

fn main() {
    let client = Client::new();
    client.get();
}