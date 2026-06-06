pub mod ingestion;
pub mod compression;
pub mod ai;
pub mod security;
pub mod automation;

pub use anyhow::Result;

pub fn version() -> &'static str {
    "2.0.0-ULTRA"
}
