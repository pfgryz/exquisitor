use crate::neural::data::SequencesBatch;
use crate::neural::loss::{ContrastiveLoss, ContrastiveLossConfig};
use burn::nn::conv::{Conv1d, Conv1dConfig};
use burn::nn::{BatchNorm, BatchNormConfig, Gelu, Linear, LinearConfig};
use burn::prelude::{Backend, Config, Module, Tensor};
use burn::tensor::backend::AutodiffBackend;
use burn::train::{RegressionOutput, TrainOutput, TrainStep, ValidStep};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv1: Conv1dBlock<B>,
    conv2: Conv1dBlock<B>,
    fc1: Linear<B>,
    fc2: Linear<B>,
    activation: Gelu,
    loss: ContrastiveLoss,
}

impl<B: Backend> Model<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let [batch_size, sequence_length] = input.dims();

        let x = input.reshape([batch_size, 1, sequence_length]).detach();
        let x = self.conv1.forward(x);
        let x = self.conv2.forward(x);

        let [batch_size, channels, sequence_length] = x.dims();
        let x = x.reshape([batch_size, channels * sequence_length]).detach();

        let x = self.fc1.forward(x);
        let x = self.activation.forward(x);

        let x = self.fc2.forward(x);

        x
    }

    pub fn forward_contrastive(
        &self,
        anchors: Tensor<B, 2>,
        positive: Tensor<B, 2>,
        negative: Tensor<B, 2>,
    ) -> RegressionOutput<B> {
        let anchors_embed = self.forward(anchors);
        let positive_embed = self.forward(positive);
        let negative_embed = self.forward(negative);

        let loss = self.loss.forward(
            anchors_embed.clone(),
            positive_embed.clone(),
            negative_embed.clone(),
        );

        RegressionOutput::new(loss, anchors_embed, positive_embed)
    }
}

impl<B: AutodiffBackend> TrainStep<SequencesBatch<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: SequencesBatch<B>) -> TrainOutput<RegressionOutput<B>> {
        let item = self.forward_contrastive(batch.anchors, batch.positive, batch.negative);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<SequencesBatch<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: SequencesBatch<B>) -> RegressionOutput<B> {
        self.forward_contrastive(batch.anchors, batch.positive, batch.negative)
    }
}

#[derive(Config, Debug)]
pub struct ModelConfig {}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device, input_size: usize) -> Model<B> {
        Model {
            conv1: Conv1dBlock::new(1, 4, 1, 1, device),
            conv2: Conv1dBlock::new(4, 16, 1, 1, device),
            fc1: LinearConfig::new(256, 128).init(device),
            fc2: LinearConfig::new(128, 64).init(device),
            loss: ContrastiveLossConfig::new().init::<B>(device, 1.0, 0.0),
            activation: Default::default(),
        }
    }
}

#[derive(Module, Debug)]
pub struct Conv1dBlock<B: Backend> {
    conv: Conv1d<B>,
    norm: BatchNorm<B, 1>,
    activation: Gelu,
}

impl<B: Backend> Conv1dBlock<B> {
    pub fn new(
        channels_in: usize,
        channels_out: usize,
        kernel_size: usize,
        dilation: usize,
        device: &B::Device,
    ) -> Self {
        Self {
            conv: Conv1dConfig::new(channels_in, channels_out, kernel_size)
                .with_dilation(dilation)
                .init(device),
            norm: BatchNormConfig::new(channels_out).init(device),
            activation: Gelu::new(),
        }
    }

    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 3> {
        let x = self.conv.forward(input);
        let x = self.norm.forward(x);

        self.activation.forward(x)
    }
}
