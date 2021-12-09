use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Eq)]
pub struct Node {
    pub state: Vec<u8>,
    pub g: u32,
    pub h: u32,
    pub parent: Option<Rc<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

impl Node {
    pub fn new(state: Vec<u8>, g: u32) -> Node {
        Node {
            state,
            g,
            h: 0,
            parent: None,
        }
    }

    pub fn possible_moves(&self, size: usize) -> Vec<Node> {
        let index = self.state.iter().position(|&v| v == 0).unwrap();
        let (y, x) = (index / size, index % size);
        let g = self.g + 1;

        let mut moves = Vec::with_capacity(4);
        if x > 0 {
            let mut state = self.state.clone();
            state.swap(index, index - 1);
            moves.push(Node::new(state, g));
        }
        if x < size - 1 {
            let mut state = self.state.clone();
            state.swap(index, index + 1);
            moves.push(Node::new(state, g));
        }
        if y > 0 {
            let mut state = self.state.clone();
            state.swap(index, index - size);
            moves.push(Node::new(state, g));
        }
        if y < size - 1 {
            let mut state = self.state.clone();
            state.swap(index, index + size);
            moves.push(Node::new(state, g));
        }
        moves
    }
}
