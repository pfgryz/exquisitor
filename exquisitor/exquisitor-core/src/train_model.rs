use burn::backend::wgpu::WgpuDevice;
use burn::backend::{Autodiff, Wgpu};
use burn::optim::AdamConfig;
use exquisitor_core::neural::model::ModelConfig;
use exquisitor_core::neural::training::{train, TrainingConfig};

type LearnBackend = Wgpu<f32, i32>;

fn get_device() -> WgpuDevice {
    WgpuDevice::default()
}

fn main() {
    type LearnAutodiffBackend = Autodiff<LearnBackend>;

    let device = get_device();
    let artifact_dir = "models/model_3";
    train::<LearnAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(), AdamConfig::new(), 44)
            .with_sequence_length(150)
            .with_num_workers(1)
            .with_batch_size(256)
            .with_learning_rate(1e-5)
            .with_dropout(0.5),
        device.clone(),
    );
}
