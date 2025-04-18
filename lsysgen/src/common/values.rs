use std::string::String;
use std::vec::Vec;
use std::collections::HashMap;

use super::lsystem::LSystem;

#[derive(Debug, Clone)]
pub struct Parameter {
  name: String,
}

#[derive(Debug)]
pub struct Function {
  params: Vec<Parameter>,
  expr: Expr,
  //ctx: ???,
}

#[derive(Debug, Clone)]
pub enum Value {
  Int(i64),
  Float(f64),
  Bool(bool),
  String(String),
  Function(&Function),
  LSystem(&LSystem),
  Null,
  Error,
}

#[derive(Debug, Clone)]
pub struct Scope {
  parent: Option<&mut Scope>,
  mapping: HashMap<String, Value>,
}

impl Function {
  pub fn call(&self, args: Option<&Vec<Value>>, scope: &Scope, ee: &ExpressionEvaluator) -> Value {
    let mut param_mapping = scope.clone();
    if self.params != None && (args.is_none() && self.params.len() > 0) || (args.is_some() && self.params.len() != args.unwrap().len()) {
      return Value::Error;
    }
    for (param, arg) in (0..args.unwrap().len()).map(|i| (self.params[i], args.unwrap()[i])) {
      param_mapping.set(param.name, arg);
    }
    ee.eval(self.expr, &param_mapping)
  }
}

impl std::fmt::Display for Parameter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl std::fmt::Display for Function {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut sparams = String::new();
    for (i, &param) in self.params.iter().enumerate() {
      if i != 0 {
        sparams += ", ";
      }
      sparams += param.name.as_str();
    }
    write!(f, "({}) -> {}", sparams, self.expr.getText())
  }
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Int(i) => write!(f, "{}", i),
      Self::Float(fl) => write!(f, "{}", fl),
      Self::Bool(b) => write!(f, "{}", b),
      Self::String(s) => write!(f, "\"{}\"", s),
      Self::Function(fun) => write!(f, "{}", fun),
      Self::LSystem(lsystem) => write!(f, "{}", lsystem),
      Self::Null => write!(f, "Null"),
      Self::Error => write!(f, "Error"),
    }
  }
}

impl Scope {
  /// Sets the value of a variable in this scope. If the variable doesn't exist in this scope, it's created.
  pub fn set(&mut self, var: String, val: Value) {
    self.mapping.insert(var, val);
  }

  /// Recursively get variable's value.
  pub fn get (&self, var: String) -> Option<&Value> {
    match self.mapping.get(&var) {
      Some(val) => Some(val),
      None => match self.parent {
        Some(scope) => scope.get(var),
        None => None
      }
    }
  }

  /// Returns whether this scope (not recursively) has that variable in it.
  pub fn has (&self, var: String) -> bool {
    match self.mapping.get(&var) {
      Some(_) => true,
      None => false
    }
  }

  /// Merges env into self (without env's ancestors).
  pub fn merge(&mut self, env: &Scope) {
    self.mapping.extend(env.mapping.clone());
  }
}
