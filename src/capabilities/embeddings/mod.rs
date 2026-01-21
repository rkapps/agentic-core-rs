

#[derive(Debug)]
pub struct Embedding {
    pub text: String,
    pub vector: Vec<f64>,
    pub model: String,
    pub dimension: usize
}

impl Embedding {

    pub fn empty() -> Self {
        Self{
            text: String::new(),
            vector: vec![],
            model: String::new(),
            dimension: 0
        }
    }
}