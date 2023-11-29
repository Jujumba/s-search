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
    use crate::token::{Token, Tokenizer};
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

        assert_eq!(tokenizer.next_token(), Token::text("Hello"));
        assert_eq!(tokenizer.next_token(), Token::LAngle);
        assert_eq!(tokenizer.next_token(), Token::text("world"));
    }
    #[test]
    fn tokenizer_exclamation_in_text() {
        let tokenizer = tokenizer!("This is a text...");

        assert_eq!(tokenizer.next_token(), Token::text("This is a text..."))
    }
    #[test] 
    fn tokenizer_trims_ends() {
        let tokenizer = tokenizer!("    A wizard is never late   !  <tag>   ");

        assert_eq!(tokenizer.next_token(), Token::text("A wizard is never late   !"));
        assert_eq!(tokenizer.next_token(), Token::LAngle);
        assert_eq!(tokenizer.next_token(), Token::text("tag"));
        assert_eq!(tokenizer.next_token(), Token::RAngle);
        assert_eq!(tokenizer.next_token(), Token::Eof);
    }
    #[test]
    fn parser_basic() {
        let parser = parser!("<p>Text   !");
        let sequence = parser.parse();

        assert_eq!(sequence[0], Element::tag("p", HashMap::new()));
        assert_eq!(sequence[1], Element::text("Text   !"));
    }
    #[test]
    fn parser_unclosed_tag() {
        let parser = parser!("<pText   !");
        let sequence = parser.parse();
        assert!(sequence.is_empty());
    }
}
