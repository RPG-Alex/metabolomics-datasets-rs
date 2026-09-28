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
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the dataset's upstream source.
    #[must_use]
    pub const fn source(&self) -> &DatasetSource {
        &self.source
    }
}



