pub struct OrganismMatch {
    sequence_id: usize,
    name: String,
    confidence_score: f32,
}

impl OrganismMatch {
    pub fn new(sequence_id: usize, name: String, confidence_score: f32) -> Self {
        Self {
            sequence_id,
            name,
            confidence_score,
        }
    }

    pub fn sequence_id(&self) -> usize {
        self.sequence_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn confidence_score(&self) -> f32{
        self.confidence_score
    }
}

pub struct OrganismFound {
    name: String,
    quality: f32,
}

impl OrganismFound {
    pub fn new(name: String, quality: f32) -> Self {
        Self {
            name,
            quality,
        }
    }
}