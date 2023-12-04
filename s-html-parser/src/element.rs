use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::token::{TokenKind, Token};

// todo: more comprehensive docs
/// A very specific representation of HTML-element
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element<'a> {
    /// Any HTML tag
    Tag {
        ident: Cow<'a, str>,
        attrs: HashMap<Cow<'a, str>, Cow<'a, str>>,
    },
    /// Text contained within a tag
    Text(Cow<'a, str>),
}
impl<'a> Element<'a> {
    #[inline]
    pub fn tag<I, A>(ident: I, attrs: A) -> Self where I: Into<Cow<'a, str>>, A: Into<HashMap<Cow<'a, str>, Cow<'a, str>>> {
        Self::Tag {
            ident: ident.into(),
            attrs: attrs.into()
        }
    }
    #[inline]
    pub fn no_attrs<I>(ident: I) -> Self where I: Into<Cow<'a, str>> {
        Self::tag(ident, HashMap::new())
    }
    #[inline]
    pub fn text<T>(text: T) -> Self where T: Into<Cow<'a, str>> {
        Self::Text(text.into())
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnstructuredSequence<'a>(Vec<Element<'a>>);

impl<'a> IntoIterator for UnstructuredSequence<'a> {

    type IntoIter = <Vec<Element<'a>> as IntoIterator>::IntoIter;
    type Item = Element<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter() 
    }
}

impl<'a> From<&'a str> for Element<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(text.into())
    }
}
impl<'a> TryFrom<TokenKind<'a>> for Element<'a> {
    type Error = (); // todo
    fn try_from(value: TokenKind<'a>) -> Result<Self, Self::Error> {
        let TokenKind::Text(text) = value else {
            return Err(());
        };
        Ok(Self::text(text))
    }
}
impl<'a> TryFrom<Token<'a>> for Element<'a> {
    type Error = (); // todo
    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        let TokenKind::Text(text) = value.kind else {
            return Err(());
        };
        Ok(Self::text(text))
    }
}
impl<'a> From<Cow<'a, str>> for Element<'a> {
    fn from(text: Cow<'a, str>) -> Self {
        Self::Text(text)
    }
}
impl<'a> From<String> for Element<'a> {
    fn from(text: String) -> Self {
        Self::Text(text.into())
    }
}

impl<'a> From<Vec<Element<'a>>> for UnstructuredSequence<'a> {
    fn from(value: Vec<Element<'a>>) -> Self {
        Self(value)
    }
}
impl<'a> Deref for UnstructuredSequence<'a> {
    type Target = Vec<Element<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> DerefMut for UnstructuredSequence<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
