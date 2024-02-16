use exolvl::{Exolvl, Read as _, Write as _};
use std::io::BufWriter;

fn main() {
    let mut input = std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap();

    let level = Exolvl::read(&mut input).unwrap();

    println!("{:#?}", level);

    let mut file = BufWriter::new(std::fs::File::create(std::env::args().nth(2).unwrap()).unwrap());

    Exolvl::write(&level, &mut file).unwrap();
}
