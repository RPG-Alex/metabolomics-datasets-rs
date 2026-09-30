use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use dirs::cache_dir;
use tokio::fs;
use zenodo_rs::{RecordId, ZenodoClient};

use crate::{
    Dataset, DatasetArtifact, DatasetError, DatasetSource, LocalDataset, MaterializeBuilder,
};

pub(crate) async fn materialize_dataset(
    builder: MaterializeBuilder<'_>,
) -> Result<LocalDataset, DatasetError> {
    let MaterializeBuilder { dataset, local_dir } = builder;
    let root = local_dir.unwrap_or_else(default_dataset_cache_dir).join(dataset.id());
    fs::create_dir_all(&root)
        .await
        .map_err(|source| DatasetError::Io { path: root.clone(), source })?;

    let artifacts = match dataset.source() {
        DatasetSource::Zenodo { record_id } => {
            fetch_zenodo_dataset(dataset, *record_id, &root).await?
        }
        DatasetSource::Url { direct_url } => {
            vec![fetch_url_dataset(dataset, direct_url, &root).await?]
        }
    };

    Ok(LocalDataset::new(dataset.clone(), root, artifacts))
}

/// Returns the default directory used for materialized datasets
#[must_use]
pub fn default_dataset_cache_dir() -> PathBuf {
    cache_dir().unwrap_or_else(env::temp_dir).join(env!("CARGO_PKG_NAME")).join("datasets")
}

/// Fetches a dataset from Zenodo
async fn fetch_zenodo_dataset(
    dataset: &Dataset,
    record_id: u64,
    root: &Path,
) -> Result<Vec<DatasetArtifact>, DatasetError> {
    let client = ZenodoClient::anonymous_builder()
        .build()
        .map_err(|source| DatasetError::Zenodo { dataset_id: dataset.id().to_owned(), source })?;

    let record_id = RecordId::from(record_id);

    let files = client
        .list_record_files(record_id)
        .await
        .map_err(|source| DatasetError::Zenodo { dataset_id: dataset.id().to_owned(), source })?;

    let mut artifacts = Vec::with_capacity(files.len());

    for file in files {
        let file_name = Path::new(&file.key).file_name().unwrap_or_else(|| OsStr::new(&file.id));

        let path = root.join(file_name);

        if !fs::try_exists(&path)
            .await
            .map_err(|source| DatasetError::Io { path: path.clone(), source })?
        {
            client.download_record_file_by_key_to_path(record_id, &file.key, &path).await.map_err(
                |source| DatasetError::Zenodo { dataset_id: dataset.id().to_owned(), source },
            )?;
        }
        artifacts.push(DatasetArtifact::new(path));
    }
    Ok(artifacts)
}
