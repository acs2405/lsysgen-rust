// use std::string::String;
// use regex::Regex;

// static tokens = vec![
//   Token::new("int", r"[0-9]+"),
//   Token::new("float", r"[0-9]*\.[0-9]+"),
//   Token::new("string", r#"(?:[^\\]|\\["])*"#),
//   Token::new("null", r"null"),
//   Token::new("true", r"true"),
//   Token::new("false", r"false"),
//   Token::new("id", r"[a-zA-Z_][a-zA-Z0-9_]*"),
//   Token::new("+", r"\+"),
//   Token::new("-", r"-"),
//   Token::new("*", r"\*"),
//   Token::new("/", r"/"),
//   Token::new("%", r"%"),
//   Token::new("^", r"\^"),
//   Token::new("(", r"\("),
//   Token::new(")", r"\)"),
// ];

// struct TokenType {
//   pub name: String,
//   pub re: Regex,
// }

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// The lexer's mode.
///
/// This is shared between the lexer and the parser.
#[derive(Debug)]
pub enum LexerMode {
  /// Normal mode: statements and expressions.
  Normal,
  /// Grammar mode: L-system rules and words.
  Grammar,
}

// impl Default for LexerMode {
//   fn default() -> Self {
//     Self::Normal
//   }
// }

// impl LexerMode {
//   pub fn new() -> Self {
//     Self::Normal
//   }
// }

#[derive(PartialEq, Eq, Debug)]
pub enum LexicalError {
  LengthOverflow(String),
  TruncatedInput(String),
}

impl fmt::Display for LexicalError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub type Spanned<Token, Loc, LexicalError> = Result<(Loc, Token, Loc), LexicalError>;

// The type of the parser's input.
//
// The parser iterators over tuples consisting of the token's starting
// position, the token itself, and the token's ending position.
pub(crate) type LexerItem<Token, Loc, LexicalError> = Spanned<Token, Loc, LexicalError>;

// #[derive(Debug, Clone, PartialEq)]
// #[allow(non_camel_case_types)]
// pub enum Token<'a> {
//   COLON,
//
//   // Whitespace.
//   SPACE,
//   HTAB,
//   VTAB,
//   CR,
//   LF,
//   FORMFEED,
//
//   // Digits.
//   N_0,
//   N_1,
//   N_2,
//   N_3,
//   N_4,
//   N_5,
//   N_6,
//   N_7,
//   N_8,
//   N_9,
//
//   // Other.
//   OTHER(&'a [u8]),
//
//   // Returned when the lexer is in literal mode.
//   LITERAL(&'a [u8]),
// }

/// Normal-mode token types.
#[derive(Debug, Clone, Eq)]
// #[allow(non_camel_case_types)]
pub enum TokenType {
  // Identifiers
  Id,
  AtId,
  Accessor,
  // LSysId, //$koch, @koch

  // Literal values
  Int,
  Float,
  String,
  Null,
  True,
  False,

  // Operators
  Assign,

  // Arithmetic Operators
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,

  // Comparison operators
  EQ,
  NE,
  LT,       // Normal + Rule modes
  LE,
  GT,       // Normal + Rule modes
  GE,

  // Logic operators
  And,
  Or,
  Not,

  // Logic operators
  BitAnd,
  BitOr,
  BitXor,
  BitNot,

  // Various brackets
  LParen,   // Normal + Rule modes
  RParen,   // Normal + Rule modes
  LBracket, // Normal + Rule modes
  RBracket, // Normal + Rule modes
  LBrace,   // Normal + Rule modes
  RBrace,   // Normal + Rule modes

  // Various symbols
  Dot,
  Comma,
  Colon,    // Normal + Rule modes
  QM, //?
  XM, //!
  // At, //@
  Arrow,    // Normal + Rule modes
  DArrow,   // Normal + Rule modes

  // Separators
  SemiColon,
  NewLine,

  // Keywords
  // And,
  Axiom,
  Coding,
  // Do,
  Else,
  For,
  // From,
  Fn,
  If,
  // Import,
  In,
  Inf,
  Let,
  Lsys,
  Main,
  // Mut,
  NaN,
  // Not,
  Null,
  // Or,
  Production,
  Return,
  Rules,
  Set,
  Table,
  // Then,
  While,

//   Other,
// }
//
// /// Grammar-mode token types.
// #[derive(Debug, Clone, Eq)]
// // #[allow(non_camel_case_types)]
// pub enum GrammarToken {

  Symbol,
  // AtID,
  // Arrow,
  // DArrow,
  // Colon,
  // Pipe,
  // LAngle,
  // RAngle,
  // LParen,
  // RParen,
  // LBranch,
  // RBranch,
  // LBlock,
  // RBlock,
  //
  // SemiColon,
  // NewLine,
  //
  Other,
}

// #[derive(Debug, Clone, Eq)]
// pub enum TokenType {
//   Normal(NormalToken),
//   Rule(GrammarToken),
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
  pub ttype: TokenType,
  pub bytes: &'a [u8],
}

impl fmt::Display for Token<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl<'a> Token<'a> {
  pub fn new(ttype: TokenType, bytes: &'a [u8]) {
    Token {ttype: ttype, bytes: bytes}
  }

  pub fn as_bytes(&self) -> &'a [u8] {
    self.bytes
  }
}

#[derive(Debug)]
pub struct Lexer<'input> {
  input: &'input [u8],

  // Offset into the original input.
  offset: usize,

  // The lexer's mode.  This is behind a `Rc<RefCell>` so that it
  // is also accessible to the parser.
  pub mode: Rc<RefCell<LexerMode>>,
}

impl<'input> Lexer<'input> {
  pub fn new(input: &'input [u8], mode: LexerMode) -> Self {
    Lexer {
      input,
      offset: 0,
      mode: Rc::new(RefCell::new(mode)),
    }
  }

  fn normal_mode_next(&mut self) {
    // TODO: Generar lexer
    if self.input.len() < count {
      return Some(Err(LexicalError::TruncatedInput(format!(
        "Expected {} octets, got {}",
        count,
        self.input.len()
      ))));
    } else {
      (count, LITERAL(&self.input[..count]))
    }
  }

  fn symbol_next(&mut self) {
    // TODO: Generar lexer
    match *self.input.first()? as char {
      ':' => (1, COLON),

      // Whitespace.
      ' ' => (1, SPACE),
      '\t' => (1, HTAB),
      '\u{0b}' => (1, VTAB),
      '\u{0d}' => (1, CR),
      '\u{0a}' => (1, LF),
      '\u{0c}' => (1, FORMFEED),

      // Digits.
      '0' => (1, N_0),
      '1' => (1, N_1),
      '2' => (1, N_2),
      '3' => (1, N_3),
      '4' => (1, N_4),
      '5' => (1, N_5),
      '6' => (1, N_6),
      '7' => (1, N_7),
      '8' => (1, N_8),
      '9' => (1, N_9),

      // Other.
      _ => (1, OTHER(&self.input[..1])),
    }
  }
}

impl<'input> Iterator for Lexer<'input> {
  type Item = LexerItem<Token<'input>, usize, LexicalError>;

  fn next(&mut self) -> Option<Self::Item> {
    use self::Token::*;

    // let limit = self.input.len();
    // let mut cursor: usize = 0;
    // let mut marker: usize;
    // let mut ctxmarker: usize;
    let mut mode = self.mode.borrow_mut();
    let yyinput = self.input;
    let mut yycursor = 0usize;
    let tokenType = // re2rust $INPUT -o $OUTPUT --no-unsafe --api simple
      /*!re2c
         re2c:YYCTYPE = "u8";

         [ \t]+ {}
         "//" .* {}
         "/*" .* "*/" {}
         [\n\r] {return Some(TokenType::NewLine)}
         [a-zA-Z_] [a-zA-Z_0-9]* {return Some(TokenType::Id)}
         [0-9] [0-9]* {return Some(TokenType::Int)}
         * {return None}
       */

    // TODO: Generar cualquier token que no empiece por: [a-zA-Z0-9+-:=/&%#·, caracteres de escape y quizá alguna cosa más].
    // Ignorar espacios y comentarios.
    let (len, token) = match mode {

      LexerMode::Normal => {
        self::normal_mode_next()
      },
      LexerMode::Grammar => {
        self::symbol_next()
      }
    };

    self.input = &self.input[len..];

    let start = self.offset;
    let end = start + len;
    self.offset += len;

    Some(Ok((start, token, end)))
  }
}

impl<'input> From<&'input [u8]> for Lexer<'input> {
  fn from(i: &'input [u8]) -> Lexer<'input> {
    Lexer::new(i)
  }
}
