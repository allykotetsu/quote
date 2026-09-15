use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub(crate) struct Quote {
    pub(crate) id: i64,
    pub(crate) message: String
}