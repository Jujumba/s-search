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
    fn tokenizer_basic() {
        let tokenizer = tokenizer!("Hello <world");

        assert_eq!(tokenizer.next_token().kind, TokenKind::text("Hello"));
        assert_eq!(tokenizer.next_token().kind, TokenKind::LAngle);
        assert_eq!(tokenizer.next_token().kind, TokenKind::text("world"));
    }
    #[test]
    fn parser_ignores_comment() {
        let parser = parser!("<p>Text <!-- Hello -->  !");
        let sequence = parser.parse();

        assert_eq!(sequence[0], Element::tag("p", HashMap::new()));
        assert_eq!(sequence[1], Element::text("Text"));
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
        assert_eq!(sequence[0], Element::text("<"));
        assert_eq!(sequence[1], Element::text("untag"));
    }
}
