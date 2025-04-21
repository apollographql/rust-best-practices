use apollo_compiler::Name;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Selection is missing field `{0}` in connect directive")]
    MissingField(Name),
}
