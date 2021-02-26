mod io;
mod mem;

use io::parser;
use std::fs::File;

fn main() {
    let file = File::open("data/brussels-subway.ttl").expect("Couldn't open file");
    let graph = parser::parse_file(file);

    for (predicate, set) in &graph {
        println!("{:#?} => {:#?}", predicate, set);
    }
}
