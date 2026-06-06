use std::fs::File;
use std::path::Path;
use memmap2::MmapOptions;
use anyhow::{Result, Context};
use log::info;

pub struct BlockStreamer {
    file_path: String,
    block_size: usize,
}

impl BlockStreamer {
    pub fn new<P: AsRef<Path>>(path: P, block_size: Option<usize>) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().into_owned();
        let size = block_size.unwrap_or(512 * 1024 * 1024); // Default 512MB
        Ok(Self {
            file_path: path_str,
            block_size: size,
        })
    }

    pub fn stream_blocks<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(&[u8]) -> Result<()>,
    {
        let file = File::open(&self.file_path)
            .with_context(|| format!("Failed to open file: {}", self.file_path))?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        
        let total_size = mmap.len();
        info!("Streaming file: {} (Total size: {} bytes)", self.file_path, total_size);

        let mut offset = 0;
        while offset < total_size {
            let end = std::cmp::min(offset + self.block_size, total_size);
            let block = &mmap[offset..end];
            
            callback(block)?;
            
            offset = end;
        }

        Ok(())
    }
}
