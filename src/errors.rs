use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntryError {
    #[error("Bundle SAID is missing")]
    MissingSaid,
}
