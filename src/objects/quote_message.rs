use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct QuoteMessage {
    pub(crate) message: String
}