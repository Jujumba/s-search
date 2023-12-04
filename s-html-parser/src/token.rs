use std::{borrow::Cow, cell::RefCell};

use crate::{element::Element, error::TokenError};
pub(crate) struct Tokenizer<'a> {
    head: RefCell<usize>,
    content: Cow<'a, str>, // todo: may be I should use a borrowed string
}
impl<'a> Tokenizer<'a> {
    pub fn new<I>(content: I) -> Self
    where
        I: Into<Cow<'a, str>>,
    {
        Self {
            head: RefCell::new(0usize),
            content: content.into(),
        }
    }
    pub fn next_token(&'a self) -> Token<'a> {
        let mut head = self.head.borrow_mut();
        let head_deref = *head;
        let len = self.content.len();

        if head_deref >= len {
            return Token::eof(len);
        }

        let current_char = self.content.chars().nth(head_deref).unwrap();
        if let Ok(kind) = current_char.try_into() {
            *head += 1;
            return Token::single(head_deref, kind);
        }

        let pos = self // relative to the `self.head`
            .content
            .chars()
            .skip(head_deref)
            .position(|c| <char as TryInto<TokenKind>>::try_into(c).is_ok())
            .unwrap_or(self.content.len() - head_deref);

        let end = head_deref + pos;
        let s = &self.content[head_deref..end];

        *head += pos;
        if s.is_empty() {
            return Token::eof(*head);
        }

        Token::new(head_deref, pos, TokenKind::text(s))
    }
    pub fn token_to_str(&'a self, token: &Token<'a>) -> &'a str {
        let end = token.start + token.len;
        &self.content[token.start..end]
    }
    pub fn token_to_str_empty(&'a self, token: &Token<'a>) -> bool {
        self.token_to_str(token).trim().is_empty()
    }
    pub fn token_to_text_element(&'a self, token: &Token<'a>) -> Element<'a> {
        let text: Cow<'a, str> = self.token_to_str(token).into();
        Element::text(text)
    }
    pub fn concat(&'a self, t1: &Token<'a>, t2: &Token<'a>) -> Token<'a> {
        let start = t1.start;
        let end = t1.start + t1.len + t2.len;
        let text: &'a str = &self.content[start..end];
        let kind = TokenKind::text(text);
        Token::new(start, end - start, kind)
    }
}
#[allow(clippy::from_over_into)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Token<'a> {
    pub(crate) start: usize,
    pub(crate) len: usize,
    pub(crate) kind: TokenKind<'a>,
}
impl<'a> Token<'a> {
    pub fn new(start: usize, len: usize, kind: TokenKind<'a>) -> Self {
        Self { start, len, kind }
    }
    pub fn eof(start: usize) -> Self {
        Self {
            start,
            kind: TokenKind::Eof,
            len: 0usize,
        }
    }
    pub fn single(start: usize, kind: TokenKind<'a>) -> Self {
        Self {
            start,
            kind,
            len: 1usize,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TokenKind<'a> {
    /// Corresponds to `<``
    LAngle,
    /// Corresponds to `>`
    RAngle,
    /// Corresponds to `/`
    Backslash,
    /// Corresponds to `=`
    Equals,
    /// Corresponds to `!`
    Exclamation,
    /// Any text inside the html tag
    Text(Cow<'a, str>),
    /// End of file
    Eof,
}
impl<'a> TokenKind<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn text<C>(text: C) -> Self
    where
        C: Into<Cow<'a, str>>,
    {
        // to make testing easier
        Self::Text(text.into())
    }
}
impl<'a> TryFrom<char> for TokenKind<'a> {
    type Error = TokenError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::LAngle),
            '>' => Ok(Self::RAngle),
            '/' => Ok(Self::Backslash),
            '=' => Ok(Self::Equals),
            '!' => Ok(Self::Exclamation),
            _ => Err(TokenError::NonCharToken(value)),
        }
    }
}
