use std::path::PathBuf;

use crate::{
    DatasetError, DatasetSource, LocalDataset, content::DatasetContent, fetch::materialize_dataset,
    license::DatasetLicense,
};

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
    /// Begins building a dataset description.
    #[must_use]
    pub fn builder(id: impl Into<String>, source: DatasetSource) -> DatasetBuilder {
        DatasetBuilder::new(id.into(), source)
    }

    /// Returns the dataset's stable identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the dataset's upstream source.
    #[must_use]
    pub const fn source(&self) -> &DatasetSource {
        &self.source
    }

    /// Returns the scientific content provided by the dataset.
    #[must_use]
    pub fn contents(&self) -> &[DatasetContent] {
        &self.contents
    }

    /// Returns the dataset's license information.
    #[must_use]
    pub const fn license(&self) -> &DatasetLicense {
        &self.license
    }

    /// Returns the dataset citation, if one is available.
    #[must_use]
    pub fn citation(&self) -> Option<&str> {
        self.citation.as_deref()
    }

    /// Begins configuring how this dataset should be materialized locally.
    #[must_use]
    pub const fn materialize(&self) -> MaterializeBuilder<'_> {
        MaterializeBuilder::new(self)
    }
}

/// Builds a [`Dataset`] description.
#[derive(Debug)]
pub struct DatasetBuilder {
    dataset: Dataset,
}

impl DatasetBuilder {
    fn new(id: String, source: DatasetSource) -> Self {
        Self {
            dataset: Dataset {
                id,
                source,
                contents: Vec::new(),
                license: DatasetLicense::unknown(),
                citation: None,
            },
        }
    }

    /// Adds scientific content classifications to the dataset.
    #[must_use]
    pub fn contents(mut self, contents: impl IntoIterator<Item = DatasetContent>) -> Self {
        self.dataset.contents.extend(contents);
        self
    }

    /// Sets the dataset's licensing information.
    #[must_use]
    pub fn license(mut self, license: DatasetLicense) -> Self {
        self.dataset.license = license;
        self
    }

    /// Sets the preferred citation for the dataset.
    #[must_use]
    pub fn citation(mut self, citation: impl Into<String>) -> Self {
        self.dataset.citation = Some(citation.into());
        self
    }

    /// Finishes building the dataset description.
    #[must_use]
    pub fn build(self) -> Dataset {
        self.dataset
    }
}

/// Configures how a [`Dataset`] should be materialized locally.
#[derive(Debug)]
pub struct MaterializeBuilder<'a> {
    pub(crate) dataset: &'a Dataset,
    pub(crate) local_dir: Option<PathBuf>,
}

impl<'a> MaterializeBuilder<'a> {
    const fn new(dataset: &'a Dataset) -> Self {
        Self { dataset, local_dir: None }
    }

    /// Sets the directory in which the dataset should be materialized.
    #[must_use]
    pub fn local_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.local_dir = Some(path.into());
        self
    }

    /// Materializes the dataset locally
    ///
    /// # Errors
    ///
    /// Returns [`DatasetError`] if problem downloading or processing downloaded
    /// dataset
    pub async fn build(self) -> Result<LocalDataset, DatasetError> {
        materialize_dataset(self).await
    }
}
