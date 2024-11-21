use burn::data::dataloader::batcher::Batcher;
use burn::data::dataloader::Dataset;
use burn::data::dataset::InMemDataset;
use burn::prelude::{Backend, Tensor};
use exquisitor_core::clustering::ALPHABET;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Result as IoResult;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SequencesRecord {
    anchor: String,
    positive: String,
    negative: String,
}

pub struct SequencesDataset {
    dataset: InMemDataset<SequencesRecord>,
}

impl SequencesDataset {
    pub fn new(path: &str) -> IoResult<Self> {
        let mut reader = csv::ReaderBuilder::new();
        let dataset = InMemDataset::from_csv(path, &reader).unwrap();

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

#[derive(Clone, Debug)]
pub struct SequencesBatcher<B: Backend> {
    device: B::Device,
}

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

fn one_hot_encode<B: Backend>(device: &B::Device, s: &str, alphabet: &[char]) -> Tensor<B, 1> {
    let mut char_index = HashMap::new();

    for (i, &ch) in alphabet.iter().enumerate() {
        char_index.insert(ch, i);
    }

    let encoded = s
        .chars()
        .map(|c| Tensor::<B, 1>::one_hot(*char_index.get(&c).unwrap(), alphabet.len(), &device))
        .map(|tensor| tensor.reshape([1, ALPHABET.len()]))
        .collect::<Vec<_>>();

    Tensor::cat(encoded, 0).flatten(0, 1)
}

impl<B: Backend> Batcher<SequencesRecord, SequencesBatch<B>> for SequencesBatcher<B> {
    fn batch(&self, items: Vec<SequencesRecord>) -> SequencesBatch<B> {
        let anchors = items
            .iter()
            .map(|item| one_hot_encode::<B>(&self.device, &item.anchor, ALPHABET))
            .map(|tensor| tensor.unsqueeze_dim::<2>(0))
            .collect();

        let positive = items
            .iter()
            .map(|item| one_hot_encode::<B>(&self.device, &item.positive, ALPHABET))
            .map(|tensor| tensor.unsqueeze_dim::<2>(0))
            .collect();

        let negative = items
            .iter()
            .map(|item| one_hot_encode::<B>(&self.device, &item.negative, ALPHABET))
            .map(|tensor| tensor.unsqueeze_dim::<2>(0))
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
