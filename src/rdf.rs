//! # RDF-Related Abstractions
//!
//! This module contains types for representing [RDF](https://www.w3.org/TR/rdf11-concepts/) data.

use oxilangtag::LanguageTag as OxiLanguageTag;
use oxiri::Iri as OxiIri;
use std::fmt;

pub type Iri = OxiIri<String>;
pub type LangTag = OxiLanguageTag<String>;
pub type BlankId = String;

/// RDF identifiers can either be Internationalized Resource Identifiers (IRIs) or blank node
/// identifiers. The latter are random identifiers which should be unique to the graph they are
/// contained in. They should in no circumstance be exposed towards the result set, they should be
/// reconstructed when formatting.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Id {
    Named(Iri),
    Blank(BlankId),
}

// There's a custom debug implementation to hide the enum variant tag when printing,
// it saves some screen space that's not needed.
impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::Named(iri) => write!(f, "\"{}\"", iri.as_str()),
            Id::Blank(id) => write!(f, "\"{}\"", id),
        }
    }
}

/// RDF Terms are either identifiers or literals.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Term {
    Id(Id),
    Literal(Literal),
}

// There's a custom debug implementation to hide the enum variant tag when printing,
// it saves some screen space that's not needed.
impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Id(id) => id.fmt(f),
            Term::Literal(lit) => lit.fmt(f),
        }
    }
}

/// RDF Literals always have a lexical 'form' as per
/// [RDF 1.1 Concepts And Abstract Syntax](https://www.w3.org/TR/rdf11-concepts/#dfn-literal).
/// They can optionally contain a datatype describing how the literal form maps to a literal value
/// (The default type is: [xs:string](http://www.w3.org/2001/XMLSchema#string), but we do not store
/// this).
///
/// If the datatype is [rdf:langString](http://www.w3.org/1999/02/22-rdf-syntax-ns#langString),
/// we can optionally supply a language tag ([BCP47](https://tools.ietf.org/html/bcp47)) such as
/// `"nl"` or `"fr"`.
///
/// # Examples
/// ```
/// // string
/// let literal = Literal::new(String::from("hello"));
/// assert_eq!("hello", format!("{:?}", literal));
/// ```
/// ```
/// // typed literal
/// let type_iri = String::from("http://www.w3.org/2001/XMLSchema#integer")
/// let type_iri = Iri::parse(type_iri).expect("Parsing failed");
/// let typed_literal = Literal::new_typed(String::from("42"), type_iri);
/// assert_eq!("\"42\"^^http://www.w3.org/2001/XMLSchema#integer", format!("{:?}", typed_literal));
/// ```
/// ```
/// // language tagged string
/// let lang_tag = LangTag::parse(String::from("nl")).expect("Parsing failed");
/// let lang_string = Literal::new_lang(String::from("hallo wereld"), lang_tag);
/// assert_eq!("\"hallo wereld\"@nl", format!("{:?}", typed_literal));
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Literal {
    form: String,
    datatype: Option<Iri>,
    lang: Option<LangTag>,
}

// There's a custom debug implementation to hide structure tags when printing,
// it saves some screen space that's not needed.
impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(lang) = &self.lang {
            write!(f, "\"{}\"@{}", self.form, lang)
        } else if let Some(dtype) = &self.datatype {
            write!(f, "\"{}\"^^{}", self.form, dtype)
        } else {
            write!(f, "\"{}\"", self.form)
        }
    }
}

impl Literal {
    /// Create a new literal with type [xs:string](http://www.w3.org/2001/XMLSchema#string) (which
    /// we do not store since it is the default type).
    pub fn new(form: String) -> Self {
        Literal {
            form,
            datatype: None,
            lang: None,
        }
    }

    /// Create a new literal with a given form and datatype.
    pub fn new_typed(form: String, datatype: Iri) -> Self {
        Literal {
            form,
            datatype: Some(datatype),
            lang: None,
        }
    }

    /// Create a new literal with a given form and langauge. Automatically sets the type to
    /// [xs:langString](http://www.w3.org/2001/XMLSchema#langString)
    pub fn new_lang(form: String, lang: LangTag) -> Self {
        let iri_string = String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString");
        let datatype = Iri::parse(iri_string).unwrap();
        Literal {
            form,
            datatype: Some(datatype),
            lang: Some(lang),
        }
    }
}
