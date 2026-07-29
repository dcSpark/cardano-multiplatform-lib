/// Defines the way to parse the object from a UTF-8 string.
///
/// This is like the standard `FromStr` trait, except that it imposes
/// additional bounds on the error type to make it more usable for
/// aggregation to higher level errors and passing between threads.
pub trait FromStr: Sized {
    type Error: core::error::Error + Send + Sync + 'static;

    fn from_str(s: &str) -> Result<Self, Self::Error>;
}

impl<T> FromStr for T
where
    T: core::str::FromStr,
    <T as core::str::FromStr>::Err: core::error::Error + Send + Sync + 'static,
{
    type Error = <T as core::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        core::str::FromStr::from_str(s)
    }
}
