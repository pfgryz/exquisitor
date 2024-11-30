use crate::clustering::ALPHABET;
use burn::data::dataloader::batcher::Batcher;
use burn::data::dataloader::Dataset;
use burn::data::dataset::InMemDataset;
use burn::prelude::{Backend, Tensor};
use burn::tensor::TensorData;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Result as IoResult;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SequencesRecord {
    anchor: String,
    positive: String,
    negative: String,
}

#[derive(Clone, Debug)]
pub struct SequencesEncodedRecord<B: Backend> {
    anchor: Tensor<B, 2>,
    positive: Tensor<B, 2>,
    negative: Tensor<B, 2>,
}

pub struct SequencesDataset<B: Backend> {
    items: Vec<SequencesEncodedRecord<B>>,
}

fn one_hot(s: &str, alphabet: &[char]) -> Vec<f32> {
    let mut char_index = HashMap::new();

    for (i, &ch) in alphabet.iter().enumerate() {
        char_index.insert(ch, i);
    }

    let mut encoded = vec![0.0; s.len() * alphabet.len()];

    for (idx, char) in s.chars().enumerate() {
        encoded[idx * alphabet.len() + char_index.get(&char).unwrap()] = 1.0;
    }

    encoded
}

pub fn encode_sequence<B: Backend>(device: &B::Device, s: &str, alphabet: &[char]) -> Tensor<B, 2> {
    Tensor::from_data(TensorData::new(one_hot(s, alphabet), [1, s.len() * alphabet.len()]), device)
}

fn encode_record<B: Backend>(
    device: B::Device,
    record: &SequencesRecord,
) -> SequencesEncodedRecord<B> {
    SequencesEncodedRecord {
        anchor: encode_sequence(&device, &record.anchor, ALPHABET),
        positive: encode_sequence(&device, &record.positive, ALPHABET),
        negative: encode_sequence(&device, &record.negative, ALPHABET),
    }
}

impl<B: Backend> SequencesDataset<B> {
    pub fn new(device: B::Device, path: &str) -> IoResult<Self> {
        let reader = csv::ReaderBuilder::new();
        let dataset = InMemDataset::from_csv(path, &reader).unwrap();

        let items = dataset
            .iter()
            .collect::<Vec<_>>()
            .par_iter()
            .map(|record| encode_record(device.clone(), record))
            .collect();

        Ok(Self { items })
    }
}

impl<B: Backend> Dataset<SequencesEncodedRecord<B>> for SequencesDataset<B> {
    fn get(&self, index: usize) -> Option<SequencesEncodedRecord<B>> {
        self.items.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

#[derive(Clone, Debug)]
pub struct SequencesBatcher;

#[derive(Clone, Debug)]
pub struct SequencesBatch<B: Backend> {
    pub(crate) anchors: Tensor<B, 2>,
    pub(crate) positive: Tensor<B, 2>,
    pub(crate) negative: Tensor<B, 2>,
}

impl SequencesBatcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl<B: Backend> Batcher<SequencesEncodedRecord<B>, SequencesBatch<B>> for SequencesBatcher {
    fn batch(&self, items: Vec<SequencesEncodedRecord<B>>) -> SequencesBatch<B> {
        let anchors = items.iter().map(|item| item.anchor.to_owned()).collect();
        let positive = items.iter().map(|item| item.positive.to_owned()).collect();
        let negative = items.iter().map(|item| item.negative.to_owned()).collect();

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
