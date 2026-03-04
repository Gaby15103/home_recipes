use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ParserTokenResponse {
    pub raw_text: String,
    pub suggested_id: Option<i32>,
    pub canonical_name: String,
    pub category: String,
    /// Strategy used: "alias_match", "fuzzy_search", or "none"
    pub match_strategy: String,
    /// Score from 0.0 to 1.0. UI can use this for color coding.
    pub confidence_score: f32,
    pub is_new_term: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCorrectionInput {
    pub raw_text: String,
    pub selected_lexicon_id: i32,
    pub was_fuzzy_match_correct: bool,
    pub manual_edit_occurred: bool,
}