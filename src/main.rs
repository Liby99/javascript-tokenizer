#![feature(const_fn, box_syntax, pattern, proc_macro_hygiene)]
#![warn(variant_size_differences)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate phf;

pub mod token;
#[macro_use]
pub mod macros;
pub mod equivalence;
pub mod error;
pub mod internship { extern crate internship; pub use internship::*; }
pub mod identifier;
pub mod number;
pub mod state;
pub mod state_machine;
pub mod string;

use std::env;
use std::fs;
use std::path::PathBuf;

mod lexer;

use lexer::Lexer;
use token::{Token, Number};

fn num_to_string(num: Number) -> String {
  match (num.integer, num.decimal, num.exponent) {
    (0, 0, _) => format!("0"),
    (0, d, 0) => format!(".{}", d),
    (i, 0, 0) => format!("{}", i),
    (0, d, e) => format!(".{}e{}", d, e),
    (i, d, 0) => format!("{}.{}", i, d),
    (i, 0, e) => format!("{}e{}", i, e),
    (i, d, e) => format!("{}.{}e{}", i, d, e),
  }
}

fn token_to_string<'a>(token: Token) -> String {
  match token {
    Token::And => String::from("&"),
    Token::AndAssign => String::from("&="),
    Token::Assign => String::from("="),
    Token::AssignBigger => String::from("=>"),
    Token::Bigger => String::from(">"),
    Token::BiggerEqual => String::from(">="),
    Token::BoolLiteral(bool) => match bool {
      true => String::from("true"),
      false => String::from("false"),
    },
    Token::Caret => String::from("^"),
    Token::CaretAssign => String::from("^="),
    Token::Colon => String::from(":"),
    Token::Comma => String::from(","),
    Token::Semicolon => String::from(";"),
    Token::Slash => String::from("/"),
    Token::SlashAssign => String::from("/="),
    Token::Dot => String::from("."),
    Token::DoubleAnd => String::from("&&"),
    Token::DoubleAssign => String::from("=="),
    Token::DoubleBigger => String::from("\\>>"),
    Token::DoubleBiggerEqual => String::from("\\>>="),
    Token::DoubleLesser => String::from("<<"),
    Token::DoubleLesserEqual => String::from("<<="),
    Token::DoubleMinus => String::from("--"),
    Token::DoubleOr => String::from("||"),
    Token::DoublePlus => String::from("++"),
    Token::DoubleStar => String::from("**"),
    Token::DoubleStarAssign => String::from("**="),
    Token::EOF => String::from(""),
    Token::Regex(s) => format!("/{}/", s),
    Token::Exclamation => String::from("!"),
    Token::ExclamationAssign => String::from("!="),
    Token::ExclamationDoubleAssign => String::from("!=="),
    Token::IdentifierName(id) => String::from(id.as_str()),
    Token::KAs => String::from("as"),
    Token::KAsync => String::from("async"),
    Token::KAwait => String::from("await"),
    Token::KBreak => String::from("break"),
    Token::KCase => String::from("case"),
    Token::KCatch => String::from("cache"),
    Token::KClass => String::from("class"),
    Token::KConst => String::from("const"),
    Token::KContinue => String::from("continue"),
    Token::KDebugger => String::from("debugger"),
    Token::KDefault => String::from("default"),
    Token::KDelete => String::from("delete"),
    Token::KDo => String::from("do"),
    Token::KElse => String::from("else"),
    Token::KExtend => String::from("extend"),
    Token::KFinally => String::from("finally"),
    Token::KFor => String::from("for"),
    Token::KFrom => String::from("from"),
    Token::KFunction => String::from("function"),
    Token::KGet => String::from("get"),
    Token::KIf => String::from("if"),
    Token::KIn => String::from("in"),
    Token::KImport => String::from("import"),
    Token::KLet => String::from("let"),
    Token::KNew => String::from("new"),
    Token::KOf => String::from("of"),
    Token::KReturn => String::from("return"),
    Token::KSet => String::from("set"),
    Token::KStatic => String::from("static"),
    Token::KSwitch => String::from("switch"),
    Token::KThis => String::from("this"),
    Token::KThrow => String::from("throw"),
    Token::KTry => String::from("try"),
    Token::KTypeof => String::from("typeof"),
    Token::KVar => String::from("var"),
    Token::KVoid => String::from("void"),
    Token::KWhile => String::from("while"),
    Token::KWith => String::from("with"),
    Token::LCurly => String::from("{"),
    Token::LessEqual => String::from("<="),
    Token::Lesser => String::from("<<"),
    Token::LineTerminator => String::from("\n"),
    Token::LNull => String::from("null"),
    Token::LUndefined => String::from("undefined"),
    Token::LRound => String::from("("),
    Token::LSquare => String::from("["),
    Token::Minus => String::from("-"),
    Token::MinusAssign => String::from("-="),
    Token::NumericLiteral(num) => num_to_string(num),
    Token::Or => String::from("|"),
    Token::OrAssign => String::from("|="),
    Token::Percent => String::from("%"),
    Token::PercentAssign => String::from("%="),
    Token::Plus => String::from("+"),
    Token::PlusAssign => String::from("+="),
    Token::QuestionMark => String::from("?"),
    Token::RCurly => String::from("}"),
    Token::RRound => String::from(")"),
    Token::RSquare => String::from("]"),
    Token::Star => String::from("*"),
    Token::StarAssign => String::from("*="),
    Token::StringLiteral(s) => format!("\"{}\"", s),
    Token::Template(s) => s,
    Token::Tilde => String::from("~"),
    Token::TripleAssign => String::from("==="),
    Token::TripleBigger => String::from(">>>"),
    Token::TripleBiggerEqual => String::from(">>>="),
    Token::TripleDot => String::from("..."),
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let mut raw_token = false;
  let mut print_help = false;
  let mut preprocess = false;
  let mut maybe_file_path = None;

  for arg in &args[1..] {
    match arg.as_str() {
      "-r" | "--raw" => raw_token = true,
      "-p" | "--preprocess" => preprocess = true,
      "-h" | "--help" => print_help = true,
      _ => if maybe_file_path.is_none() { maybe_file_path = Some(String::from(arg)) },
    }
  }

  if print_help {
    println!("Javascript Tokenizer");
    println!("Will by default print out the JSON format string array containing tokens");
    println!("Usage:");
    println!("\ttokenize [-r|--raw] [-p|--preprocess] [-h|--help] file.js");
    println!("Options:");
    println!("\t-r | --raw:\tIf enabled, print out the raw tokens instead of JS tokens");
    println!("\t-p | --preprocess:\tIf enabled, will filter out all new-line tokens");
    println!("\t-h | --help:\tPrint out this help message");
  } else {
    if let Some(file_path) = maybe_file_path {
      if let Ok(full_path) = fs::canonicalize(&PathBuf::from(file_path.clone())) {
        if let Ok(file_content) = fs::read_to_string(full_path.clone()) {
          if let Ok(tokens) = Lexer::lex_tokens(format!("{}\n", file_content).as_str()) {

            // Do preprocess to filter out all line terminator/EOF if needed
            let tokens = if preprocess {
              tokens.into_iter().filter(|t| match t {
                Token::LineTerminator => false,
                Token::EOF => false,
                _ => true
              }).collect::<Vec<_>>()
            } else {
              tokens
            };

            // Print the tokens
            if raw_token {
              println!("{:?}", tokens);
            } else {
              println!("{:?}", tokens.into_iter().map(token_to_string).collect::<Vec<_>>());
            }
          } else {
            eprintln!("Unable to tokenize");
            std::process::exit(4);
          }
        } else {
          eprintln!("Cannot read file {:?}", full_path);
          std::process::exit(3);
        }
      } else {
        eprintln!("Unknown file name {}", file_path);
        std::process::exit(2);
      }
    } else {
      eprintln!("Please specify a file name");
      std::process::exit(1);
    }
  }
}