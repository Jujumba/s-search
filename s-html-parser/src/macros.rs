#[macro_export]
macro_rules! on_err {
    ($expr:expr, $err: tt) => {{
        match $expr {
            ::std::result::Result::Ok(a) => a,
            ::std::result::Result::Err(_) => $err,
        }
    }};
}
