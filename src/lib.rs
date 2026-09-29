#![doc = include_str!("../README.md")]

mod fetch;
mod source;
mod dataset;
mod datasets;
mod license;
mod local;
mod content;

pub use dataset::{
    Dataset,
    DatasetBuilder,
    MaterializeBuilder,
};
pub use license::{
    DatasetLicense,
    License,
    LicenseExpression,
    LicenseStatus
};

pub use source::DatasetSource;

