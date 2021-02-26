use oxilangtag::LanguageTag as OxiLanguageTag;
use oxiri::Iri as OxiIri;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub type Iri = OxiIri<String>;
pub type LangTag = OxiLanguageTag<String>;
pub type BlankId = String;
pub type Predicate = Iri;
pub type Graph = BTreeMap<Predicate, BTreeSet<Pair>>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Id {
    Named(Iri),
    Blank(BlankId),
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::Named(iri) => write!(f, "\"{}\"", iri.as_str()),
            Id::Blank(id) => write!(f, "\"{}\"", id),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Term {
    Id(Id),
    Literal(Literal),
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Id(id) => id.fmt(f),
            Term::Literal(lit) => lit.fmt(f),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Literal {
    form: String,
    datatype: Option<Iri>,
    lang: Option<LangTag>,
}

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
    pub fn new_untyped(form: String) -> Self {
        Literal {
            form,
            datatype: None,
            lang: None,
        }
    }

    pub fn new_typed(form: String, datatype: Iri) -> Self {
        Literal {
            form,
            datatype: Some(datatype),
            lang: None,
        }
    }

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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Pair {
    subject: Id,
    object: Term,
}

impl Pair {
    pub fn new(subject: Id, object: Term) -> Self {
        Pair { subject, object }
    }
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.subject, self.object)
    }
}
