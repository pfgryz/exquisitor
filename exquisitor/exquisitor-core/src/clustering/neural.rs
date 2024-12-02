use crate::clustering::ALPHABET;
use crate::io::sequence::{Alignment, Sequence};
use crate::neural::data::encode_sequence;
use crate::neural::model::Model;
use crate::neural::training::TrainingConfig;
use burn::config::Config;
use burn::module::Module;
use burn::prelude::Backend;
use burn::record::{BinGzFileRecorder, CompactRecorder, FullPrecisionSettings, Recorder};
use burn::tensor::Tensor;
use std::io::Error as IoError;
use std::io::Result as IoResult;

pub struct NeuralEmbedder<B: Backend> {
    model: Model<B>,
    sequence_length: usize,
}

impl<B: Backend> NeuralEmbedder<B> {
    pub fn new(artifact_dir: &str, device: B::Device) -> IoResult<Self> {
        let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
            .map_err(|e| IoError::new(std::io::ErrorKind::Other, format!("Cannot load config: {e}")))?;

        let record = BinGzFileRecorder::<FullPrecisionSettings>::new()
            .load(format!("{artifact_dir}/model.bin").into(), &device)
            .map_err(|e| IoError::new(std::io::ErrorKind::Other, format!("Cannot load model: {e}")))?;

        let model = config
            .model
            .init::<B>(&device, config.sequence_length, config.dropout)
            .load_record(record);

        Ok(Self {
            model,
            sequence_length: config.sequence_length,
        })
    }

    pub fn embed(&self, device: B::Device, sequences: &Vec<Sequence>) -> Tensor<B, 2> {
        let encoded = sequences
            .clone()
            .iter_mut()
            .map(|s| s.pad(self.sequence_length, '-'.into(), Alignment::Center))
            .map(|s| s.truncate(self.sequence_length, Alignment::Center))
            .map(|s| encode_sequence::<B>(&device, s.content().as_ref(), ALPHABET))
            .map(|tensor| tensor.unsqueeze_dim::<2>(0))
            .collect::<Vec<_>>();

        let batch = Tensor::cat(encoded, 0);
        self.model.forward(batch)
    }
}
