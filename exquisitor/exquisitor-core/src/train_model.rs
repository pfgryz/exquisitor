use burn::backend::wgpu::WgpuDevice;
use burn::backend::{Autodiff, Wgpu};
use burn::optim::AdamWConfig;
use exquisitor_core::neural::model::ModelConfig;
use exquisitor_core::neural::training::{train, TrainingConfig};

type LearnBackend = Wgpu<f32, i32>;

fn get_device() -> WgpuDevice {
    WgpuDevice::default()
}

fn main() {
    type LearnAutodiffBackend = Autodiff<LearnBackend>;

    let device = get_device();
    train::<LearnAutodiffBackend>(
        format!("models/model_final").as_str(),
        TrainingConfig::new(
            ModelConfig::new(),
            AdamWConfig::new(),
            44,
        )
        .with_sequence_length(150)
        .with_num_workers(1)
        .with_batch_size(256)
        .with_learning_rate(1e-6)
        .with_dropout(0.4)
        .with_num_epochs(3),
        device.clone(),
    );
}
