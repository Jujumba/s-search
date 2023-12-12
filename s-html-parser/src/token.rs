use std::{borrow::Cow, cell::{RefCell, RefMut}, marker::PhantomData};

use crate::{element::Element, error::TokenError};
pub(crate) struct Tokenizer<'a, 'b> {
    head: RefCell<usize>,
    content: &'b str,
    ph: PhantomData<&'a ()>
}
impl<'a, 'b> Tokenizer<'a, 'b> where 'b: 'a {
    pub fn new<I>(content: I) -> Self
    where
        I: Into<&'b str>,
    {
        Self {
            head: RefCell::new(0usize),
            content: content.into(),
            ph: PhantomData
        }
    }
    pub fn next_token(&'a self) -> Token<'b> {
        let mut head = self.get_head();
        
        let len = self.content.len();

        if *head >= len {
            return Token::eof(len);
        }

        let Some(pos) = self.content.chars().skip(*head).position(|c| !c.is_whitespace()) else {
            return Token::eof(*head);
        };

        *head += pos;
        let head_deref = *head;

        let current_char = self.content.chars().nth(head_deref).unwrap();
        if let Ok(kind) = current_char.try_into() {
            *head += 1;
            return Token::single(head_deref, kind);
        }

        let pos = self // relative to the `self.head`
            .content
            .chars()
            .skip(head_deref)
            .position(|c| <char as TryInto<TokenKind>>::try_into(c).is_ok() || c.is_whitespace())
            .unwrap_or(self.content.len() - head_deref);

        let end = head_deref + pos;
        let s: &'b str = &self.content[head_deref..end];

        *head += pos;
        if s.is_empty() {
            return Token::eof(*head);
        }

        Token::new(head_deref, pos, TokenKind::text(s))
    }
    pub fn has_spaces(&self) -> bool {
        let head = *self.get_head();
        self.content.chars().nth(head).unwrap().is_whitespace()
    }
    #[inline]
    fn get_head(&self) -> RefMut<usize> {
        self.head.borrow_mut()
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
}
impl<'a, 'b> Iterator for Tokenizer<'a, 'b> where 'b: 'a {
    type Item = Token<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
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
