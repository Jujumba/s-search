pub mod element;
pub mod error;
pub mod macros;
pub mod parser;
pub mod token;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::element::Element;
    use crate::parser::Parser;
    use crate::token::{Tokenizer, TokenKind};
    macro_rules! tokenizer {
        ($text:tt) => {{
            Tokenizer::new($text) 
        }}
    }
    macro_rules! parser {
        ($text:tt) => {{
            Parser::new($text)
        }}
    }
    #[test]
    fn tokens_ordered() {
        let tokenizer = tokenizer!("<<>>!!!<< text<////Abcdefgh//!");

        let mut prev = tokenizer.next_token();
        loop {
            let next = tokenizer.next_token();
            if matches!(next.kind, TokenKind::Eof) {
                break;
            }
            if next.start - prev.start - prev.len != 0 {
                panic!("There is a gap between tokens: prev - {prev:?}, next - {next:?}");
            }
            prev = tokenizer.next_token();
        }
    }
    #[test] 
    fn tokenizer_basic() {
        let tokenizer = tokenizer!("Hello <world");

        assert_eq!(tokenizer.next_token().kind, TokenKind::text("Hello "));
        assert_eq!(tokenizer.next_token().kind, TokenKind::LAngle);
        assert_eq!(tokenizer.next_token().kind, TokenKind::text("world"));
    }
    #[test]
    fn tokenizer_exclamation_in_text() {
        let tokenizer = tokenizer!("This is a text...");

        assert_eq!(tokenizer.next_token().kind, TokenKind::text("This is a text..."))
    }
    #[test] 
    #[ignore = "tokenizer should not trim"]
    fn tokenizer_trims_ends() {
        let tokenizer = tokenizer!("    A wizard is never late   !  <tag>   ");

        assert_eq!(tokenizer.next_token().kind, TokenKind::text("A wizard is never late   !"));
        assert_eq!(tokenizer.next_token().kind, TokenKind::LAngle);
        assert_eq!(tokenizer.next_token().kind, TokenKind::text("tag"));
        assert_eq!(tokenizer.next_token().kind, TokenKind::RAngle);
        assert_eq!(tokenizer.next_token().kind, TokenKind::Eof);
    }
    #[test]
    fn parser_basic() {
        let parser = parser!("<p>Text   !");
        let sequence = parser.parse();

        assert_eq!(sequence[0], Element::tag("p", HashMap::new()));
        assert_eq!(sequence[1], Element::text("Text   "));
        assert_eq!(sequence[2], Element::text("!"));
    }
    #[test]
    fn parser_unclosed_tag() {
        let parser = parser!("<pText   !");
        let sequence = parser.parse();
        assert!(sequence.is_empty());
    }
    #[test]
    fn parser_non_tag_as_text() {
        let parser = parser!("< untag");
        let sequence = parser.parse();
        assert_eq!(sequence[0], Element::text("< untag"));
    }
}
