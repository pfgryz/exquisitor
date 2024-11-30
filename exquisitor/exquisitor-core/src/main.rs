use burn::backend::{Autodiff, Wgpu};
use burn::optim::AdamConfig;
use exquisitor_core::neural::model::ModelConfig;
use exquisitor_core::neural::training::{train, TrainingConfig};

fn main() {
    type LearnBackend = Wgpu<f32, i32>;
    type LearnAutodiffBackend = Autodiff<LearnBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    let artifact_dir = "model1";
    train::<LearnAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(), AdamConfig::new(), 44).with_sequence_length(4),
        device.clone(),
    );
}
