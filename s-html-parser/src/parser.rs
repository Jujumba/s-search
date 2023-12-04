use std::borrow::Cow;
use std::collections::HashMap;

use crate::element::{Element, UnstructuredSequence};
use crate::error::ParseError;
use crate::{on_err, expect_token};
use crate::token::{Token, TokenKind, Tokenizer};

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
/// assert_eq!(seq[1], Element::text(" Hello"));
/// assert_eq!(seq[2], Element::text("!"));
/// assert_eq!(seq[3], Element::tag("p", HashMap::new()));
/// assert!(seq.get(4).is_none());
/// ```
pub struct Parser<'a> {
    pub(crate) tokenizer: Tokenizer<'a>,
}
impl<'a> Parser<'a> {
    pub fn new<A>(content: A) -> Self
    where
        A: Into<Cow<'a, str>>,
    {
        Self {
            tokenizer: Tokenizer::new(content),
        }
    }
    pub fn parse(&self) -> UnstructuredSequence {
        let mut seq = UnstructuredSequence::default();

        loop {
            let token = self.tokenizer.next_token();
            let element = match token.kind.clone() {
                TokenKind::LAngle => on_err!(self.parse_tag(token), continue),
                TokenKind::Text(s) if !s.trim().is_empty() => s.into(),
                TokenKind::Eof => break,
                // it would be better if I would merge all text elements in a row in to a single one
                _ if !self.tokenizer.token_to_str_empty(&token) => self.tokenizer.token_to_text_element(&token),
                _ => continue
            };
            seq.push(element);
        }

        seq
    }
    fn parse_tag(&'a self, langle: Token<'a>) -> Result<Element<'a>, ParseError> {
        let token: Token<'a> = self.tokenizer.next_token();
        let ident = match token.kind.clone() {
            TokenKind::Text(ident) if ident.starts_with(' ') => {
                let merged: Element<'a> =
                    self.tokenizer.concat(&langle, &token).try_into().unwrap();
                return Ok(merged);
            }
            TokenKind::Text(ident) => ident,
            TokenKind::Exclamation => match self.tokenizer.next_token().kind {
                TokenKind::Text(tag) if tag == "--" => {
                    todo!("skip to end of comment tag and return it")
                }
                TokenKind::Text(tag) => tag,
                _ => return Err(ParseError::Unexpected),
            },
            TokenKind::Backslash => {
                expect_token!(self, TokenKind::Text(ident), Err(ParseError::Unexpected));
                ident
            }
            _ => return Err(ParseError::Unexpected),
        };

        let mut attrs = HashMap::new();
        loop {
            let token_kind = self.tokenizer.next_token().kind;
            if matches!(token_kind, TokenKind::RAngle) {
                break;
            }
            if matches!(token_kind, TokenKind::Backslash) {
                expect_token!(self, TokenKind::RAngle, Err(ParseError::Unexpected));
                break
            }
            expect_token!(self, TokenKind::Text(key), Err(ParseError::Unexpected));
            expect_token!(self, TokenKind::Equals, Err(ParseError::Unexpected));
            expect_token!(self, TokenKind::Text(value), Err(ParseError::Unexpected));
            attrs.insert(key, value);
        }

        Ok(Element::Tag { ident, attrs })
    }
}
