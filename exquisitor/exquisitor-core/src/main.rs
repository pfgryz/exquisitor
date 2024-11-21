use crate::neural::model::ModelConfig;
use crate::neural::training::{train, TrainingConfig};
use burn::backend::{Autodiff, Wgpu};
use burn::optim::AdamConfig;

pub mod neural;

fn main() {
    type LearnBackend = Wgpu<f32, i32>;
    type LearnAutodiffBackend = Autodiff<LearnBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    let artifact_dir = "model0";
    train::<LearnAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(), AdamConfig::new(), 44),
        device.clone(),
    );
}
