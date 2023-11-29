use std::collections::HashMap;

use crate::element::{Element, UnstructuredSequence};
use crate::error::ParseError;
use crate::on_err;
use crate::token::{Token, Tokenizer};

/// The unstructured parser.
/// Does not build the tree of elements or whatever.
/// Such parser never fails, it does not ensure HTML correctness.
/// ```
/// use std::collections::HashMap;
/// 
/// use crate::s_html_parser::element::{Element, UnstructuredSequence};
/// use crate::s_html_parser::parser::Parser;
/// 
/// let parser = Parser::new("<p> Hello! </p>");
/// let seq: UnstructuredSequence  = parser.parse();
/// 
/// // an element with empty attributes
/// assert_eq!(seq[0], Element::tag("p", HashMap::new()));
/// assert_eq!(seq[1], Element::text("Hello!"));
/// assert_eq!(seq[2], Element::tag("p", HashMap::new()));
/// assert!(seq.get(3).is_none());
/// ```
pub struct Parser {
    pub(crate) tokenizer: Tokenizer,
}
impl Parser {
    pub fn new<A>(content: A) -> Self where A: Into<String> {
        Self {
            tokenizer: Tokenizer::new(content),
        }
    }
    pub fn parse(&self) -> UnstructuredSequence {
        let mut seq = UnstructuredSequence::default();
        loop {
            let element = match self.tokenizer.next_token() {
                Token::LAngle => on_err!(self.parse_tag(), continue),
                Token::Text(s) => s.into(),
                Token::Eof => break,
                _ => continue // todo: we don't care about the erroneous tag now, but may in the future
            };
            seq.push(element);
        }
        seq
    }
    fn parse_tag(&self) -> Result<Element, ParseError> {
        let ident = match self.tokenizer.next_token() {
            Token::Text(ident) => ident,
            Token::Exclamation | Token::Backslash => {
                let Token::Text(ident) = self.tokenizer.next_token() else {
                    return Err(ParseError::Unexpected);
                };
                ident
            }
            _ => return Err(ParseError::Unexpected),
        };

        let mut attrs = HashMap::new();
        let mut token = self.tokenizer.next_token();
        loop {
            if matches!(token, Token::RAngle) {
                break
            }
            let Token::Text(key) = token else {
                return Err(ParseError::Unexpected);
            };
            let Token::Equals = self.tokenizer.next_token() else {
                return Err(ParseError::Unexpected);
            };
            let Token::Text(value) = self.tokenizer.next_token() else {
                return Err(ParseError::Unexpected);
            };
            attrs.insert(key, value);
            token = self.tokenizer.next_token();
        }

        Ok(Element::Tag { ident, attrs })
    }
}
