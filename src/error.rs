#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("'{0}' is not in alphabet")]
    InvalidChar(char),

    #[error("The string is not aligned with 4")]
    UnalignedStr,
}
