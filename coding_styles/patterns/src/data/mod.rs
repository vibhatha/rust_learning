use std::fmt::{Display, Formatter, Result};

mod sealed {
    pub trait Sealed {}

    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Numeric types allowed in nodes, biases, and convolution layers.
pub trait Numeric: sealed::Sealed + Copy + Display {}

impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

pub struct Node<T: Numeric> {
    pub node_id: u8,
    pub data: T,
}

pub struct Bias<T: Numeric> {
    pub layer_id: u8,
    pub value: T,
}

pub struct ConvolutionLayer<T: Numeric> {
    pub nodes: Vec<Node<T>>,
    pub biases: Option<Bias<T>>,
}

impl<T: Numeric> ConvolutionLayer<T> {
    pub fn new(nodes: Vec<Node<T>>) -> Self {
        Self { nodes, biases: None }
    }

    pub fn set_bias(&mut self, bias: Bias<T>) {
        self.biases = Some(bias);
    }
}

impl<T: Numeric> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Node {{ node_id: {}, data: {} }}",
            self.node_id, self.data
        )
    }
}

impl<T: Numeric> Display for Bias<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Bias {{ layer_id: {}, value: {} }}",
            self.layer_id, self.value
        )
    }
}

impl<T: Numeric> Display for ConvolutionLayer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ConvolutionLayer {{ nodes: [")?;
        for (index, node) in self.nodes.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{node}")?;
        }
        match &self.biases {
            Some(bias) => write!(f, "], biases: {bias} }}"),
            None => write!(f, "], biases: none }}"),
        }
    }
}
