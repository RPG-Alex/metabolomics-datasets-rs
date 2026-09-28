use std::borrow::Cow;

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
    pub const fn new(id: &'static str, source: DatasetSource) -> Self {
        Self { id, source }
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

pub struct DatasetLicense {
    status: LicenseStatus,
    expression: Option<LicenseExpression>,
    license_source: Option<String>,
    notes: Vec<String>,
}

impl DatasetLicense {
    #[must_use]
    pub fn with_license_source(
        mut self, 
        source: impl Into<Cow<'static, str>>
    ) -> Self {
        self.license_source = Some(source.into());
        self
    }

    #[must_use]
    pub fn with_notes<I,S>(mut self, notes: I) -> Self where I: IntoIterator<Item = S>, S: Into<Cow<'static, str>> {
        self.notes.extend(notes.into_iter().map(Into::into));
        self
    }
}

pub enum LicenseStatus {
    Known,
    NotSpecified,
    Unknown,
}

pub enum LicenseExpression {
    License(License),
    And(Box<LicenseExpression>, Box<LicenseExpression>),
    Or(Box<LicenseExpression>, Box<LicenseExpression>),
}

impl LicenseExpression {
    #[must_use]
    pub fn and(self, other: impl Into<Self>) -> Self {
        Self::And(Box::new(self), Box::new(other.into()))
    }

    #[must_use]
    pub fn or(self, other: impl Into<Self>) -> Self {
        Self::Or(Box::new(self), Box::new(other.into()))
    }
}

impl From<License> for LicenseExpression {
    fn from(license: License) -> Self {
        Self::License(license)
    }
}

pub enum License {
    Apache2,
    Mit,
    Cc0_1_0,
    CcBy4_0,

    Custom {
        name: String,
        url: Option<String>,
    },
}