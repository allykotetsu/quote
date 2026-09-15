use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct QuoteId {
    pub(crate) id: i64
}