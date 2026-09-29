use std::path::{Path, PathBuf};

use crate::Dataset;

/// A dataset materialized locally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDataset {
    dataset: Dataset,
    root: PathBuf,
    artifacts: Vec<DatasetArtifact>,
}

impl LocalDataset {
    /// Returns the dataset's metadata
    #[must_use]
    pub const fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    /// Returns root of locally materialized dataset
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns a specific local artifact of the dataset
    #[must_use]
    pub fn artifacts(&self) -> &[DatasetArtifact] {
        &self.artifacts
    }
}

/// A local artifact belonging to a materialized dataset
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetArtifact {
    path: PathBuf,
}

impl DatasetArtifact {
    /// Creates a new artifact
    #[must_use]
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }
    /// Returns the artifact's path
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}
