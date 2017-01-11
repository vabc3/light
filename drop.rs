struct D {}

impl Drop for D {
    fn drop(&mut self) {
        println!("BOOM times ");
    }
}

fn main(){
    let a = D {};
}