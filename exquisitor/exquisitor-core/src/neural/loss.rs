use burn::prelude::{Backend, Config, Module, Tensor};

#[derive(Config, Debug)]
pub struct ContrastiveLossConfig;

impl ContrastiveLossConfig {
    pub fn init<B: Backend>(
        &self,
        device: &B::Device,
        margin_positive: f64,
        margin_negative: f64,
    ) -> ContrastiveLoss {
        ContrastiveLoss {
            margin_positive,
            margin_negative,
        }
    }
}

#[derive(Module, Clone, Debug)]
pub struct ContrastiveLoss {
    /// Margin for positive cases (similar)
    pub margin_positive: f64,

    /// Margin for negative cases (dissimilar)
    pub margin_negative: f64,
}

impl ContrastiveLoss {
    pub fn forward<B: Backend>(
        &self,
        anchors: Tensor<B, 2>,
        positives: Tensor<B, 2>,
        negatives: Tensor<B, 2>,
    ) -> Tensor<B, 1> {
        let magnitude = |tensor: Tensor<B, 2>| {
            tensor
                .clone()
                .powi_scalar(2)
                .sum_dim(0)
                .transpose()
                .squeeze::<1>(1)
                .sqrt()
        };

        let cosine = |tensor: Tensor<B, 2>, other: Tensor<B, 2>| {
            tensor
                .clone()
                .mul(other.clone())
                .sum_dim(0)
                .transpose()
                .squeeze(1)
                .div(magnitude(tensor.clone()))
                .div(magnitude(other.clone()))
        };

        let pos: Tensor<B, 1> = cosine(anchors.clone(), positives);
        let neg: Tensor<B, 1> = cosine(anchors, negatives);

        let loss =
            (pos.ones_like() * self.margin_positive - pos.clone()).max_pair(pos.zeros_like()) +
                (neg.clone() - neg.ones_like() * self.margin_negative).max_pair(neg.zeros_like());

        loss.mean()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::Wgpu;
    use burn::prelude::TensorData;

    #[test]
    fn test_contrastive_loss() {
        type TestBackend = Wgpu;
        let device = &Default::default();
        let loss = ContrastiveLossConfig::new().init::<TestBackend>(device, 1.0, 0.0f64);

        let anchor = Tensor::<TestBackend, 2>::from_data(
            TensorData::from([[1.0, 0.0], [0.0, 1.0]]),
            &device,
        );
        let positive = Tensor::<TestBackend, 2>::from_data(
            TensorData::from([[0.5, 0.5], [0.5, 0.5]]),
            &device,
        );
        let negative = Tensor::<TestBackend, 2>::from_data(
            TensorData::from([[-1.0, 0.0], [0.0, 1.0]]),
            &device,
        );

        let l = loss.forward(anchor, positive, negative);

        assert!(false);
    }
}
