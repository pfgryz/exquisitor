use std::fmt::Debug;
use tch::{nn, Device, Kind, Tensor};
use tch::nn::{Module, OptimizerConfig};

#[derive(Debug)]
struct Model {
    fc1: nn::Linear,
    fc2: nn::Linear,
}

impl Model {
    fn new(vs: &nn::Path) -> Self {
        let fc1 = nn::linear(vs / "fc1", 1, 32, Default::default());
        let fc2 = nn::linear(vs / "fc2", 32, 1, Default::default());
        Self { fc1, fc2 }
    }
}

impl nn::Module for Model {
    fn forward(&self, xs: &Tensor) -> Tensor {
        xs.apply(&self.fc1).relu().apply(&self.fc2)
    }
}

fn main() {
    let device = Device::cuda_if_available();

    let vs = nn::VarStore::new(device);
    let net = Model::new(&vs.root());

    let inputs = Tensor::arange_start_step(1., 11., 1., (Kind::Float, device)).view([10, 1]);
    let targets = (&inputs).to_kind(Kind::Float);


    let mut opt = nn::Adam::default().build(&vs, 1e-3).unwrap();

    for epoch in 1..20000 {
        let predictions = net.forward(&inputs);
        let loss = predictions.mse_loss(&targets, tch::Reduction::Mean);

        opt.backward_step(&loss);

        if epoch % 100 == 0 {
            println!("Epoch: {}, Loss: {:.4}|", epoch, &loss);
        }
    }

    let test_input = Tensor::from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).view([9, 1]).to(device).to_kind(Kind::Float);
    let output = net.forward(&test_input);
    println!("Test input: {}, Prediction: {}", test_input, output);
}