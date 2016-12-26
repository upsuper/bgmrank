use kuchiki::{NodeDataRef, ElementData};
use kuchiki::iter::{Select, Elements, Descendants};

pub type ElementDataRef = NodeDataRef<ElementData>;

pub trait QuerySelector {
    type Iter: Iterator;
    fn query_selector(&self, selectors: &str) -> Option<<Self::Iter as Iterator>::Item> {
        self.query_selector_all(selectors).next()
    }
    fn query_selector_all(&self, selectors: &str) -> Self::Iter;
}

impl QuerySelector for ElementDataRef {
    type Iter = Select<Elements<Descendants>>;
    fn query_selector_all(&self, selectors: &str) -> Self::Iter {
        self.as_node().select(selectors).unwrap()
    }
}
