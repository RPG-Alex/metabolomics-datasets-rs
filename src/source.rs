
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
        /// URL from which the dataset can be downloaded.
        url: String,
    },
}

