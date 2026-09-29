// use std::{env, path::PathBuf};

// use dirs::cache_dir;
// use zenodo_rs::ZenodoClient;

// #[non_exhaustive]
// #[derive(Debug, Clone)]
// pub struct DatasetFetchOptions {
//     /// Directory used to cache downloaded datasets.
//     pub cache_dir: PathBuf,

//     /// Whether an existing cached dataset should be downloaded again.
//     pub force: bool,
// }

// impl Default for DatasetFetchOptions {
//     fn default() -> Self {
//         Self {
//             cache_dir: default_dataset_cache_dir(),
//             force: false,
//         }
//     }
// }

// /// Returns the default cache directory used by dataset fetching.
// ///
// /// # Example
// /// ```
// /// use metabolomics_datasets_rs::default_dataset_cache_dir;
// ///
// /// assert!(default_dataset_cache_dir().ends_with("metabolomics-datasets-rs/
// datasets")); /// ```
// pub fn default_dataset_cache_dir() -> PathBuf {
//     cache_dir().unwrap_or_else(env::temp_dir).join("metabolomics-datasets-rs"
// ).join("datasets") }

// pub(crate) fn fetch_zenodo_dataset<D>(
//     dataset: &D,
//     options: &DatasetFetchOptions,
// ) -> Result<DatasetArtifact, DatasetError>
// where
//     D: ZenodoDatasetSource + ?Sized {}
