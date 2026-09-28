/// Describes an upstream source from which a dataset can be obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DatasetSource {
    /// A dataset published as a Zenodo record.
    Zenodo {
        /// The Zenodo record identifier.
        record_id: u64,
    },
}

/// Describes a known external dataset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dataset {
    /// Human-readable dataset name.
    name: &'static str,

    /// Upstream source for the dataset.
    source: DatasetSource,
}

impl Dataset {
    /// Creates a dataset description.
    #[must_use]
    pub const fn new(name: &'static str, source: DatasetSource) -> Self {
        Self { name, source }
    }

    /// Returns the dataset's name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the dataset's upstream source.
    #[must_use]
    pub const fn source(&self) -> &DatasetSource {
        &self.source
    }
}