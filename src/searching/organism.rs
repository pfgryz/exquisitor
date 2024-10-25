use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct OrganismMatch {
    sequence_id: usize,
    name: String,
    confidence_score: f64,
}

impl OrganismMatch {
    pub fn new(sequence_id: usize, name: String, confidence_score: f64) -> Self {
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

    pub fn confidence_score(&self) -> f64 {
        self.confidence_score
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct OrganismFound {
    name: String,
    quality: f64,
}

impl OrganismFound {
    pub fn new(name: String, quality: f64) -> Self {
        Self {
            name,
            quality,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn quality(&self) -> f64 {
        self.quality
    }

    pub fn to_tuple(self) -> (String, f64) {
        (self.name, self.quality)
    }
}

pub fn save_found_organisms(buffer: &mut dyn Write, organisms: &Vec<OrganismFound>) -> std::io::Result<()> {
    let json = serde_json::to_string(&organisms)?;
    buffer.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_found_organisms(buffer: &mut dyn Read) -> std::io::Result<Vec<OrganismFound>> {
    let mut data = String::new();
    println!("Data");
    buffer.read_to_string(&mut data)?;
    println!("Read");
    let vec: Vec<OrganismFound> = serde_json::from_str(&data).unwrap();
    println!("Vec");
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use super::*;

    #[test]
    fn test_save_organisms() {
        let organisms = vec![
            OrganismFound::new("A".into(), 2f64),
            OrganismFound::new("B".into(), 1.45f64),
        ];
        let mut buffer = Vec::new();

        let result = save_found_organisms(&mut buffer, &organisms);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).expect("Failed to convert buffer to string");
        assert_eq!(output, "[{\"name\":\"A\",\"quality\":2.0},{\"name\":\"B\",\"quality\":1.45}]");
    }

    #[test]
    fn test_load_organisms() {
        let json = r#"[{"name":"A","quality":2.0},{"name":"B","quality":1.45}]"#;
        let mut reader = json.as_bytes();

        let result = load_found_organisms(&mut reader);
        assert!(result.is_ok());

        let organisms = result.unwrap();
        assert_eq!(organisms.len(), 2);
        assert_eq!(organisms[1].name(), "B");
        assert_approx_eq!(f64, organisms[1].quality(), 1.45);
    }
}