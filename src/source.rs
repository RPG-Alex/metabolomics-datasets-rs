/// Describes an upstream source from which a dataset can be obtained.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DatasetSource {
    /// A dataset published as a Zenodo record.
    Zenodo {
        /// The Zenodo record identifier.
        record_id: u64,
    },

    /// A directly downloadable dataset.
    Url {
        /// URL for directly downloading the dataset.
        direct_url: String,
    },
}

impl DatasetSource {
    /// Creates a Zenodo dataset source.
    #[must_use]
    pub const fn zenodo(record_id: u64) -> Self {
        Self::Zenodo { record_id }
    }

    /// Creates a direct URL dataset source.
    #[must_use]
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url { direct_url: url.into() }
    }

    /// Returns teh zenodo record id if present
    #[must_use]
    pub const fn zenodo_record_id(&self) -> Option<u64> {
        match self {
            Self::Zenodo { record_id } => Some(*record_id),
            Self::Url { .. } => None,
        }
    }

    /// Returns the direct download URL if present
    #[must_use]
    pub fn direct_url(&self) -> Option<&str> {
        match self {
            Self::Url { direct_url } => Some(direct_url),
            Self::Zenodo { .. } => None,
        }
    }
}
