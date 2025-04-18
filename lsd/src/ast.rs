use std::vec::Vec;

/// This module contains normal-mode ASTs.
pub mod normal {
  /// This normal-mode AST is returned by the LSD LsdFile parser.
  pub struct Module<'a> {
    name: Option<&'a str>,
    path: Option<&'a str>,
    stmts: Vec<GlobalStmt<'a>>,
  }

  /// This normal-mode AST is returned by the LSD Expr parser.
  pub enum Expr<'a> {
    Int(i64),
    Float(f64),
    String(&'a str),
    List(Vec<Expr<'a>>),
    Bool(bool),
    Null,
    ID(&'a str),
    PropAcc(Expr<'a>, &'a str),
    FnCall(Expr<'a>, Vec<Expr<'a>>),
    IndexExpr(Expr<'a>, Expr<'a>),
    // Assign(&'a str, Expr<'a>),
    Plus(Expr<'a>),
    Minus(Expr<'a>),
    Not(Expr<'a>),
    BitNot(Expr<'a>),
    Pow(Expr<'a>, Expr<'a>),
    Mul(Expr<'a>, Expr<'a>),
    Div(Expr<'a>, Expr<'a>),
    Mod(Expr<'a>, Expr<'a>),
    Add(Expr<'a>, Expr<'a>),
    Sub(Expr<'a>, Expr<'a>),
    LT(Expr<'a>, Expr<'a>),
    LE(Expr<'a>, Expr<'a>),
    GT(Expr<'a>, Expr<'a>),
    GE(Expr<'a>, Expr<'a>),
    EQ(Expr<'a>, Expr<'a>),
    NE(Expr<'a>, Expr<'a>),
    BitAnd(Expr<'a>, Expr<'a>),
    BitXor(Expr<'a>, Expr<'a>),
    BitOr(Expr<'a>, Expr<'a>),
    And(Expr<'a>, Expr<'a>),
    Or(Expr<'a>, Expr<'a>),
    IfElse(Expr<'a>, Expr<'a>, Expr<'a>),
    Pow(Expr<'a>, Expr<'a>),
    In(Expr<'a>, Expr<'a>, bool),
    Lambda(Vec<Param<'a>>, Expr<'a>),
  }

  // Normal-mode fragments:

  pub enum ModStmt<'a> {
    Import(ImportStmt<'a>),
    VarDecl(VarDecl<'a>),
    FnDef(FnDef<'a>),
    LSysDef(LSysDef<'a>),
  }

  pub enum LSysStmt<'a> {
    Stmt(Stmt<'a>),
    AxiomDef(Word<'a, char>),
    TableDef(Vec<Rule<'a, char>>),
    RulesDef(Vec<Rule<'a, char>>),
    ProductionRulesDef(Vec<Rule<'a, char>>),
    CodingRulesDef(Vec<Rule<'a, char>>),
    // LSysDef(LSysDef<'a, char>),
  }

  pub enum Stmt<'a> {
    Expr(Expr<'a>),
    Assign(&'a str, Expr<'a>),
    VarDecl(VarDecl<'a>),
    FnDef(FnDef<'a>),
    LSysDef(LSysDef<'a, char>),
    If(Expr<'a>, Stmt<'a>, Option<Stmt<'a>>),
    For(&'a str, Expr<'a>, Stmt<'a>),
    While(Expr<'a>, Stmt<'a>),
    Return(Option<Expr<'a>>),
    Block(Vec<Stmt<'a>>),
  }

  pub struct VarDecl<'a> {
    name: &'a str,
    value: Option<Expr<'a>>,
  }

  pub struct FnDef<'a> {
    name: &'a str,
    params: Vec<Param<'a>>,
    stmts: Vec<Stmt<'a>>,
  }

  pub struct LSysDef<'a, C> {
    name: Option<&'a str>,
    params: Vec<Param<'a>>,
    axiom: Vec<Word<'a, C>>,
    tables: Vec<RulesTable<'a, C>>,
    // default_table: RulesTable,
    names: Vec<&'a str>, // Vars, functions. Include tables' names?
    stmts: Vec<LSysStmt<'a>>,
  }

  pub struct Param<'a> {
    name: &'a str,
    default_value: Option<Expr<'a>>,
  }

  pub struct ImportStmt<'a> {
    module: &'a str,
    alias: Option<&'a str>,
    // symbols: Vec<&'a str>,
  }
}



/// This module contains grammar-mode ASTs.
pub mod grammar {
  /// This grammar-mode AST is returned by the LSD Rules parser.
  pub struct RulesTable<'a, C> {
    name: Option<&'a str>,
    rules: Vec<Rule<'a, C>>,
  }

  /// This grammar-mode AST is returned by the LSD Word parser.
  pub struct Word<'a, C> (Vec<Node<'a, C>>);

  // Grammar-mode fragments:

  pub enum Rule<'a, C> {
    Production(RuleBase<'a, C>),
    Coding(RuleBase<'a, C>),
  }

  pub struct RuleBase<'a, C> {
    weight: f64,
    leftLeaf: LeftLeaf<'a, C>,
    condition: Option<Expr<'a>>,
    lCtx: Vec<CtxNode<'a, C>>,
    rCtx: Vec<CtxNode<'a, C>>,
    rightSide: Word<'a, C>,
  }

  pub enum Node<'a, C> {
    Leaf(Leaf<'a, C>),
    Branch(Vec<Node<'a, C>>),
    Expansion(Expansion<'a>),
    Block(Vec<Stmt<'a>>),
  }

  pub enum CtxNode<'a, C> {
    Leaf(LeftLeaf<'a, C>),
    Branch(Vec<CtxNode<'a, C>>),
  }

  pub struct LeftLeaf<'a, C> {
    symbol: C,
    params: Option<Vec<&'a str>>,
  }
  pub struct Leaf<'a, C> {
    symbol: C,
    args: Option<Vec<Expr<'a>>>,
  }
  pub struct Expansion<'a> {
    to: &'a str,
    args: Option<Vec<Expr<'a>>>,
  }
}
