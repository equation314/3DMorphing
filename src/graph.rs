use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::{Rc, Weak};

pub type Face = Vec<usize>;

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    oppo_edge: Option<Weak<RefCell<Edge>>>,
}

#[derive(Debug, Clone)]
pub struct EdgeList(BTreeSet<Edge>);

#[derive(Debug)]
pub struct GraphEdgeList(Vec<Vec<Rc<RefCell<Edge>>>>);

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.from, self.to).cmp(&(other.from, other.to))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Eq for Edge {}

impl std::ops::Deref for EdgeList {
    type Target = BTreeSet<Edge>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EdgeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            oppo_edge: None,
        }
    }
}

impl EdgeList {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn add(&mut self, from: usize, to: usize) -> bool {
        if from < to {
            self.0.insert(Edge::new(from, to))
        } else if from > to {
            self.0.insert(Edge::new(to, from))
        } else {
            false
        }
    }
}

impl GraphEdgeList {
    pub fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn add_pair(&mut self, from: usize, to: usize) {
        let e1 = Rc::new(RefCell::new(Edge::new(from, to)));
        let e2 = Rc::new(RefCell::new(Edge::new(to, from)));
        e1.borrow_mut().oppo_edge = Some(Rc::downgrade(&e2));
        e2.borrow_mut().oppo_edge = Some(Rc::downgrade(&e1));
        self.0[from].push(e1);
        self.0[to].push(e2);
    }
}
