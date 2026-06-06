use anyhow::Result;

pub struct Quantizer {
    bit_width: u8,
}

impl Quantizer {
    pub fn new(bit_width: u8) -> Self {
        Self { bit_width }
    }

    /// BitNet 1.58b inspired 1-bit/2-bit quantization
    pub fn quantize_vector(&self, vector: &[f32]) -> Result<Vec<i8>> {
        let mut quantized = Vec::with_capacity(vector.len());
        
        // Simple 1.58b-style quantization: map to {-1, 0, 1}
        let mean = vector.iter().sum::<f32>() / vector.len() as f32;
        let abs_mean = vector.iter().map(|x| (x - mean).abs()).sum::<f32>() / vector.len() as f32;
        let scale = 1.0 / (abs_mean + 1e-8);

        for &val in vector {
            let scaled = (val - mean) * scale;
            let q = if scaled > 0.5 {
                1
            } else if scaled < -0.5 {
                -1
            } else {
                0
            };
            quantized.push(q);
        }

        Ok(quantized)
    }

    pub fn dequantize_vector(&self, quantized: &[i8], mean: f32, scale: f32) -> Vec<f32> {
        quantized.iter().map(|&q| (q as f32 / scale) + mean).collect()
    }
}
