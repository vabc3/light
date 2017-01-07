use std::fmt;

struct ChessBoard {
    size: u8,
    places: Vec<u8>,
}

impl ChessBoard {
    fn new(size: u8) -> ChessBoard {
        ChessBoard { size: size, places: vec![0;size as usize] }
    }

    fn valid(&self, current: usize) -> bool {
        let mut br = false;
        for i in 0..current+1 {
            for j in i+1..current+1 {
                let abs = match self.places[i].checked_sub(self.places[j]) {
                        Some(val) => {val},
                        _ => { self.places[j] - self.places[i] }
                    };
                //println!("i={},j={},abs={}",i,j,abs);
                if abs ==0 || (j-i) as u8 == abs {
                    // println!("i={},j={}",i,j);
                    br = true;break;
                }
            }
            if br { break; }
        }
        !br
    }
}

impl std::fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ">-\n").unwrap();
        for y in 0..self.size {
            // write!(f,"{}", self.places[y as usize]); continue;
            for x in 0..self.size {
                if self.places[y as usize] == x {
                    write!(f, "Q").unwrap();
                } else {
                    write!(f, "-").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "-<")
    }
}


fn main() {
    let mut board = ChessBoard::new(4+4);
    {
        let mut current: usize = 0;
        let mut new = true;
        loop {
            match current as u8{
                x if x >= board.size => {
                    println!("found!");
                    println!("{}", board);
                    current -= 1;
                    new = false;
                }
                _ => {
                    if new {
                        board.places[current] = 0;
                        new = false;
                    } else {
                        board.places[current] += 1;
                    }

                    if board.places[current] >= board.size {
                        if current == 0 { break; }
                        current -= 1;
                    } else if board.valid(current) {
                        current += 1;
                        new = true;
                    }
                }
            }
        }
    }
}
