//! Module containing implementation of dataset and batcher of artificial neural network

use crate::clustering::ALPHABET;
use burn::data::dataloader::batcher::Batcher;
use burn::data::dataloader::Dataset;
use burn::data::dataset::SqliteDataset;
use burn::prelude::{Backend, Tensor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Result as IoResult;

/// Data record for training sample
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SequencesRecord {
    /// Anchor sequences
    anchor: String,

    /// Positive case; sequences similar to anchor
    positive: String,

    /// Negative case; sequences dissimilar to anchor
    negative: String,
}

/// Dataset of sequences stored in Sqlite
pub struct SequencesDataset {
    dataset: SqliteDataset<SequencesRecord>,
}

/// Encoded the sequences using one-hot encoding with given alphabet
fn one_hot(s: &str, alphabet: &[char]) -> Vec<f32> {
    let mut char_index = HashMap::new();

    for (i, &ch) in alphabet.iter().enumerate() {
        char_index.insert(ch, i);
    }

    let mut encoded = vec![0.0; s.len() * alphabet.len()];

    for (idx, char) in s.chars().enumerate() {
        let position: usize = match char_index.get(&char) {
            None => 0,
            Some(p) => *p,
        };

        encoded[idx * alphabet.len() + position] = 1.0;
    }

    encoded
}

/// Encodes the sequence using one-hot and converts to tensor
pub fn encode_sequence<B: Backend>(device: &B::Device, s: &str, alphabet: &[char]) -> Tensor<B, 2> {
    Tensor::<B, 1>::from_data(one_hot(s, alphabet).as_slice(), device).unsqueeze_dim::<2>(0)
}
impl SequencesDataset {
    pub fn new(path: &str) -> IoResult<Self> {
        let dataset = SqliteDataset::from_db_file(path, "data").unwrap();

        Ok(Self { dataset })
    }
}

impl Dataset<SequencesRecord> for SequencesDataset {
    fn get(&self, index: usize) -> Option<SequencesRecord> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}

/// Sequences batcher
#[derive(Clone, Debug)]
pub struct SequencesBatcher<B: Backend> {
    device: B::Device,
}

/// Batch for training
#[derive(Clone, Debug)]
pub struct SequencesBatch<B: Backend> {
    pub(crate) anchors: Tensor<B, 2>,
    pub(crate) positive: Tensor<B, 2>,
    pub(crate) negative: Tensor<B, 2>,
}

impl<B: Backend> SequencesBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

impl<B: Backend> Batcher<SequencesRecord, SequencesBatch<B>> for SequencesBatcher<B> {
    fn batch(&self, items: Vec<SequencesRecord>) -> SequencesBatch<B> {
        let anchors = items
            .iter()
            .map(|item| encode_sequence(&self.device, &item.anchor, ALPHABET))
            .collect();
        let positive = items
            .iter()
            .map(|item| encode_sequence(&self.device, &item.positive, ALPHABET))
            .collect();
        let negative = items
            .iter()
            .map(|item| encode_sequence(&self.device, &item.negative, ALPHABET))
            .collect();

        let anchors = Tensor::cat(anchors, 0);
        let positive = Tensor::cat(positive, 0);
        let negative = Tensor::cat(negative, 0);

        SequencesBatch {
            anchors,
            positive,
            negative,
        }
    }
}
