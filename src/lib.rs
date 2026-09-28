#![doc = include_str!("../README.md")]

use std::{env, path::PathBuf};

use dirs::cache_dir;
use zenodo_rs::ZenodoClient;

/// Returns the default cache directory used by dataset fetching.
///
/// # Example
/// ```
/// use metabolomics_datasets_rs::default_dataset_cache_dir;
///
/// assert!(default_dataset_cache_dir().ends_with("metabolomics-datasets-rs/datasets"));
/// ```
pub fn default_dataset_cache_dir() -> PathBuf {
    cache_dir().unwrap_or_else(env::temp_dir).join("metabolomics-datasets-rs").join("datasets")
}
