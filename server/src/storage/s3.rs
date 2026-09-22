//! S3 remote files.


use std::sync::Arc;

use async_trait::async_trait;
use futures::TryStreamExt as _;
use object_store::ObjectStoreExt;
use object_store::aws::{AmazonS3Builder, AmazonS3};
use object_store::path::Path;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWriteExt};
use tokio_util::io::StreamReader;

use super::{Download, RemoteFile, StorageBackend};
use crate::error::{ErrorKind, ServerError, ServerResult};

type Client = Arc<AmazonS3>;

/// The S3 remote file storage backend.
#[derive(Debug)]
pub struct S3Backend {
    client: Client,
    config: S3StorageConfig,
}

/// S3 remote file storage configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct S3StorageConfig {
    /// The AWS region.
    region: String,

    /// The name of the bucket.
    bucket: String,

    /// Custom S3 endpoint.
    ///
    /// Set this if you are using an S3-compatible object storage (e.g., Minio).
    endpoint: Option<String>,

    /// S3 credentials.
    ///
    /// If not specified, it's read from the `AWS_ACCESS_KEY_ID` and
    /// `AWS_SECRET_ACCESS_KEY` environment variables.
    credentials: Option<S3CredentialsConfig>,
}

/// S3 credential configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct S3CredentialsConfig {
    /// Access key ID.
    access_key_id: String,

    /// Secret access key.
    secret_access_key: String,
}

/// Reference to a file in an S3-compatible storage bucket.
///
/// We store the region and bucket to facilitate migration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S3RemoteFile {
    /// Name of the S3 region.
    pub region: String,

    /// Name of the bucket.
    pub bucket: String,

    /// Key of the file.
    pub key: String,
}

impl S3Backend {
    fn config_builder(config: &S3StorageConfig) -> AmazonS3Builder {
        let mut builder = AmazonS3Builder::from_env()
            // We allow HTTP, because using a self-hosted object storage is a common setup.
            .with_allow_http(true)
            .with_region(&config.region)
            .with_bucket_name(&config.bucket);

        if let Some(endpoint) = &config.endpoint {
            builder = builder.with_endpoint(endpoint);
        }

        if let Some(credentials) = &config.credentials {
            builder = builder.with_access_key_id(&credentials.access_key_id)
                    .with_secret_access_key(&credentials.secret_access_key);
        }

        builder
    }

    pub async fn new(config: S3StorageConfig) -> ServerResult<Self> {
        let client = Arc::new(Self::config_builder(&config).build()?);

        Ok(Self {
            client,
            config,
        })
    }

    async fn get_client_from_db_ref<'a>(
        &self,
        file: &'a RemoteFile,
    ) -> ServerResult<(Client, &'a S3RemoteFile)> {
        let file = if let RemoteFile::S3(file) = file {
            file
        } else {
            return Err(ErrorKind::StorageError(anyhow::anyhow!(
                "Does not understand the remote file reference"
            ))
            .into());
        };

        let client = if self.config.region == file.region {
            self.client.clone()
        } else {
            // TODO: Cache the client instance
            Arc::new(Self::config_builder(&self.config)
                .with_region(&file.region)
                .build()
                .map_err(ServerError::storage_error)?)
        };

        Ok((client, file))
    }
}

#[async_trait]
impl StorageBackend for S3Backend {
    async fn upload_file(
        &self,
        name: String,
        mut stream: &mut (dyn AsyncRead + Unpin + Send),
    ) -> ServerResult<RemoteFile> {
        let mut writer = object_store::buffered::BufWriter::new(self.client.clone(), name.clone().into());

        tokio::io::copy(&mut stream, &mut writer).await.map_err(ServerError::storage_error)?;
        writer.shutdown().await.map_err(ServerError::storage_error)?;

        Ok(RemoteFile::S3(S3RemoteFile {
            region: self.config.region.clone(),
            bucket: self.config.bucket.clone(),
            key: name,
        }))
    }

    async fn delete_file(&self, _name: String) -> ServerResult<()> {
        // let deletion = self
        //     .client
        //     .delete_object()
        //     .bucket(&self.config.bucket)
        //     .key(&name)
        //     .send()
        //     .await
        //     .map_err(ServerError::storage_error)?;

        // tracing::debug!("delete_file -> {:#?}", deletion);

        // Ok(())
        todo!()
    }

    async fn delete_file_db(&self, _file: &RemoteFile) -> ServerResult<()> {
        // let (client, file) = self.get_client_from_db_ref(file).await?;

        // let deletion = client
        //     .delete_object()
        //     .bucket(&file.bucket)
        //     .key(&file.key)
        //     .send()
        //     .await
        //     .map_err(ServerError::storage_error)?;

        // tracing::debug!("delete_file -> {:#?}", deletion);

        // Ok(())
        todo!()
    }

    // TODO Pass references instead
    async fn download_file(&self, name: String, _prefer_stream: bool) -> ServerResult<Download> {

        // TODO: prefer_stream

        let payload = self.client.get(&Path::from(name)).await.map_err(ServerError::storage_error)?.payload;

        let stream = match payload {
            object_store::GetResultPayload::Stream(stream) => stream,
        };

        // AsyncReader can only emit std::io::Error.
        let io_stream = stream.map_err(std::io::Error::other);

        let reader: Box<dyn AsyncRead + Send + Unpin> = Box::new(StreamReader::new(io_stream));
        Ok(Download::AsyncRead(reader))
    }

    async fn download_file_db(
        &self,
        _file: &RemoteFile,
        _prefer_stream: bool,
    ) -> ServerResult<Download> {
        // let (client, file) = self.get_client_from_db_ref(file).await?;

        // let req = client.get_object().bucket(&file.bucket).key(&file.key);

        // self.get_download(req, prefer_stream).await
        todo!()
    }

    async fn make_db_reference(&self, name: String) -> ServerResult<RemoteFile> {
        Ok(RemoteFile::S3(S3RemoteFile {
            region: self.config.region.clone(),
            bucket: self.config.bucket.clone(),
            key: name,
        }))
    }
}
