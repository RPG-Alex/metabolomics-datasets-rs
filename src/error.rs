use std::{io, path::PathBuf};

use thiserror::Error;
use zenodo_rs::ZenodoError;

/// Errors raised while fetching and materializing datasets
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DatasetError {
    /// A direct dataset download failed
    #[error("failed to download `{dataset_id}` from {url}: {source}")]
    Download {
        /// Identifier of the dataset being downloaded
        dataset_id: String,
        /// Upstream URL
        url: String,
        /// Associated HTTP Error
        #[source]
        source: reqwest::Error,
    },
    /// A Zenodo operation failed
    #[error("failed to fetch dataset `{dataset_id}` from Zenodo: {source}")]
    Zenodo {
        /// Identifier of the dataset being fetched
        dataset_id: String,
        /// Underlying Zenodo error
        #[source]
        source: ZenodoError,
    },
    /// A filesystem operation failed.
    #[error("failed to access dataset path {path}: {source}")]
    Io {
        /// Path involved in the failed operation.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },
    /// A downloaded artifact could not be extracted.
    #[error("failed to extract dataset artifact {path}: {source}")]
    Extraction {
        /// Artifact being extracted.
        path: PathBuf,
        /// Underlying extraction error.
        #[source]
        source: io::Error,
    },
}
