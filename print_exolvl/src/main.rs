use exolvl::{gzip::extract, types::exolvl::Exolvl, Read};
use std::io::Cursor;
use tracing_forest::ForestLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

fn main() {
    Registry::default().with(ForestLayer::default()).init();

    let filename = std::env::args().nth(1).expect("No filename given");

    let mut bytes = extract(&std::fs::read(filename).unwrap()).unwrap();

    let lvl = Exolvl::read(&mut Cursor::new(&mut bytes)).unwrap();

    println!("{:#?}", lvl);
}
