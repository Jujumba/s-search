#[macro_export]
macro_rules! on_err {
    ($expr:expr, $err: tt) => {{
        match $expr {
            ::std::result::Result::Ok(a) => a,
            ::std::result::Result::Err(_) => $err,
        }
    }};
}
#[macro_export]
macro_rules! expect_token {
    ($parser:expr, $pat:pat, $err:expr) => {
        let $pat = $parser.tokenizer.next_token().kind else {
            return $err;
        };
    };
}