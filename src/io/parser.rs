use crate::mem::rdf::{Graph, Id, Iri, LangTag, Literal, Pair, Term};
use rio_api::model::Literal::{LanguageTaggedString as LTString, Simple, Typed};
use rio_api::model::NamedOrBlankNode::{BlankNode, NamedNode};
use rio_api::model::Term::{BlankNode as Blank, Literal as Lit, NamedNode as Named};
use rio_api::parser::TriplesParser;
use rio_turtle::{TurtleError, TurtleParser};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;

/// Utility function which can be used for parsing RDF/Turtle files into an in-memory RDF graph
/// representation.
///
/// # Future Work
///
/// * Different parser
/// * Support RDF/XML, TriG, ...
/// * Stream directly to the storage engine
/// * ...
///
/// # Example
/// ```
/// let file = File::open("data/brussels-subway.ttl").expect("Couldn't open file");
/// let graph = parser::parse_file(file);
/// for (predicate, set) in &graph {
///     println!("{:#?} => {:#?}", predicate, set);
/// }
/// ```
pub fn parse_file(file: File) -> Graph {
    let mut parser = TurtleParser::new(BufReader::new(file), None);
    let mut graph = Graph::new();

    parser
        .parse_all(&mut |t| -> Result<(), TurtleError> {
            // TODO: The parsing library `Rio` has an API entirely based around passing string
            // slices into the file buffer, which obviously get cleaned up after the buffer gets
            // invalidated. As a result, this code contains quite a bit of string conversion. An
            // opportunity for cleaner code and optimisation would be to write a parser that goes
            // straight to my in-memory representation of RDF.
            let predicate = Iri::parse(t.predicate.iri.to_string()).unwrap();
            let subject = match t.subject {
                NamedNode(named) => Id::Named(Iri::parse(named.iri.to_string()).unwrap()),
                BlankNode(blank) => Id::Blank(blank.id.to_string()),
            };
            let object = match t.object {
                Named(named) => Term::Id(Id::Named(Iri::parse(named.iri.to_string()).unwrap())),
                Blank(blank) => Term::Id(Id::Blank(blank.id.to_string())),
                Lit(literal) => match literal {
                    Simple { value } => Term::Literal(Literal::new_untyped(value.to_string())),
                    Typed { value, datatype } => {
                        let iri = Iri::parse(datatype.iri.to_string()).unwrap();
                        Term::Literal(Literal::new_typed(value.to_string(), iri))
                    }
                    LTString { value, language } => {
                        let lang = LangTag::parse(language.to_string()).unwrap();
                        Term::Literal(Literal::new_lang(value.to_string(), lang))
                    }
                },
            };
            let pair = Pair::new(subject, object);

            match graph.get_mut(&predicate) {
                Some(set) => {
                    set.insert(pair);
                }
                None => {
                    let mut set = BTreeSet::new();
                    set.insert(pair);
                    graph.insert(predicate, set);
                }
            };

            Ok(())
        })
        .expect("Error while parsing");

    graph
}
