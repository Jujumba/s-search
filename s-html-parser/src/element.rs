use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

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
    pub fn tag<I, A>(ident: I, attrs: A) -> Self where I: Into<Cow<'a, str>>, A: Into<HashMap<Cow<'a, str>, Cow<'a, str>>> {
        Self::Tag {
            ident: ident.into(),
            attrs: attrs.into()
        }
    }
    pub fn text<T>(text: T) -> Self where T: Into<Cow<'a, str>> {
        Self::Text(text.into())
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnstructuredSequence<'a>(Vec<Element<'a>>);

impl<'a> From<&'a str> for Element<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(text.into())
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
