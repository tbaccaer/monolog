// Each BTreeSet needs an Iterator with an additional
// - fn seek(&mut self, &key)
// method, which positions the iterator at the least upper bound for seekKey
// i.e. the least key >= seekKey
// move to the end if no such key exists.
//

// Leapfrog Join is itself an iterator:
//
// keeps track of references to iterators of all BTreeSets in the join
//
// - track the smallest and largest keys, do seek() on smallest to a LUB of the largest, until
// iterators are positioned at the same key, add it to the result set
//

// use crate::graph::Graph;

// pub fn leapjoin(&
