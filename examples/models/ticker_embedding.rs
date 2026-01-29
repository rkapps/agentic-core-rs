use serde::{Deserialize, Serialize};
use storage_core::core::RepoModel;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerEmbedding {
    pub id: String,
    pub symbol: String,
    pub sentiment_id: String,
    pub embedding_text: String,
    pub vector: Vec<f32>
   
}



impl RepoModel<String> for TickerEmbedding {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn collection(&self) -> &'static str {
        "ticker_embedding"
    }
}
