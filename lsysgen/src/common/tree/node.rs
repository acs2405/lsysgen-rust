use std::vec::Vec;

use crate::common::{Value, Parameter};

#[derive(Debug, Clone)]
pub enum Node<Ctx=context::Instance, Char=char> {
  BranchStart(usize),
  BranchEnd(usize),
  Leaf(NodeContent<Ctx, Char>),
}

#[derive(Debug, Clone)]
pub struct NodeContent<Ctx=context::Instance, Char=char> {
  pub character: Char,
  pub context: Ctx,
}

pub mod context {
  use crate::common::{Value, Parameter};

  pub trait Context {
    fn is_left_side() -> bool {false}
    fn is_right_side() -> bool {false}
    fn is_instance() -> bool {false}
  }

  /// Context for nodes in the left side of a rule
  #[derive(Debug, Clone)]
  pub struct LeftSide {
    pub params: Vec<Parameter>,
  }

  /// Context for nodes in the right side of a rule
  #[derive(Debug, Clone)]
  pub struct RightSide {
    pub args: Vec<Expr>,
  }

  /// Context for instance nodes
  #[derive(Debug, Clone)]
  pub struct Instance {
    pub values: Vec<Value>,
  }

  impl Context for LeftSide {
    fn is_left_side() -> bool {true}
  }

  impl Context for RightSide {
    fn is_right_side() -> bool {true}
  }

  impl Context for Instance {
    fn is_instance() -> bool {true}
  }
}

impl<Char> NodeContent<context::LeftSide, Char> {
  /// Creates a new left side node content.
  pub fn new_left(ch: Char) -> Self {
    NodeContent {
      character: ch,
      context: context::LeftSide{params: Vec::new()},
    }
  }
}

impl<Char> NodeContent<context::RightSide, Char> {
  /// Creates a new right side node content.
  pub fn new_right(ch: Char) -> Self {
    NodeContent {
      character: ch,
      context: context::RightSide{args: Vec::new()},
    }
  }
}

impl<Char> NodeContent<context::Instance, Char> {
  /// Creates a new instance node content.
  pub fn new_instance(ch: Char) -> Self {
    NodeContent {
      character: ch,
      context: context::Instance{values: Vec::new()},
    }
  }
}
