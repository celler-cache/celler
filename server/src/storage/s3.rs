//! S3 remote files.

use std::time::Duration;

use async_trait::async_trait;
use futures::TryStreamExt as _;
use object_store::ObjectStoreExt;
use object_store::aws::{AmazonS3Builder, AmazonS3};
use bytes::BytesMut;
use futures::future::join_all;
use object_store::path::Path;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;

use super::{Download, RemoteFile, StorageBackend};
use crate::error::{ErrorKind, ServerError, ServerResult};
use attic::io::read_chunk_async;
use attic::util::Finally;

/// The chunk size for each part in a multipart upload.
const CHUNK_SIZE: usize = 8 * 1024 * 1024;

type Client = AmazonS3;

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
        Ok(Self {
            client: Self::config_builder(&config).build()?,
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
            Self::config_builder(&self.config)
                .with_region(&file.region)
                .build()
                .map_err(ServerError::storage_error)?
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
        // let buf = BytesMut::with_capacity(CHUNK_SIZE);
        // let first_chunk = read_chunk_async(&mut stream, buf)
        //     .await
        //     .map_err(ServerError::storage_error)?;

        // if first_chunk.len() < CHUNK_SIZE {
        //     // do a normal PutObject
        //     let put_object = self
        //         .client
        //         .put_object()
        //         .bucket(&self.config.bucket)
        //         .key(&name)
        //         .body(first_chunk.into())
        //         .send()
        //         .await
        //         .map_err(ServerError::storage_error)?;

        //     tracing::debug!("put_object -> {:#?}", put_object);

        //     return Ok(RemoteFile::S3(S3RemoteFile {
        //         region: self.config.region.clone(),
        //         bucket: self.config.bucket.clone(),
        //         key: name,
        //     }));
        // }

        // let multipart = self
        //     .client
        //     .create_multipart_upload()
        //     .bucket(&self.config.bucket)
        //     .key(&name)
        //     .send()
        //     .await
        //     .map_err(ServerError::storage_error)?;

        // let upload_id = multipart.upload_id().unwrap();

        // let cleanup = Finally::new({
        //     let bucket = self.config.bucket.clone();
        //     let client = self.client.clone();
        //     let upload_id = upload_id.to_owned();
        //     let name = name.clone();

        //     async move {
        //         tracing::warn!("Upload was interrupted - Aborting multipart upload");

        //         let r = client
        //             .abort_multipart_upload()
        //             .bucket(bucket)
        //             .key(name)
        //             .upload_id(upload_id)
        //             .send()
        //             .await;

        //         if let Err(e) = r {
        //             tracing::warn!("Failed to abort multipart upload: {}", e);
        //         }
        //     }
        // });

        // let mut part_number = 1;
        // let mut parts = Vec::new();
        // let mut first_chunk = Some(first_chunk);

        // loop {
        //     let chunk = if part_number == 1 {
        //         first_chunk.take().unwrap()
        //     } else {
        //         let buf = BytesMut::with_capacity(CHUNK_SIZE);
        //         read_chunk_async(&mut stream, buf)
        //             .await
        //             .map_err(ServerError::storage_error)?
        //     };

        //     if chunk.is_empty() {
        //         break;
        //     }

        //     let client = self.client.clone();
        //     let fut = tokio::task::spawn({
        //         client
        //             .upload_part()
        //             .bucket(&self.config.bucket)
        //             .key(&name)
        //             .upload_id(upload_id)
        //             .part_number(part_number)
        //             .body(chunk.clone().into())
        //             .send()
        //     });

        //     parts.push(fut);
        //     part_number += 1;
        // }

        // #[allow(clippy::result_large_err)]
        // let completed_parts = join_all(parts)
        //     .await
        //     .into_iter()
        //     .map(|join_result| join_result.unwrap())
        //     .collect::<std::result::Result<Vec<_>, _>>()
        //     .map_err(ServerError::storage_error)?
        //     .into_iter()
        //     .enumerate()
        //     .map(|(idx, part)| {
        //         let part_number = idx + 1;
        //         CompletedPart::builder()
        //             .set_e_tag(part.e_tag().map(str::to_string))
        //             .set_part_number(Some(part_number as i32))
        //             .build()
        //     })
        //     .collect::<Vec<_>>();

        // let completed_multipart_upload = CompletedMultipartUpload::builder()
        //     .set_parts(Some(completed_parts))
        //     .build();

        // let completion = self
        //     .client
        //     .complete_multipart_upload()
        //     .bucket(&self.config.bucket)
        //     .key(&name)
        //     .upload_id(upload_id)
        //     .multipart_upload(completed_multipart_upload)
        //     .send()
        //     .await
        //     .map_err(ServerError::storage_error)?;

        // tracing::debug!("complete_multipart_upload -> {:#?}", completion);

        // cleanup.cancel();

        // Ok(RemoteFile::S3(S3RemoteFile {
        //     region: self.config.region.clone(),
        //     bucket: self.config.bucket.clone(),
        //     key: name,
        // }))
        todo!()
    }

    async fn delete_file(&self, name: String) -> ServerResult<()> {
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

    async fn delete_file_db(&self, file: &RemoteFile) -> ServerResult<()> {
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
    async fn download_file(&self, name: String, prefer_stream: bool) -> ServerResult<Download> {

        // TODO: prefer_stream

        let payload = self.client.get(&Path::from(name)).await.map_err(ServerError::storage_error)?.payload;

        let stream = match payload {
            object_store::GetResultPayload::Stream(stream) => stream,
            _ => unreachable!(),
        };

        // AsyncReader can only emit std::io::Error.
        let io_stream = stream.map_err(std::io::Error::other);

        let reader: Box<dyn AsyncRead + Send + Unpin> = Box::new(StreamReader::new(io_stream));
        Ok(Download::AsyncRead(reader))
    }

    async fn download_file_db(
        &self,
        file: &RemoteFile,
        prefer_stream: bool,
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
