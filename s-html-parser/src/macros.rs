#[macro_export]
macro_rules! on_err {
    ($expr:expr, $err: tt) => {{
        match $expr {
            ::std::option::Option::Some(a) => a,
            ::std::option::Option::None => $err,
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