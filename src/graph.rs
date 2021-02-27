use crate::rdf::{Id, Iri, Term};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Representation of 'tuples' or 'pairs', if we see the predicates as relationship names, this is
/// merely a representational choice
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
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

pub type GroundSet = BTreeSet<Pair>;

pub struct AtomSet {
    stable: GroundSet,
    delta: GroundSet,
    backlog: GroundSet,
}

impl AtomSet {
    pub fn new() -> Self {
        AtomSet {
            stable: GroundSet::new(),
            delta: GroundSet::new(),
            backlog: GroundSet::new(),
        }
    }

    pub fn insert(&mut self, pair: Pair) -> bool {
        self.stable.insert(pair)
    }

    pub fn changed(&mut self) -> bool {
        self.stable.append(&mut self.delta);
        self.delta = self.backlog.difference(&self.stable).cloned().collect();
        self.backlog = GroundSet::new();
        !self.delta.is_empty()
    }
}

pub struct Graph {
    elements: BTreeMap<Iri, AtomSet>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            elements: BTreeMap::new(),
        }
    }

    pub fn get_mut(&mut self, predicate: &Iri) -> Option<&mut AtomSet> {
        self.elements.get_mut(predicate)
    }

    pub fn insert(&mut self, predicate: Iri, pairset: AtomSet) -> Option<AtomSet> {
        self.elements.insert(predicate, pairset)
    }
}

impl<'a> IntoIterator for &'a Graph {
    type Item = (&'a Iri, &'a AtomSet);
    type IntoIter = std::collections::btree_map::Iter<'a, Iri, AtomSet>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}
