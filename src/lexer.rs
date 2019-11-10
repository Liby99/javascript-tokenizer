//! Lexer module consumes text input and produces linear stream of tokens.
//!
//! Tokens are defined in [token](lexer::token)
// Grammar rules are in [div](lexer::div)
//
// Supporting macro in [macros](lexer::macros)


use std::str;

use super::{state_machine::parse, token::*};
use super::error;

/// Lexer implmementation
#[derive(Debug, Copy, Clone)]
pub struct Lexer;

impl Lexer {
    /// Transform string to stream of tokens
    pub fn lex_tokens(s: &str) -> Result<Vec<Token>, error::Error> {
        let mut tokens = parse(s)?;
        tokens.push(Token::EOF);
        Ok(tokens)
        // Ok(vec![])
    }
}
