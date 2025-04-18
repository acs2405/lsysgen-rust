use std::vec::Vec;

pub mod lexer;
pub mod ast;

#[cfg(test)]
mod test;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);
use grammar::LsdModuleParser;
use grammar::LsdExprParser;
use grammar::LsdWordParser;
use grammar::LsdRulesParser;

pub fn parse_lsd_module<'input>(&'input input) -> Option<Module<'input>> {
  let lexer = Lexer::new(input, LexerMode::Normal);
  let mode = Rc::clone(&lexer.mode);

  return LsdModuleParser::new().parse(&mode, lexer); //.unwrap();
}

pub fn parse_expr<'input>(&'input input) -> Option<Expr<'input>> {
  let lexer = Lexer::new(input, LexerMode::Normal);
  let mode = Rc::clone(&lexer.mode);

  return LsdExprParser::new().parse(&mode, lexer); //.unwrap();
}

pub fn parse_word<'input>(&'input input) -> Option<Vec<Node<'input>>> {
  let lexer = Lexer::new(input, LexerMode::Rule);
  let mode = Rc::clone(&lexer.mode);

  return LsdWordParser::new().parse(&mode, lexer); //.unwrap();
}

pub fn parse_rules<'input>(&'input input) -> Option<Vec<Rule<'input>>> {
  let lexer = Lexer::new(input, LexerMode::Rule);
  let mode = Rc::clone(&lexer.mode);

  return LsdRulesParser::new().parse(&mode, lexer); //.unwrap();
}
