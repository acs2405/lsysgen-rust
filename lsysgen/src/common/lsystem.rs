use std::string::String;
use std::vec::Vec;
use std::collections::HashMap;

use crate::common::tree::*;
use super::values::Scope;

#[derive(Debug, Clone)]
pub struct LSystem<T> {
  scope: Scope,

  name: String,
  tables: HashMap<String, Table<T>>,
  default_table: Table<T>,
  coding_rules: Table<T>,

  axiom: Tree<node::context::Instance, T>,
  target_iterations: i32,
  settings_2d: Settings2D,
  ignore_chars: Vec<T>,
  table_func: Function,

  current_iter: usize,
  current_tree: Tree<node::context::Instance, T>,
  encoded_trees: Vec<Tree<node::context::Instance, T>>,

  err: ErrorHandler,
  derivator: Derivator<T>,
}
