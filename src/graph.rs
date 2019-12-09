use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::{Rc, Weak};

pub type Face = Vec<usize>;

pub type RcGraphEdge = Rc<RefCell<GraphEdge>>;
pub type WeakGraphEdge = Weak<RefCell<GraphEdge>>;

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct EdgeList(BTreeSet<Edge>);

#[derive(Debug)]
pub struct GraphEdge {
    pub from: usize,
    pub to: usize,
    pub oppo: WeakGraphEdge,
    pub next: WeakGraphEdge,
    pub visited: bool,
}

#[derive(Debug)]
pub struct Graph(Vec<Vec<RcGraphEdge>>);

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
        Self { from, to }
    }
}

impl GraphEdge {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            oppo: Weak::new(),
            next: Weak::new(),
            visited: false,
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

impl Graph {
    pub fn new(capacity: usize) -> Self {
        Self(vec![Vec::new(); capacity])
    }

    pub fn add_pair(&mut self, from: usize, to: usize) {
        let e1 = Rc::new(RefCell::new(GraphEdge::new(from, to)));
        let e2 = Rc::new(RefCell::new(GraphEdge::new(to, from)));
        e1.borrow_mut().oppo = Rc::downgrade(&e2);
        e2.borrow_mut().oppo = Rc::downgrade(&e1);
        self.0[from].push(e1);
        self.0[to].push(e2);
    }

    pub fn neighbors(&self, index: usize) -> impl Iterator<Item = &RcGraphEdge> {
        self.0[index].iter()
    }
}
