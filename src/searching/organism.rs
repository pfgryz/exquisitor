pub struct OrganismMatch {
    sequence_id: String,
    name: String,
    confidence_score: f32,
}

impl OrganismMatch {
    pub fn new(sequence_id: String, name: String, confidence_score: f32) -> Self {
        Self {
            sequence_id,
            name,
            confidence_score
        }
    }
}