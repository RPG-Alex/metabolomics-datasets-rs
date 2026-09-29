use std::{env, fs, path::PathBuf};

use dirs::cache_dir;
use zenodo_rs::ZenodoClient;

use crate::{DatasetError, DatasetSource, LocalDataset, MaterializeBuilder};

pub(crate) async fn materialize_dataset(builder: MaterializeBuilder<'_>) -> Result<LocalDataset, DatasetError> {
    let MaterializeBuilder { dataset, local_dir} = builder;
    let root = local_dir.unwrap_or_else(default_dataset_cache_dir).join(dataset.id());
    fs::create_dir_all(&root).await.map_err(|source| DatasetError::Io { path: root.clone(), source })?;

    let artifacts = match dataset.source() {
        DatasetSource::Zenodo { record_id } => {
            fetch_zenodo_dataset(dataset, *record_id, &root).await?
        },
        DatasetSource::Url { direct_url } => {
            vec![fetch_url_dataset(dataset, direct_url, &root).await?]
        },
    };

    Ok(LocalDataset::new(
        dataset.clone(),
        root,
        artifacts,
    ))
}