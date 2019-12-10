use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::{Rc, Weak};

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
pub struct Graph {
    edges: Vec<Vec<RcGraphEdge>>,
    index_map: Vec<usize>,
    nr_nodes: usize,
    unique_edges: EdgeList,
}

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
    pub fn new<T: Ord>(nodes: &Vec<T>) -> Self {
        let n = nodes.len();
        let mut index_map = Vec::with_capacity(n);
        let mut map = BTreeMap::<&T, usize>::new();

        // unique vertices
        for i in 0..n {
            let v = &nodes[i];
            let id = if let Some(&id) = map.get(&v) { id } else { i };
            map.insert(v, id);
            index_map.push(id);
        }

        Self {
            edges: vec![Vec::new(); n],
            index_map,
            nr_nodes: n,
            unique_edges: EdgeList::new(),
        }
    }

    pub fn add_pair(&mut self, from: usize, to: usize) {
        let (from, to) = (self.index_map[from], self.index_map[to]);
        if !self.unique_edges.add(from, to) {
            return;
        }
        let e1 = Rc::new(RefCell::new(GraphEdge::new(from, to)));
        let e2 = Rc::new(RefCell::new(GraphEdge::new(to, from)));
        e1.borrow_mut().oppo = Rc::downgrade(&e2);
        e2.borrow_mut().oppo = Rc::downgrade(&e1);
        self.edges[from].push(e1);
        self.edges[to].push(e2);
    }

    pub fn neighbors_count(&self, index: usize) -> usize {
        self.edges[index].len()
    }

    pub fn neighbors(&self, index: usize) -> impl Iterator<Item = &RcGraphEdge> {
        self.edges[index].iter()
    }
}
