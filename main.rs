use std::io;

fn parse_cmd(line :String) -> (String, String) {
    let l1 = line.trim();
    match l1.find(" "){
        Some(i) => return(l1[0..i].to_string(), l1[i+1..].to_string()),
        None    => return(l1.to_string(), "".to_string()),
    }
}

fn main() {
    let len                 = 150;
    let mut v: Vec<u32>     = vec![1; len];
    let mut mode: u8        = 0;

    println!("Welcome!");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("read error");
        let (ins, par) = parse_cmd(line);
        match ins.as_ref() {
            "h"     => println!("Help!"),
            "" | "q"  => break,
            "n"     => {
                    mode = par.parse().expect("oh");
                    v = vec![1; len];
                }
            "t"     => {
                let id :usize = par.parse().expect("idn");
                for i in 0..len {
                    if i % id != 0{
                        continue;
                    }
                    if mode==0 {
                        v[i] = 1 - v[i];
                    } else {
                        v[i] = 0;
                    }
                }
            }
            "p"     => {
                let mut c = 0;
                for i in 0..len {
                    c += v[i];
                }
                println!("Count:{}",c);
            }
            w       => println!("Unknown cmd: {}, help.", w),
        }
    }
}
