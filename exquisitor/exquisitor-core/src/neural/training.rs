use crate::clustering::ALPHABET;
use crate::neural::data::{SequencesBatcher, SequencesDataset};
use crate::neural::model::ModelConfig;
use burn::config::Config;
use burn::data::dataloader::DataLoaderBuilder;
use burn::module::Module;
use burn::optim::AdamConfig;
use burn::record::{BinGzFileRecorder, CompactRecorder, FullPrecisionSettings};
use burn::tensor::backend::AutodiffBackend;
use burn::train::metric::LossMetric;
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
    pub optimizer: AdamConfig,

    /// Learning rate for optimizer,
    #[config(default = 1e-5)]
    pub learning_rate: f64,

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
        .num_workers(config.num_workers)
        .build(SequencesDataset::new("data/training.csv").unwrap());

    let dataloader_validate = DataLoaderBuilder::new(batcher_validate)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(SequencesDataset::new("data/validation.csv").unwrap());

    // Learning
    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(LossMetric::new()) // add metric for
        .metric_valid_numeric(LossMetric::new())
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
            config.learning_rate,
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
