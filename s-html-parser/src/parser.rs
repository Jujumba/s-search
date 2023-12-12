use std::collections::HashMap;

use crate::element::{Element, UnstructuredSequence};
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
/// assert_eq!(seq[1], Element::text("Hello"));
/// assert_eq!(seq[2], Element::text("!"));
/// assert_eq!(seq[3], Element::tag("p", HashMap::new()));
/// assert!(seq.get(4).is_none());
/// ```
pub struct Parser<'a, 'b> {
    pub(crate) tokenizer: Tokenizer<'a, 'b>,
}
impl<'a, 'b> Parser<'a, 'b> where 'b: 'a {
    pub fn new<A>(content: A) -> Self
    where
        A: Into<&'b str>,
    {
        Self {
            tokenizer: Tokenizer::new(content),
        }
    }
    pub fn parse(&self) -> UnstructuredSequence {
        let mut seq = UnstructuredSequence::default();

        loop {
            let token = self.tokenizer.next_token();
            let element = match token.kind {
                TokenKind::LAngle => on_err!(self.parse_tag(), continue),
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
    fn parse_tag(&'a self) -> Option<Element<'a>> {
        if self.tokenizer.has_spaces() {
            return Some(Element::text("<"));
        }
        let token: Token<'a> = self.tokenizer.next_token();
        let ident = match token.kind.clone() {
            TokenKind::Text(ident) => ident,
            TokenKind::Exclamation => match self.tokenizer.next_token().kind {
                TokenKind::Text(tag) if tag == "--" => {
                    self.skip_comment();
                    return None;
                }
                TokenKind::Text(tag) => tag,
                _ => return None,
            },
            TokenKind::Backslash => {
                expect_token!(self, TokenKind::Text(ident), None);
                ident
            }
            _ => return None,
        };

        let mut attrs = HashMap::new();
        loop {
            let token_kind = self.tokenizer.next_token().kind;
            if matches!(token_kind, TokenKind::RAngle) {
                break;
            }
            if matches!(token_kind, TokenKind::Backslash) {
                expect_token!(self, TokenKind::RAngle, None);
                break
            }
            expect_token!(self, TokenKind::Text(key), None);
            expect_token!(self, TokenKind::Equals, None);
            expect_token!(self, TokenKind::Text(value), None);
            attrs.insert(key, value);
        }

        Some(Element::tag(ident, attrs))
    }
    // should only be called *after* a comment tag (<!--)
    fn skip_comment(&'a self) {
        loop {
            let token = self.tokenizer.next_token();
            if let TokenKind::Text(s) = token.kind { // sorry
                if s == "--" {
                    let next = self.tokenizer.next_token();
                    if matches!(next.kind, TokenKind::RAngle) {
                        break;
                    }
                }
            }
        }
    }
}
