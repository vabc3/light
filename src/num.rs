
fn main() {
    let parts: Vec<&str> = ["2j1"].to_vec();
    let a:u16= try!(parts[0]
        .parse()
        .map_err(|_| 25u16));
        // .unwrap();
    println!("{}", a);
}