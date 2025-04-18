use std::vec::Vec;
use std::iter::{Iterator, DoubleEndedIterator};

use super::node::*;

#[derive(Debug, Clone)]
pub struct Tree<Ctx=context::Instance, Char=char> {
  nodes: Vec<Node<Ctx, Char>>,
  open_branches: Vec<usize>,
}

pub struct TreeIterator<Ctx, Char=char> {
  tree: &Tree<Ctx, Char>,
  idx: usize,
}

pub struct TreeBranchIterator<Ctx, Char=char> {
  tree: &Tree<Ctx, Char>,
  idx: usize,
  depth: i32,
}

impl<Ctx, Char> Tree<Ctx, Char> {
  pub fn new() -> Self {
    Tree{
      nodes: Vec::new(),
      open_branches: Vec::new(),
    }
  }

  pub fn node_at(&self, i: usize) -> &Node<Ctx, Char> {&self.nodes[i]}

  pub fn add_leaf(&mut self, content: NodeContent<Ctx, Char>) {self.nodes.push(Node::Leaf(content));}

  pub fn open_branch(&mut self) {
    let i = self.nodes.len();
    self.nodes.push(Node::BranchStart(0));
    self.open_branches.push(i);
  }

  pub fn close_branch(&mut self) {
    match self.open_branches.pop() {
      Some(last_open_branch) => {
        let i = self.nodes.len();
        self.nodes.push(Node::BranchEnd(last_open_branch));
        self.nodes[last_open_branch] = Node::BranchStart(i);
      },
    };
  }

  pub fn is_left_side()  -> bool {Ctx::is_left_side() }
  pub fn is_right_side() -> bool {Ctx::is_right_side()}
  pub fn is_instance()   -> bool {Ctx::is_instance()  }

  pub fn iter(&self) -> TreeIterator<Ctx, Char> {
    TreeIterator{
      tree: self,
      idx: 0 //from.unwrap_or(0)
    }
  }
  pub fn branch_iter(&self, from: usize) -> TreeBranchIterator<Ctx, Char> {
    TreeBranchIterator{
      tree: self,
      idx: from, //from.unwrap_or(0)
      depth: 0
    }
  }
}

impl<Ctx, Char> Iterator for TreeIterator<Ctx, Char> {
  type Item = &Node<Ctx, Char>;

  fn next(&mut self) -> Option<Self::Item> {
    self.idx += 1;
    if self.idx < self.tree.nodes.len() {
      Some(self.tree.node_at(self.idx))
    } else {
      None
    }
  }
}

impl<Ctx, Char> Iterator for TreeBranchIterator<Ctx, Char> {
  type Item = &Node<Ctx, Char>;

  fn next(&mut self) -> Option<Self::Item> {
    // Si no existe siguiente nodo, ya hemos terminado
    if self.idx + 1 >= self.tree.nodes.len() {
      return None;
    }

    // Si hemos salido de la rama que habíamos comenzado recorriendo, ya hemos terminado
    if self.depth < 0 {
      return None;
    }

    self.idx += 1;
    match self.tree.node_at(self.idx) {
      Node::BranchStart(i) => {
        self.idx = i; // Ir al final de la rama (para el siguiente)
        self.depth += 1;
        Some(Node::BranchStart(i))
      },
      Node::BranchEnd(i) => {
        self.depth -= 1;
        Some(Node::BranchEnd(i))
      },
      x => Some(x)
    }
  }
}

impl<Ctx, Char> DoubleEndedIterator for TreeBranchIterator<Ctx, Char> {
  fn next_back(&mut self) -> Option<Self::Item> {
    // Si estamos en el primer nodo, no existe anterior nodo, por lo que ya hemos terminado
    if self.idx == 0 {
      return None;
    }

    self.idx -= 1;
    match self.tree.node_at(self.idx) {
      Node::BranchStart(i) => {
        self.depth += 1;
        Some(Node::BranchStart(i))
      },
      Node::BranchEnd(i) => {
        self.idx = i; // Ir al principio de la rama (para el siguiente)
        self.depth -= 1;
        Some(Node::BranchEnd(i))
      },
      x => Some(x)
    }
  }
}
