#[derive(Debug)]
pub enum Error {
    SerializationError(serde_qs::Error),
    ApiKeyError,
}
