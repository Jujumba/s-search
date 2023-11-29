use std::{borrow::Cow, cell::RefCell};

use crate::error::TokenError;
pub(crate) struct Tokenizer {
    head: RefCell<usize>,
    content: String, // todo: may be I should use a borrowed string
}
impl Tokenizer {
    pub fn new<I>(content: I) -> Self
    where
        I: Into<String>,
    {
        Self {
            head: RefCell::new(0usize),
            content: content.into(),
        }
    }
    pub fn next_token(&self) -> Token {
        self.next_token_raw(false)
    }
    pub fn next_token_by_word(&self) -> Token {
        self.next_token_raw(true)
    }
    fn next_token_raw(&self, word_by_word: bool) -> Token {
        let mut head = self.head.borrow_mut();
        let head_deref = *head;

        if head_deref >= self.content.len() {
            return Token::Eof;
        }

        let current_char = self.content.chars().nth(head_deref).unwrap();
        if let Ok(token) = current_char.try_into() {
            *head += 1;
            return token;
        }

        let closure = if word_by_word {
            |c: char| c.is_whitespace()
        } else {
            |_c: char| false
        };

        let pos = self // relative to the `self.head`
            .content
            .chars()
            .skip(head_deref)
            .position(|c| c == '<' || c == '>' || closure(c))
            .unwrap_or(self.content.len() - head_deref);

        let end = head_deref + pos;
        let s = (self.content[head_deref..end]).trim();

        *head += pos;
        if s.is_empty() {
            return Token::Eof;
        }

        Token::Text(s.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token<'a> {
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
impl<'a> Token<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn text<C>(text: C) -> Self where C: Into<Cow<'a, str>> { // to make testing easier
        Self::Text(text.into())
    }
}
impl<'a> TryFrom<char> for Token<'a> {
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
