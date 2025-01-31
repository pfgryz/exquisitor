//! Module with train function for artificial neural network

use crate::clustering::ALPHABET;
use crate::neural::data::{SequencesBatcher, SequencesDataset};
use crate::neural::model::ModelConfig;
use burn::config::Config;
use burn::data::dataloader::DataLoaderBuilder;
use burn::lr_scheduler::exponential::ExponentialLrSchedulerConfig;
use burn::module::Module;
use burn::optim::AdamWConfig;
use burn::record::{BinGzFileRecorder, CompactRecorder, FullPrecisionSettings};
use burn::tensor::backend::AutodiffBackend;
use burn::train::metric::{CpuMemory, CpuUse, LearningRateMetric, LossMetric};
use burn::train::LearnerBuilder;
use std::env;

#[derive(Config)]
pub struct TrainingConfig {
    /// Model
    pub model: ModelConfig,

    /// Number of epochs
    #[config(defualt = 100)]
    pub num_epochs: usize,

    /// Optimizer
    pub optimizer: AdamWConfig,

    /// Learning rate for optimizer,
    #[config(default = 1e-5)]
    pub learning_rate: f64,

    /// Gamma parameter for Exponential LR
    #[config(default = 0.9999)]
    pub gamma: f64,

    /// Sequence length
    #[config(default = 200)]
    pub sequence_length: usize,

    /// Dropout
    #[config(default = 0.2)]
    pub dropout: f64,

    /// Size of the batch
    #[config(default = 32)]
    pub batch_size: usize,

    #[config(default = 4)]
    /// Number of workers
    pub num_workers: usize,

    /// Seed for the backend
    pub seed: u64,
}

/// Trains the given model using given device and stores it's artifacts in given directory
pub fn train<B: AutodiffBackend>(artifact_dir: &str, config: TrainingConfig, device: B::Device) {
    // Create directory for artifacts
    if std::fs::exists(artifact_dir).unwrap() {
        std::fs::remove_dir_all(artifact_dir).unwrap();
    }
    std::fs::create_dir_all(artifact_dir).unwrap();

    // Seed for random generator
    B::seed(config.seed);

    // Batching
    let batcher_train = SequencesBatcher::<B>::new(device.clone());
    let batcher_validate = SequencesBatcher::<B::InnerBackend>::new(device.clone());

    match env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => eprintln!("Error retrieving current directory: {}", e),
    }

    // Dataloaders
    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .build(SequencesDataset::new("data/training.db").unwrap());

    let dataloader_validate = DataLoaderBuilder::new(batcher_validate)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .build(SequencesDataset::new("data/validation.db").unwrap());

    // Learning
    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(LossMetric::new()) // add metric for
        .metric_valid_numeric(LossMetric::new())
        .metric_train(LossMetric::new()) // add metric for
        .metric_valid(LossMetric::new())
        .metric_train(CpuUse::new())
        .metric_valid(CpuUse::new())
        .metric_train(CpuMemory::new())
        .metric_valid(CpuMemory::new())
        .metric_train_numeric(LearningRateMetric::new())
        .metric_train(LearningRateMetric::new())
        .num_epochs(config.num_epochs)
        .devices(vec![device.clone()])
        .with_file_checkpointer(CompactRecorder::new())
        .summary()
        .build(
            config.model.init::<B>(
                &device,
                config.sequence_length * ALPHABET.len(),
                config.dropout,
            ),
            config.optimizer.init(),
            ExponentialLrSchedulerConfig::new(config.learning_rate, config.gamma).init(),
        );

    // Train
    let trained_model = learner.fit(dataloader_train, dataloader_validate);

    // Save
    config
        .save(format!("{artifact_dir}/config.json"))
        .expect("Training config should be saved!");
    trained_model
        .save_file(
            format!("{artifact_dir}/model.bin"),
            &BinGzFileRecorder::<FullPrecisionSettings>::new(),
        )
        .expect("Trained model should be saved!");
}
