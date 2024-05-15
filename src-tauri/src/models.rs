#[derive(thiserror::Error, Debug)]
pub enum GeorgError {
    #[error("Unknown error")]
    Unknown
}