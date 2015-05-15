use chem::ConcentrationExt;
use rand::{Rng, thread_rng};

struct Neuron {
    weights: Vec<f32>,
}

impl Neuron {
    pub fn new(input_count: usize) -> Neuron {
        Neuron { 
            weights: { 
                let mut vec = Vec::with_capacity(input_count + 1);
                let mut rng = thread_rng();
                for _ in 0 .. input_count + 1 {
                    vec.push(rng.gen::<f32>().clamp(-1.0, 1.0));
                }
                vec
            }
        }
    }
}

struct NeuronLayer {
    neurons: Vec<Neuron>,
}

impl NeuronLayer {
    pub fn new(neuron_count: usize, inputs_per_neuron: usize) -> NeuronLayer {
        NeuronLayer {
            neurons: {
                let mut vec = Vec::with_capacity(neuron_count);
                for _ in 0 .. neuron_count {
                    vec.push(Neuron::new(inputs_per_neuron));
                }
                vec
            }
        }
    }
}

pub struct NeuralNet {
    input_count: usize,
    layers: Vec<NeuronLayer>,
}

impl NeuralNet {
    pub fn new(input_count: usize, output_count: usize, hidden_layer_count: usize, 
               neurons_per_hidden_layer: usize) -> NeuralNet {
        NeuralNet {
            input_count: input_count,
            layers: {
                let mut vec = Vec::with_capacity(hidden_layer_count + 1);
                if hidden_layer_count > 0 {
                    vec.push(NeuronLayer::new(neurons_per_hidden_layer, input_count));
                    for _ in 0 .. hidden_layer_count - 1 {
                        vec.push(NeuronLayer::new(neurons_per_hidden_layer, 
                                                  neurons_per_hidden_layer))
                    }
                    vec.push(NeuronLayer::new(output_count, neurons_per_hidden_layer));
                } else {
                    vec.push(NeuronLayer::new(output_count, input_count));
                }
                vec
            }
        }
    }

    pub fn update(&self, inputs: Vec<f32>) -> Option<Vec<f32>> {
        if inputs.len() != self.input_count { return None }
        Some(self.layers.iter().fold(inputs, |acc, ref layer| {
            layer.neurons.iter().map(|neuron| {
                sigmoid(neuron.weights.iter()
                              .zip(acc.iter().chain([-1.0].iter()))
                              .map(|(w, v)| w * v)
                              .fold(0.0, |acc, ref n| acc + n), 1.0)
            }).collect()
        }))
    }
}

fn sigmoid(a: f32, p: f32) -> f32 {
    1.0 / (1.0 + (-a / p).exp())
}
