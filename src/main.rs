mod data;
mod model;
mod training;

use burn::backend::{
    wgpu::{Wgpu, WgpuDevice},
    Autodiff,
};

fn main() {
    let device = WgpuDevice::default();
    training::run::<Autodiff<Wgpu>>(device);
}
