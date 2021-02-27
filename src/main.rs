mod engine;
mod graph;
mod parser;
mod rdf;

use std::fs::File;

fn main() {
    let file = File::open("data/brussels-subway.ttl").expect("Couldn't open file");
    let graph = parser::parse_file(file);

    for (predicate, set) in &graph {
        println!("{:#?} => {:#?}", predicate, set);
    }
}
