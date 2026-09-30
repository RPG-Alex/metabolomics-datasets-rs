#![doc = include_str!("../README.md")]

mod content;
mod dataset;
mod datasets;
mod error;
mod fetch;
mod license;
mod local;
mod source;

pub use dataset::{Dataset, DatasetBuilder, MaterializeBuilder};
pub use error::DatasetError;
pub use license::{DatasetLicense, License, LicenseExpression, LicenseStatus};
pub use local::{DatasetArtifact, LocalDataset};
pub use source::DatasetSource;
