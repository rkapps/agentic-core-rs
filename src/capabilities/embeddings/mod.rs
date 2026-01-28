use anyhow::Error;


#[derive(Debug, Clone)]
pub struct Embedding(Vec<f32>);

impl Embedding {
    pub fn new(vector: Vec<f32>) -> Self {
        Self(vector)
    }
    
    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
    
    pub fn into_vec(self) -> Vec<f32> {
        self.0
    }
    
    pub fn dimension(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub struct BatchResult {
    pub successful: Vec<(usize, Embedding)>,
    pub failed: Vec<(usize, Error)>,
}