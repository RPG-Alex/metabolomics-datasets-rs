use std::path::PathBuf;

use crate::{DatasetSource, content::DatasetContent, license::DatasetLicense};

/// Describes a known external dataset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dataset {
    id: String,
    source: DatasetSource,
    contents: Vec<DatasetContent>,
    license: DatasetLicense,
    citation: Option<String>,
}
impl Dataset {
    /// Creates a dataset description.
    #[must_use]
    pub const fn new(
        id: String, 
        source: DatasetSource,
        contents: Vec<DatasetContent>,
        license: DatasetLicense,
        citation: Option<String>

    ) -> Self {
        Self { id, source, contents, license, citation }
    }

    /// Returns the dataset's name.
    #[must_use]
    pub const fn id(&self) -> &str {
        &self.id
    }

    /// Returns the dataset's upstream source.
    #[must_use]
    pub const fn source(&self) -> &DatasetSource {
        &self.source
    }

    /// Returns the [`DatasetContent`] of this dataset
    #[must_use]
    pub const fn contents(&self) -> &[DatasetContent] {
        &self.contents
    }

    /// Returns the license information.
    #[must_use]
    pub const fn license(&self) -> &DatasetLicense {
        &self.license
    }

    /// Returns citation information if present
    #[must_use]
    pub const fn citation(&self) -> &Option<String> {
        &self.citation
    }

    /// Begins configuring how dataset should be materialized locally
    #[must_use]
    pub fn materialize(&self) -> DatasetMaterializer {
        DatasetMaterializer::new(self)
    }
}


pub struct DatasetMaterializer {
    dataset: Dataset,
    local_dir: Option<PathBuf>,
    extract: bool,
}