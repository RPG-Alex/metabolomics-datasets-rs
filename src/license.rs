use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetLicense {
    status: LicenseStatus,
    expression: Option<LicenseExpression>,
    license_source: Option<String>,
    notes: Vec<String>,
}

impl DatasetLicense {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            status: LicenseStatus::Unknown,
            expression: None,
            license_source: None,
            notes: Vec::new(),
        }
    }
    #[must_use]
    pub fn with_license_source(mut self, source: String) -> Self {
        self.license_source = Some(source);
        self
    }

    #[must_use]
    pub fn with_notes(mut self, notes: Vec<String>) -> Self {
        self.notes.extend(notes);
        self
    }

    #[must_use]
    pub const fn unknown() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseStatus {
    Known,
    NotSpecified,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum License {
    Apache2,
    Mit,
    Cc0_1_0,
    CcBy4_0,

    Custom { name: String, url: Option<String> },
}

impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            License::Apache2 => todo!(),
            License::Mit => todo!(),
            License::Cc0_1_0 => todo!(),
            License::CcBy4_0 => todo!(),
            License::Custom { name, url } => todo!(),
        }
    }
}
