use anyhow::Result;
use ort::{Environment, SessionBuilder, Value};
use std::sync::Arc;

pub struct RagEngine {
    env: Arc<Environment>,
}

impl RagEngine {
    pub fn new() -> Result<Self> {
        let env = Arc::new(Environment::builder()
            .with_name("QVX_RAG")
            .build()?);
        Ok(Self { env })
    }

    pub fn run_inference(&self, model_path: &str, input_tensor: Vec<f32>, dims: Vec<i64>) -> Result<Vec<f32>> {
        let session = SessionBuilder::new(&self.env)?
            .with_model_from_file(model_path)?;
        
        let input_value = Value::from_array(
            session.allocator(),
            &ndarray::Array::from_shape_vec(
                dims.iter().map(|&d| d as usize).collect::<Vec<_>>(),
                input_tensor
            )?
        )?;

        let outputs = session.run(vec![input_value])?;
        let output_tensor: ndarray::ArrayViewD<f32> = outputs[0].try_extract()?;
        
        Ok(output_tensor.to_owned().into_raw_vec())
    }
}
