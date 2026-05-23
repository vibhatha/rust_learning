use patterns::{Bias, ConvolutionLayer, Node};

fn main() {
    let node = Node {
        node_id: 1,
        data: 42.0_f32,
    };

    let layer_without_bias = ConvolutionLayer::new(vec![node]);

    let node = Node {
        node_id: 2,
        data: 10.0_f32,
    };

    let mut layer_with_bias = ConvolutionLayer::new(vec![node]);
    layer_with_bias.set_bias(Bias {
        layer_id: 0,
        value: 0.1_f32,
    });

    println!("{layer_without_bias}");
    println!("{layer_with_bias}");
}
