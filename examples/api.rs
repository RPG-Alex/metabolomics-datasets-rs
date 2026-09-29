// // Built-in dataset.
// let lotus = datasets::lotus();
//
//
// // Query its scientific metadata.
// assert!(lotus.contains(DatasetContent::Smiles));
// let license = lotus.license();
//
//
// // Download + extract using defaults.
// let local = lotus
//     .materialize()
//     .extract()
//     .build()?;
//
//
// // Download somewhere explicitly.
// let local = lotus
//     .materialize()
//     .extract()
//     .local_dir("./datasets")
//     .build()?;
//
//
// // Discover built-in datasets.
// let smiles = datasets::with_content(DatasetContent::Smiles);
//
// let paired = datasets::with_all([
//     DatasetContent::Smiles,
//     DatasetContent::MsMs,
// ]);
//
//
// // Define a completely downstream dataset.
// let custom = Dataset::new(
//     "my-dataset",
//     DatasetSource::url("https://example.org/data.zip"),
// )
// .with_contents([
//     DatasetContent::Smiles,
//     DatasetContent::MsMs,
// ])
// .with_license(DatasetLicense::from(License::CcBy4_0));
//
//
// // It gets exactly the same materialization API.
// let local = custom
//     .materialize()
//     .extract()
//     .build()?;
//
//
// // Add it alongside our built-ins.
// let mut registry = DatasetRegistry::with_builtins();
// registry.insert(custom)?;
//
// let smiles = registry.with_content(DatasetContent::Smiles);
